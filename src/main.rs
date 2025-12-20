use std::{env, error::Error, io, path::Path};

use crate::{
    btree::{BTree, InsertedData},
    data::Data,
};

mod btree;
mod consts;
mod data;
mod data_file;
mod node;
mod nodes_file;
mod record;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_address: &String = &args[1];
    let tree_order: usize = args[2].parse().unwrap();
    let print_opt = &args[3] == "y";

    let mut btree = BTree::new(
        &Path::new((file_address.clone() + "_nodes").as_str()),
        &Path::new((file_address.clone() + "_data").as_str()),
        tree_order,
    );

    while let Some(line) = io::stdin().lines().next() {
        match line {
            Ok(input) => {
                match parse_input(&mut btree, &input){
                    Ok(_) => {
                        if print_opt {
                            btree.print();
                        }
                    },
                    Err(_) => {
                        eprintln!("Error parsing input or executing command")
                    },
                }
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
            }
        }
    }
}

fn parse_input(btree: &mut BTree, input: &String) -> Result<(), Box<dyn Error>> {
    match input.chars().next().unwrap() {
        'i' => {
            let mut split = input.split(" ");
            split.next();
            let key_unparsed = split.next().ok_or("Failed to read key")?;
            let key: u64 = key_unparsed.parse()?;
            let mut data_string = String::new();
            while let Some(s) = split.next() {
                data_string.push_str(" ");
                data_string.push_str(s);
            }
            let data = Data::try_from(data_string.as_bytes()).unwrap();

            btree
                .insert(key, btree::InsertedData::NewData(data))
                .unwrap_or_else(|e| println!("Error inserting record: {e:?}"));

            Ok(())
        }
        'd' => {
            let mut split = input.split(" ");
            split.next();
            let key_unparsed = split.next().ok_or("Failed to read key")?;
            let key: u64 = key_unparsed.parse()?;

            btree
                .delete(key)
                .unwrap_or_else(|e| println!("Error deleting record: {e:?}"));

            Ok(())
        }
        'u' => {
            let mut split = input.split(" ");
            split.next();
            let old_key_unparsed = split.next().ok_or("Failed to read old key")?;
            let old_key: u64 = old_key_unparsed.parse()?;

            let new_key_unparsed = split.next().ok_or("Failed to read new key")?;
            let new_key: u64 = new_key_unparsed.parse()?;

            let mut data_string = String::new();
            while let Some(s) = split.next() {
                data_string.push_str(" ");
                data_string.push_str(s);
            }
            let data = if data_string.len() > 0 {
                InsertedData::NewData(Data::try_from(data_string.as_bytes()).unwrap())
            } else {
                InsertedData::None
            };

            btree.update(old_key, new_key, data)
                .unwrap_or_else(|e| println!("Error updating record: {e:?}"));

            Ok(())
        }
        's' => {
            let mut split = input.split(" ");
            split.next();
            let key_unparsed = split.next().ok_or("Failed to read key")?;
            let key: u64 = key_unparsed.parse()?;

            match btree.search(key) {
                Ok(record_data) => {
                    println!("Record with key: {key} was found");
                    let data = btree.data_file.get_data(&record_data.0).unwrap();
                    println!("{}: {}", key, data);
                },
                Err(_) => {
                    println!("Record with key: {key} was not found");
                }
            }

            Ok(())

        }
        unknown => Err(format!("Unknown operation: {unknown}").into()),
    }
}
