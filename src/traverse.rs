pub struct Node {
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<Node>,
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

pub fn collection_toc(node: &Node) -> String {
    // Incomplete, Need to do small change
    fn tree_to_toc_util(node: &Node, level: usize, toc_string: &mut String) {
        // toc_string.push_str(&format!(
        //     "{: >width$}- {path}\n",
        //     "",
        //     width = level,
        //     path = &node.path
        // ));
        for x in node.children.iter() {
            toc_string.push_str(&format!(
                "{: >width$}- {path}\n",
                "",
                width = level + 2,
                path = &x.path
            ));
            if x.is_dir {
                tree_to_toc_util(&x, level + 2, toc_string);
            }
        }
    }

    let mut toc = String::new();
    tree_to_toc_util(node, 0, &mut toc);
    toc
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
    dirs
}

pub fn main() {
    fn test_node() -> Node {
        Node {
            is_dir: true,
            path: "/Users/abrar/Documents/github/ft-sync/docs".to_string(),
            children: vec![Node {
                is_dir: true,
                path: "/Users/abrar/Documents/github/ft-sync/docs/a".to_string(),
                children: vec![Node {
                    is_dir: true,
                    path: "/Users/abrar/Documents/github/ft-sync/docs/a/b".to_string(),
                    children: vec![Node {
                        is_dir: true,
                        path: "/Users/abrar/Documents/github/ft-sync/docs/a/b/c".to_string(),
                        children: vec![Node {
                            is_dir: true,
                            path: "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d".to_string(),
                            children: vec![Node {
                                is_dir: true,
                                path: "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d/e"
                                    .to_string(),
                                children: vec![Node {
                                    is_dir: false,
                                    path:
                                        "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d/e/f.txt"
                                            .to_string(),
                                    children: vec![],
                                }],
                            }],
                        }],
                    }],
                }],
            }],
        }
    }

    let expected_output = vec![
        "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d/e".to_string(),
        "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d".to_string(),
        "/Users/abrar/Documents/github/ft-sync/docs/a/b/c".to_string(),
        "/Users/abrar/Documents/github/ft-sync/docs/a/b".to_string(),
        "/Users/abrar/Documents/github/ft-sync/docs/a".to_string(),
    ];

    let output = dir_till_path(
        &test_node(),
        "/Users/abrar/Documents/github/ft-sync/docs/a/b/c/d/e/f.txt",
    );

    assert_eq!(expected_output, output);
}
