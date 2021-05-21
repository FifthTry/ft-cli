#[derive(Debug)]
pub struct Node {
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn readme(&self) -> Option<String> {
        if !self.is_dir {
            return None;
        }
        self.children
            .iter()
            .filter(|c| !c.is_dir)
            .find(|c| {
                let p = std::path::PathBuf::from(&self.path).join("readme");
                c.path
                    .to_lowercase()
                    .starts_with(&p.to_string_lossy().to_string())
            })
            .map(|x| x.path.to_string())
    }
}

pub fn root_tree(root_dir: &std::path::Path) -> crate::Result<Node> {
    fn traverse_tree(root_dir: &std::path::Path) -> crate::Result<Vec<Node>> {
        let mut children = vec![];

        for entry in std::fs::read_dir(root_dir)? {
            let p = entry?.path();
            if p.is_dir() {
                children.push(Node {
                    is_dir: true,
                    path: p.to_string_lossy().to_string(),
                    children: traverse_tree(&p)?,
                });
            } else {
                children.push(Node {
                    is_dir: false,
                    path: p.to_string_lossy().to_string(),
                    children: vec![],
                });
            }
        }
        Ok(children)
    }

    let root = Node {
        is_dir: true,
        path: root_dir.to_string_lossy().to_string(),
        children: traverse_tree(root_dir)?,
    };
    Ok(root)
}

pub fn collection_toc(node: &Node, root_dir: &str, collection_id: &str) -> String {
    fn tree_to_toc_util(
        node: &Node,
        level: usize,
        toc_string: &mut String,
        root_dir: &str,
        collection_id: &str,
    ) {
        for x in node.children.iter() {
            let x_path = std::path::Path::new(&x.path)
                .strip_prefix(root_dir)
                .unwrap_or_else(|_| {
                    panic!(
                        "path `{}` is not starts with root_dir `{}`",
                        &x.path, root_dir
                    )
                });
            let mut path = std::path::PathBuf::from(collection_id).join(&x_path);
            let file_name = path
                .clone()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            if let Some(readme) = x.readme() {
                let x_path = std::path::Path::new(&readme)
                    .strip_prefix(root_dir)
                    .unwrap_or_else(|_| {
                        panic!(
                            "path `{}` is not starts with root_dir `{}`",
                            &readme, root_dir
                        )
                    });
                path = std::path::PathBuf::from(collection_id).join(&x_path);
            }

            toc_string.push_str(&format!(
                "{: >width$}- {path}\n",
                "",
                width = level,
                path = path.to_string_lossy()
            ));
            if x.is_dir {
                toc_string.push_str(&format!(
                    "{: >width$}`{path}/`\n",
                    "",
                    width = level + 2,
                    path = file_name
                ));
            } else {
                toc_string.push_str(&format!(
                    "{: >width$}`{path}`\n",
                    "",
                    width = level + 2,
                    path = file_name
                ));
            }

            if x.is_dir {
                tree_to_toc_util(&x, level + 2, toc_string, root_dir, collection_id);
            }
        }
    }

    let mut toc = "-- toc:\n\n".to_string();
    tree_to_toc_util(node, 0, &mut toc, root_dir, collection_id);
    toc
}

pub fn to_markdown(node: &Node, root_dir: &str, collection_id: &str) -> String {
    fn tree_to_toc_util(
        node: &Node,
        level: usize,
        markdown: &mut String,
        root_dir: &str,
        collection_id: &str,
    ) {
        for x in node.children.iter() {
            let x_path = std::path::Path::new(&x.path)
                .strip_prefix(root_dir)
                .unwrap_or_else(|_| {
                    panic!(
                        "path `{}` is not starts with root_dir `{}`",
                        &x.path, root_dir
                    )
                });
            let path = std::path::PathBuf::from(collection_id).join(&x_path);
            let file_name = path.file_name().unwrap().to_string_lossy();
            markdown.push_str(&format!(
                "{: >width$}- [`{file_name}`]({path})\n",
                "",
                width = level,
                file_name = file_name,
                path = path.to_string_lossy()
            ));
            if x.is_dir {
                tree_to_toc_util(&x, level + 2, markdown, root_dir, collection_id);
            }
        }
    }
    let mut markdown = "-- markdown:\n\n".to_string();
    tree_to_toc_util(node, 0, &mut markdown, root_dir, collection_id);
    markdown
}

pub fn dir_till_path(node: &Node, path: &str) -> Vec<String> {
    fn dir_till_path_util(node: &Node, path: &str, dirs: &mut Vec<String>) -> bool {
        if node.path.eq(path) {
            return true;
        }

        for node in node.children.iter() {
            if node.is_dir && dir_till_path_util(&node, path, dirs) {
                dirs.push(node.path.to_string());
                return true;
            }
            if node.path.eq(path) {
                return true;
            }
        }
        false
    }

    let mut dirs = vec![];
    dir_till_path_util(node, path, &mut dirs);
    dirs.reverse();
    dirs
}

#[cfg(test)]
mod tests {
    use super::Node;
    fn test_node() -> super::Node {
        Node {
            is_dir: true,
            path: "docs".to_string(),
            children: vec![Node {
                is_dir: true,
                path: "docs/a".to_string(),
                children: vec![Node {
                    is_dir: true,
                    path: "docs/a/b".to_string(),
                    children: vec![Node {
                        is_dir: true,
                        path: "docs/a/b/c".to_string(),
                        children: vec![
                            Node {
                                is_dir: true,
                                path: "docs/a/b/c/d".to_string(),
                                children: vec![Node {
                                    is_dir: true,
                                    path: "docs/a/b/c/d/e".to_string(),
                                    children: vec![Node {
                                        is_dir: false,
                                        path: "docs/a/b/c/d/e/f.txt".to_string(),
                                        children: vec![],
                                    }],
                                }],
                            },
                            Node {
                                is_dir: false,
                                path: "docs/a/b/c/readme.md".to_string(),
                                children: vec![],
                            },
                        ],
                    }],
                }],
            }],
        }
    }

    #[test]
    fn collection_toc_test() {
        let node = test_node();
        assert_eq!(
            super::collection_toc(&node, "docs", "testuser/index"),
            r#"-- toc:

- testuser/index/a
  `a/`
  - testuser/index/a/b
    `b/`
    - testuser/index/a/b/c/readme.md
      `c/`
      - testuser/index/a/b/c/d
        `d/`
        - testuser/index/a/b/c/d/e
          `e/`
          - testuser/index/a/b/c/d/e/f.txt
            `f.txt`
      - testuser/index/a/b/c/readme.md
        `readme.md`
"#
            .to_string()
        )
    }

    #[test]
    fn to_markdown() {
        let node = test_node();
        assert_eq!(
            super::to_markdown(&node, "docs", "testuser/index"),
            r#"-- markdown:

- [`a`](testuser/index/a)
  - [`b`](testuser/index/a/b)
    - [`c`](testuser/index/a/b/c)
      - [`d`](testuser/index/a/b/c/d)
        - [`e`](testuser/index/a/b/c/d/e)
          - [`f.txt`](testuser/index/a/b/c/d/e/f.txt)
      - [`readme.md`](testuser/index/a/b/c/readme.md)
"#
        )
    }

    #[test]
    fn till_dir() {
        let expected_output = vec![
            "docs/a".to_string(),
            "docs/a/b".to_string(),
            "docs/a/b/c".to_string(),
            "docs/a/b/c/d".to_string(),
            "docs/a/b/c/d/e".to_string(),
        ];

        let output = super::dir_till_path(&test_node(), "docs/a/b/c/d/e/f.txt");
        assert_eq!(expected_output, output);
    }
}
