use std::path::Path;

use crate::{node::Node, nodes_file::NodesFile};

#[derive(PartialEq, Debug)]
pub struct BTree {
    pub root_id: Option<u64>,
    pub nodes_file: NodesFile,
    pub order: usize,
}

impl BTree {
    pub fn new(nodes_file_name: &Path, order: usize) -> Self {
        Self {
            root_id: None,
            nodes_file: NodesFile::new(nodes_file_name),
            order,
        }
    }

    pub fn get_next_id(&self) -> u64 {
        self.nodes_file.next_id
    }

    pub fn get_node(&mut self, id: u64) -> Node {
        self.nodes_file.get_node(id)
    }

    pub fn insert(&mut self, key: u64) -> Result<(), ()> {
        match self.search(key) {
            Ok(_) => Err(()),
            Err((_, node_id)) => {
                if let None = self.root_id {
                    self.create_new_root(true);
                }
                let mut node = self.nodes_file.get_node(node_id);
                node.insert(self, key, None, None)
            }
        }
    }

    pub fn delete(&mut self, key: u64) -> Result<(), ()> {
        let (_, node_id) = self.search(key).map_err(|_| ())?;

        self.get_node(node_id).delete(self, key)?;
        Ok(())
    }

    pub fn update(&mut self, old_key: u64, new_key: u64) -> Result<(), ()> {
        if let Ok(_) =  self.search(new_key) {
            return Err(())
        }
        self.delete(old_key)?;
        self.insert(new_key)?;

        Ok(())
    }

    pub fn create_new_root(&mut self, is_leaf: bool) -> Node {
        let root = Node::new(is_leaf, self.get_next_id(), self.order);
        self.nodes_file.create_node(&root);
        self.root_id = Some(root.id);
        root
    }

    pub fn search(&mut self, key: u64) -> Result<(u64, u64), (u64, u64)> {
        match self.root_id {
            Some(id) => self.get_node(id).search(self, key),
            None => Err((0, 0)),
        }
    }
}
