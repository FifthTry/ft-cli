#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
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

    pub fn readme_exists(&self) -> bool {
        self.readme().is_some()
    }

    pub fn document_id(&self, root: &str, collection: &str) -> std::path::PathBuf {
        crate::id::to_document_id(&self.path, root, collection)
    }

    pub fn to_markdown(&self, root: &str, collection: &str) -> String {
        self::to_markdown(self, root, collection)
    }

    pub fn collection_toc(&self, root: &str, collection: &str) -> String {
        self::collection_toc(self, root, collection)
    }

    pub fn to_ftd_toc(&self, root: &str, collection: &str) -> ftd::ToC {
        self::to_ftd_toc(self, root, collection)
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
        children.sort();

        Ok(children)
    }

    let root = Node {
        is_dir: true,
        path: root_dir.to_string_lossy().to_string(),
        children: traverse_tree(root_dir)?,
    };
    Ok(root)
}

pub fn to_ftd_toc(node: &Node, root_dir: &str, collection_id: &str) -> ftd::toc::ToC {
    fn to_toc_items(node: &Node, root_dir: &str, collection_id: &str) -> Vec<ftd::toc::TocItem> {
        node.children
            .iter()
            .map(|c| {
                let mut path = c.document_id(root_dir, collection_id);
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                let title = if c.is_dir {
                    format!("`{}/`", file_name)
                } else {
                    format!("`{}`", file_name)
                };

                if let Some(readme) = c.readme() {
                    path = crate::id::to_document_id(&readme, root_dir, collection_id);
                }

                let mut item =
                    ftd::toc::TocItem::with_title_and_id(&title, &path.to_string_lossy());
                item.children = to_toc_items(c, root_dir, collection_id);
                item
            })
            .collect()
    }
    ftd::ToC {
        items: to_toc_items(node, root_dir, collection_id),
    }
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
            let mut path = x.document_id(root_dir, collection_id);

            let file_name = path
                .clone()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            if let Some(readme) = x.readme() {
                path = crate::id::to_document_id(&readme, root_dir, collection_id);
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
    fn tree_to_markdown_util(
        node: &Node,
        level: usize,
        markdown: &mut String,
        root_dir: &str,
        collection_id: &str,
    ) {
        for x in node.children.iter() {
            let mut path = x.document_id(root_dir, collection_id);
            let file_name = path
                .clone()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            if x.is_dir {
                if let Some(readme) = x.readme() {
                    path = crate::id::to_document_id(&readme, root_dir, collection_id);
                }
                markdown.push_str(&format!(
                    "{: >width$}- [`{file_name}/`](/{path})\n",
                    "",
                    width = level,
                    file_name = file_name,
                    path = path.to_string_lossy()
                ));
            } else {
                markdown.push_str(&format!(
                    "{: >width$}- [`{file_name}`](/{path})\n",
                    "",
                    width = level,
                    file_name = file_name,
                    path = path.to_string_lossy()
                ));
            }
            if x.is_dir {
                tree_to_markdown_util(&x, level + 2, markdown, root_dir, collection_id);
            }
        }
    }
    let mut markdown = "-- markdown:\n\n".to_string();
    tree_to_markdown_util(node, 0, &mut markdown, root_dir, collection_id);
    markdown
}

pub fn ancestors<'a>(node: &'a Node, path: &str) -> Vec<&'a Node> {
    fn dir_till_path_util<'a>(node: &'a Node, path: &str, dirs: &mut Vec<&'a Node>) -> bool {
        if node.path.eq(path) {
            return true;
        }

        for node in node.children.iter() {
            if node.is_dir && dir_till_path_util(&node, path, dirs) {
                dirs.push(node);
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
                                path: "docs/a/b/c/README.md".to_string(),
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
    - testuser/index/a/b/c/README.md
      `c/`
      - testuser/index/a/b/c/d
        `d/`
        - testuser/index/a/b/c/d/e
          `e/`
          - testuser/index/a/b/c/d/e/f.txt
            `f.txt`
      - testuser/index/a/b/c/README.md
        `README.md`
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

- [`a/`](/testuser/index/a)
  - [`b/`](/testuser/index/a/b)
    - [`c/`](/testuser/index/a/b/c/README.md)
      - [`d/`](/testuser/index/a/b/c/d)
        - [`e/`](/testuser/index/a/b/c/d/e)
          - [`f.txt`](/testuser/index/a/b/c/d/e/f.txt)
      - [`README.md`](/testuser/index/a/b/c/README.md)
"#
        )
    }

    #[test]
    fn till_dir() {
        let tree = vec![
            super::Node {
                is_dir: true,
                path: "docs/a".to_string(),
                children: vec![super::Node {
                    is_dir: true,
                    path: "docs/a/b".to_string(),
                    children: vec![super::Node {
                        is_dir: true,
                        path: "docs/a/b/c".to_string(),
                        children: vec![
                            super::Node {
                                is_dir: true,
                                path: "docs/a/b/c/d".to_string(),
                                children: vec![super::Node {
                                    is_dir: true,
                                    path: "docs/a/b/c/d/e".to_string(),
                                    children: vec![super::Node {
                                        is_dir: false,
                                        path: "docs/a/b/c/d/e/f.txt".to_string(),
                                        children: vec![],
                                    }],
                                }],
                            },
                            super::Node {
                                is_dir: false,
                                path: "docs/a/b/c/README.md".to_string(),
                                children: vec![],
                            },
                        ],
                    }],
                }],
            },
            super::Node {
                is_dir: true,
                path: "docs/a/b".to_string(),
                children: vec![super::Node {
                    is_dir: true,
                    path: "docs/a/b/c".to_string(),
                    children: vec![
                        super::Node {
                            is_dir: true,
                            path: "docs/a/b/c/d".to_string(),
                            children: vec![super::Node {
                                is_dir: true,
                                path: "docs/a/b/c/d/e".to_string(),
                                children: vec![super::Node {
                                    is_dir: false,
                                    path: "docs/a/b/c/d/e/f.txt".to_string(),
                                    children: vec![],
                                }],
                            }],
                        },
                        super::Node {
                            is_dir: false,
                            path: "docs/a/b/c/README.md".to_string(),
                            children: vec![],
                        },
                    ],
                }],
            },
            super::Node {
                is_dir: true,
                path: "docs/a/b/c".to_string(),
                children: vec![
                    super::Node {
                        is_dir: true,
                        path: "docs/a/b/c/d".to_string(),
                        children: vec![super::Node {
                            is_dir: true,
                            path: "docs/a/b/c/d/e".to_string(),
                            children: vec![super::Node {
                                is_dir: false,
                                path: "docs/a/b/c/d/e/f.txt".to_string(),
                                children: vec![],
                            }],
                        }],
                    },
                    super::Node {
                        is_dir: false,
                        path: "docs/a/b/c/README.md".to_string(),
                        children: vec![],
                    },
                ],
            },
            super::Node {
                is_dir: true,
                path: "docs/a/b/c/d".to_string(),
                children: vec![super::Node {
                    is_dir: true,
                    path: "docs/a/b/c/d/e".to_string(),
                    children: vec![super::Node {
                        is_dir: false,
                        path: "docs/a/b/c/d/e/f.txt".to_string(),
                        children: vec![],
                    }],
                }],
            },
            super::Node {
                is_dir: true,
                path: "docs/a/b/c/d/e".to_string(),
                children: vec![super::Node {
                    is_dir: false,
                    path: "docs/a/b/c/d/e/f.txt".to_string(),
                    children: vec![],
                }],
            },
        ];

        let test_tree = test_node();
        let output = super::ancestors(&test_tree, "docs/a/b/c/d/e/f.txt");
        assert_eq!(tree.iter().collect::<Vec<_>>(), output);
    }
}
