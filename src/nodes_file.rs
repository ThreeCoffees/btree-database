use std::{fs::File, io::{BufReader, BufWriter, Seek}, path::Path};

use crate::btree::Node;

#[derive(Debug)]
pub struct NodesFile {
    pub file: File,
    pub next_id: u64,
}

impl PartialEq for NodesFile {
    fn eq(&self, other: &Self) -> bool {
        //self.file == other.file
        true
    }
}

impl NodesFile {
    pub fn new(file_name: &Path) -> Self {
        Self {
            file: File::options()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true) // TODO: switch to reading the node count and updating next_id accordingly
                .open(file_name)
                .unwrap(),
            next_id: 0,
        }
    }

    pub fn get_node(&mut self, id: u64) -> Node {
        let reader = BufReader::new(self.file.try_clone().unwrap());

        let nodes: Vec<Node> = serde_json::from_reader(reader).unwrap();

        self.file.rewind().unwrap();

        nodes[id as usize].clone()
    }

    pub fn update_node(&mut self, node: &Node) {
        let mut nodes: Vec<Node> = vec![];

        let reader = BufReader::new(self.file.try_clone().unwrap());

        nodes = serde_json::from_reader(reader).unwrap();
        nodes[node.id as usize] = node.clone();

        println!("{:?}", nodes);

        self.file.rewind().unwrap();
        self.file.set_len(0).unwrap();
        let writer = BufWriter::new(self.file.try_clone().unwrap());
        serde_json::to_writer(writer, &nodes).unwrap();

        self.file.sync_data().unwrap();
        self.file.rewind().unwrap();
    }

    pub fn create_node(&mut self, node: &Node) {
        let mut nodes: Vec<Node> = vec![];

        let reader = BufReader::new(self.file.try_clone().unwrap());

        nodes = serde_json::from_reader(reader).unwrap_or(vec![]);
        nodes.push(node.clone());
        println!("{:?}", nodes);
        self.next_id += 1;

        self.file.rewind().unwrap();
        self.file.set_len(0).unwrap();
        let writer = BufWriter::new(self.file.try_clone().unwrap());
        serde_json::to_writer(writer, &nodes).unwrap();

        self.file.sync_data().unwrap();
        self.file.rewind().unwrap();
    }
}

