use std::path::Path;

use crate::{btree::{BTree, Inserted_Data}, data::Data};

mod btree;
mod consts;
mod data;
mod data_file;
mod node;
mod nodes_file;
mod record;
mod test;

fn main() {
    let mut btree = BTree::new(
        &Path::new("test_files/btree"),
        &Path::new("test_files/data"),
        2,
    );
    let data_count: u64 = 50;
    for (i, data) in rand::random_iter::<Data>().take(data_count as usize).enumerate() {
        println!("{}: {}", data_count - i as u64, data);
        btree.insert(data_count - i as u64, Inserted_Data::NewData(data)).unwrap();
        //btree.print();
        //btree.print_all_nodes();
    }

    btree.print();
}
