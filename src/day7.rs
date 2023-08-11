use std::fmt::{Display, Formatter, write};

enum Node {
    Dir { name: String, nodes: Vec<Box<Node>> },
    File { name: String, size: usize }
}

impl Node {
    fn empty_dir(name: &str) -> Node {
        Node::Dir {
            name: name.to_string(),
            nodes: vec![]
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Dir { nodes, .. } => {
                nodes.iter()
                    .map(|node| node.size())
                    .sum()
            }
            Node::File { size, .. } => {
                *size
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Node::Dir { name, .. } => {
                name
            }
            Node::File { name, .. } => {
                name
            }
        }
    }

    fn add_node(&mut self, other: Node) {
        match self {
            Node::Dir { nodes, .. } => {
                nodes.push(Box::new(other));
            }
            Node::File { .. } => {
                panic!("Tried to add node to File");
            }
        }
    }

    fn has_node(&self, name: &str) -> bool {
        self.get_node(name).is_some()
    }

    fn get_node(&self, name: &str) -> Option<&Box<Node>> {
        match self {
            Node::Dir { nodes, .. } => {
                nodes.iter().find(|node| node.name() == name)
            }
            Node::File { .. } => {
                None
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Dir { name, nodes } => {
                write!(f, "- {} (dir)", name)?;
                for node in nodes.iter() {
                    write!(f, "\t{}", node)?;
                }
            }
            Node::File { name, size } => {
                write!(f, "- {} (file, size={})", name, size)?;
            }
        }

        Ok(())
    }
}

pub fn question1() -> usize {
    let mut root_dir = Node::Dir {
        name: "/".to_string(),
        nodes: vec![]
    };

    let mut dirs: Vec<&mut Node> = vec![&mut root_dir];

    let file = crate::read_input_file(7);

    for line in file.lines() {
        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(dir) = command.strip_prefix("cd ") {
                if dir == ".." {
                    dirs.remove(dirs.len() - 1);
                    continue;
                }
                dirs.push(&mut dirs.last().unwrap().get_node(dir).unwrap());
            } else {
                continue;
            }
        } else if let Some(dir) = line.strip_prefix("dir ") {
            dirs.last_mut().unwrap().add_node(Node::empty_dir(dir));
        } else {
            // File
            let parts = line.split(' ')
                .collect::<Vec<&str>>();
            let size: usize = parts[0].parse()
                .expect("Failed to parse number");
            let name = parts[1].to_string();

            dirs.last_mut().unwrap().add_node(Node::File {
                name,
                size
            })
        }
    }

    println!("{}", root_dir);

    0usize
}