use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};

use crate::node::Node;

#[derive(Debug)]
pub struct NodesFile {
    pub file: File,
    pub next_id: u64,
    pub order: usize,
    pub node_byte_size: usize,
    pub node_buffer: Vec<u8>,
    pub cache: HashMap<u64, Node>,
    cache_size: usize,
    pub file_write_ctr: u32,
    pub file_read_ctr: u32,
}

impl PartialEq for NodesFile {
    fn eq(&self, _: &Self) -> bool {
        //self.file == other.file
        true
    }
}

impl NodesFile {
    pub fn new(file_name: &Path, order: usize, cache_size: usize) -> Self {
        let node_byte_size = Node::byte_size(order);
        Self {
            file: File::options()
                .create(true)
                .read(true)
                .write(true)
                .open(file_name)
                .unwrap(),
            next_id: 0,
            order,
            node_byte_size,
            node_buffer: vec![0; node_byte_size],
            cache: HashMap::new(),
            cache_size,
            file_write_ctr: 0,
            file_read_ctr: 0,
        }
    }

    pub fn get_node(&mut self, id: u64) -> Node {
        if self.cache.contains_key(&id) {
            self.cache.get(&id).unwrap().clone()
        } else {
            self.file_read_ctr += 1;
            self.file
                .seek(std::io::SeekFrom::Start(id * self.node_byte_size as u64))
                .unwrap();
            self.file
                .read_exact(self.node_buffer.as_mut_slice())
                .unwrap();
            let node = Node::from_bytes(self.node_buffer.as_slice(), self.order);
            self.add_to_cache(&node);
            node
        }
    }

    pub fn update_node(&mut self, node: &Node) {
        self.add_to_cache(node);
    }

    pub fn add_to_cache(&mut self, node: &Node) {
        self.cache.insert(node.id, node.clone());
        if self.cache.len() >= self.cache_size {
            println!("Writing cache");
            self.write_cache();
        }
    }

    pub fn create_node(&mut self, node: &Node) {
        self.add_to_cache(node);
        self.next_id += 1;
    }

    pub fn write_cache(&mut self) {
        for (_, node) in self.cache.iter() {
            self.file
                .seek(std::io::SeekFrom::Start(
                    node.id * self.node_byte_size as u64,
                ))
                .unwrap();
            self.node_buffer = node.to_bytes(self.order);
            self.file.write(self.node_buffer.as_slice()).unwrap();
            self.file_write_ctr += 1;
        }
        self.file.sync_data().unwrap();
        self.cache.clear();
    }
}

impl Drop for NodesFile {
    fn drop(&mut self) {
        self.write_cache();
    }
}
