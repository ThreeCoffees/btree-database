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

fn generate_instructions(i: usize, instructions: &str) -> String {
    //let instructions = ["i", "u", "d"];
    //let instructions = ["i"];
    let instructions: Vec<char> = instructions.chars().collect();
    match instructions.choose(&mut rng()) {
        Some(instr) => match *instr {
            'i' => {
                format!(
                    "i {} {}\n",
                    rng().random_range(0..i),
                    rng().random::<Data>()
                )
            }
            'd' => {
                format!("d {} \n", rng().random_range(0..i),)
            }
            'u' => {
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
        let instructions = &args[4];
        let mut file = File::options()
            .create(true)
            .read(true)
            .truncate(true)
            .write(true)
            .open(file_address)
            .unwrap();

        for _ in 0..instruction_count {
            file.write(generate_instructions(2 * instruction_count, instructions).as_bytes())
                .unwrap();
        }

        println!("Generation completed");
        return;
    }
    let tree_order: usize = args[2].parse().unwrap();
    let print_opt = &args[3] == "y";
    let buf_size: usize = args[4].parse().unwrap();
    let cache_size: usize = args[5].parse().unwrap();
    let log_file_address: &String = &args[6];
    println!("{}", log_file_address);
    let is_logging = log_file_address != "";
    let mut log_file = if is_logging {
        Some(
            File::options()
                .create(true)
                .truncate(true)
                .write(true)
                .open(log_file_address)
                .unwrap(),
        )
    } else {
        None
    };

    if is_logging {
        log_file
            .as_mut()
            .unwrap()
            .write("command, index_writes, index_reads, data_writes, data_reads, tree_height, record_count\n".as_bytes())
            .unwrap();
    }

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
                Ok(c) => {
                    if print_opt {
                        btree.print();
                        println!(
                            "{}, {}, {}, {}, {}, {}, {}\n",
                            c,
                            btree.nodes_file.file_write_ctr,
                            btree.nodes_file.file_read_ctr,
                            btree.data_file.file_write_ctr,
                            btree.data_file.file_read_ctr,
                            btree.height,
                            btree.record_count,
                        )
                    }

                    if is_logging {
                        log_file
                            .as_mut()
                            .unwrap()
                            .write(
                                format!(
                                    "{}, {}, {}, {}, {}, {}, {}\n",
                                    c,
                                    btree.nodes_file.file_write_ctr,
                                    btree.nodes_file.file_read_ctr,
                                    btree.data_file.file_write_ctr,
                                    btree.data_file.file_read_ctr,
                                    btree.height,
                                    btree.record_count,
                                )
                                .as_bytes(),
                            )
                            .unwrap();
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
    println!("Index file reads/writes: ");
    println!("{}", btree.nodes_file.file_read_ctr);
    println!("{}", btree.nodes_file.file_write_ctr + 1);
    println!("Data file reads/writes: ");
    println!("{}", btree.data_file.file_read_ctr);
    println!("{}", btree.data_file.file_write_ctr + 1);
}

fn parse_input(btree: &mut BTree, input: &String) -> Result<String, Box<dyn Error>> {
    match input.chars().next().unwrap() {
        'l' => {
            println!("Logging|");
            println!("Index file reads/writes: ");
            println!("{}", btree.nodes_file.file_read_ctr);
            println!("{}", btree.nodes_file.file_write_ctr + 1);
            println!("Data file reads/writes: ");
            println!("{}", btree.data_file.file_read_ctr);
            println!("{}", btree.data_file.file_write_ctr + 1);
            Ok("l".into())
        }
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

            match btree.insert(key, btree::InsertedData::NewData(data)) {
                Ok(_) => Ok("i".into()),
                Err(_) => Err("Error inserting record".into()),
            }
        }
        'd' => {
            let mut split = input.split(" ");
            split.next();
            let key_unparsed = split.next().ok_or("Failed to read key")?;
            let key: u64 = key_unparsed.parse()?;

            println!("Deleting| {}", key);

            match btree.delete(key) {
                Ok(_) => Ok("d".into()),
                Err(_) => Err("Error deleting record".into()),
            }
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
            match btree.update(old_key, new_key, data) {
                Ok(_) => Ok("u".into()),
                Err(_) => Err("Error updating record".into()),
            }
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
                    Ok("s".into())
                }
                Err(_) => {
                    println!("Record with key: {key} was not found");
                    Ok("s".into())
                }
            }
        }
        unknown => Err(format!("Unknown operation: {unknown}").into()),
    }
}
