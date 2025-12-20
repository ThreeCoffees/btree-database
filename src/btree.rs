use std::{io::Read, path::Path};

use crate::{
    btree, data::Data, data_file::DataFile, node::Node, nodes_file::NodesFile, record::Record,
};

#[derive(PartialEq, Debug)]
pub struct BTree {
    pub root_id: Option<u64>,
    pub nodes_file: NodesFile,
    pub data_file: DataFile,
    pub order: usize,
}

#[derive(Debug)]
pub enum Inserted_Data {
    NewData(Data),
    ExistingData(u64),
    None,
}

impl BTree {
    pub fn new(nodes_file_name: &Path, data_file_name: &Path, order: usize) -> Self {
        Self {
            root_id: None,
            nodes_file: NodesFile::new(nodes_file_name, order),
            data_file: DataFile::new(data_file_name),
            order,
        }
    }

    pub fn get_next_id(&self) -> u64 {
        self.nodes_file.next_id
    }

    pub fn get_node(&mut self, id: u64) -> Node {
        self.nodes_file.get_node(id)
    }

    pub fn update_node(&mut self, node: &Node) {
        self.nodes_file.update_node(node);
    }

    pub fn create_node(&mut self, node: &Node) {
        self.nodes_file.create_node(node);
    }

    pub fn insert(&mut self, key: u64, data: Inserted_Data) -> Result<(), ()> {
        match self.search(key) {
            Ok(_) => Err(()),
            Err((_, node_id)) => {
                if let None = self.root_id {
                    self.create_new_root(true);
                }
                let mut node = self.nodes_file.get_node(node_id);
                let data_id = if let Inserted_Data::ExistingData(id) = data {
                    id
                } else {
                    self.data_file.next_id
                };
                let record = Record::new(key, data_id);
                if let Inserted_Data::NewData(data) = data {
                    self.data_file.write_data(&record, &data).unwrap();
                }
                node.insert(self, record, None, None)
            }
        }
    }

    pub fn delete(&mut self, key: u64) -> Result<(), ()> {
        let (_, node_id) = self.search(key).map_err(|_| ())?;

        self.get_node(node_id).delete(self, key)?;
        Ok(())
    }

    pub fn print(&mut self) {
        println!("\n=== B Tree ===");
        println!("Order: {}", self.order);
        println!("Nodes file: {:?}", self.nodes_file.file);
        println!("Data file: {:?}", self.data_file.file);
        println!("--- Nodes ---");
        if let Some(root_id) = self.root_id {
            let root = self.get_node(root_id);
            root.print(self, 0);
        }
        println!("--- Records ---");
        if let Some(root_id) = self.root_id {
            let root = self.get_node(root_id);
            root.print_in_order(self);
        }
        println!("=== === === ===");
    }

    pub fn print_all_nodes(&mut self) {
        println!("\n=== B Tree Nodes ===");

        let node_count = self.nodes_file.next_id;
        for id in 0..node_count {
            println!("{:?}", self.nodes_file.get_node(id));
        }

        println!("=== === === === ===");
    }

    pub fn update(&mut self, old_key: u64, new_key: u64, new_data: Inserted_Data) -> Result<(), ()> {
        if let Ok((record, _)) = self.search(old_key) {
            if let Ok(_) = self.search(new_key)
                && old_key != new_key
            {
                return Err(());
            }

            if let Inserted_Data::NewData(data) = new_data {
                self.data_file.write_data(&record, &data).unwrap();
            }

            if old_key != new_key {
                self.delete(old_key)?;
                self.insert(new_key, Inserted_Data::ExistingData(record.data_id))?;
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn create_new_root(&mut self, is_leaf: bool) -> Node {
        let root = Node::new(is_leaf, self.get_next_id(), self.order);
        self.nodes_file.create_node(&root);
        self.root_id = Some(root.id);
        root
    }

    pub fn search(&mut self, key: u64) -> Result<(Record, u64), (u64, u64)> {
        match self.root_id {
            Some(id) => self.get_node(id).search(self, key),
            None => Err((0, 0)),
        }
    }
}
