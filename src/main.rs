use std::path::Path;

use crate::{btree::BTree, data::Data};

mod btree;
mod consts;
mod data;
mod data_file;
mod node;
mod nodes_file;
mod record;
mod test;

fn main() {
    let mut btree = BTree::new(&Path::new("test_files/btree"), &Path::new("test_files/data"), 2);
    for (i, data) in rand::random_iter::<Data>().take(16).enumerate() {
        println!("{}: {}", i, data);
        btree.insert(i as u64, &data).unwrap();
    }
    
}
