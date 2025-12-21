use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Write},
    path::Path,
};

use rand::{Rng, rng, seq::IndexedRandom};

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

fn generate_instructions(i: usize) -> String {
    let instructions = ["i", "u", "d"];
    //let instructions = ["i"];
    match instructions.choose(&mut rng()) {
        Some(instr) => match *instr {
            "i" => {
                format!(
                    "i {} {}\n",
                    rng().random_range(0..i),
                    rng().random::<Data>()
                )
            }
            "d" => {
                format!("d {} \n", rng().random_range(0..i),)
            }
            "u" => {
                format!(
                    "u {} {} {}\n",
                    rng().random_range(0..i),
                    rng().random_range(0..i),
                    rng().random::<Data>()
                )
            }
            _ => todo!(),
        },
        None => todo!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_address: &String = &args[1];
    if args[2] == "gen" {
        let instruction_count = args[3].parse().unwrap();
        let mut file = File::options()
            .create(true)
            .read(true)
            .truncate(true)
            .write(true)
            .open(file_address)
            .unwrap();

        for _ in 0..instruction_count {
            file.write(generate_instructions(2 * instruction_count).as_bytes())
                .unwrap();
        }

        println!("Generation completed");
        return;
    }
    let tree_order: usize = args[2].parse().unwrap();
    let print_opt = &args[3] == "y";
    let buf_size: usize = args[4].parse().unwrap();
    let cache_size: usize = args[5].parse().unwrap();

    let mut btree = BTree::new(
        &Path::new((file_address.clone() + "_nodes").as_str()),
        &Path::new((file_address.clone() + "_data").as_str()),
        tree_order,
        buf_size,
        cache_size,
    );

    while let Some(line) = io::stdin().lines().next() {
        match line {
            Ok(input) => match parse_input(&mut btree, &input) {
                Ok(_) => {
                    if print_opt {
                        btree.print();
                    }
                }
                Err(_) => {
                    eprintln!("Error parsing input or executing command")
                }
            },
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
            }
        }
    }
    btree.print();
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

            println!("Inserting| {}: {}", key, data);

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

            println!("Deleting| {}", key);

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

            println!("Updating| {} -> {}: {}", old_key, new_key, data_string);
            btree
                .update(old_key, new_key, data)
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
                }
                Err(_) => {
                    println!("Record with key: {key} was not found");
                }
            }

            Ok(())
        }
        unknown => Err(format!("Unknown operation: {unknown}").into()),
    }
}
