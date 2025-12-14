use std::{error::Error, fs::File, io::{Read, Seek, SeekFrom, Write}, path::Path};

use crate::{consts::{MAX_RECORD_LENGTH, PADDING_CHAR}, data::Data, record::Record};

#[derive(Debug)]
pub struct DataFile {
    file: File,
    pub next_id: u64,
}

impl PartialEq for DataFile {
    fn eq(&self, other: &Self) -> bool {
        //self.file == other.file
        true
    }
}

impl DataFile {
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

    pub fn get_data(&mut self, record: &Record) -> Result<Data, Box<dyn Error>>{
        let mut buf = [PADDING_CHAR; MAX_RECORD_LENGTH];

        self.file.seek(SeekFrom::Start(record.data_address()))?;

        if let Ok(_) = self.file.read_exact(&mut buf) {
            Ok(Data::try_from(buf.as_slice())?)
        } else {
            return Err("Error reading data from file".into());
        }
    }

    pub fn write_data(&mut self, record: &Record, data: &Data) -> Result<(), Box<dyn Error>>{
        self.file.seek(SeekFrom::Start(record.data_address()))?;

        self.file.write(Vec::from(data).as_slice())?;

        self.next_id+=1;
        Ok(())
    }

    pub fn update_data(&mut self, record: &Record, data: &Data) -> Result<(), Box<dyn Error>>{
        self.file.seek(SeekFrom::Start(record.data_address()))?;

        self.file.write(Vec::from(data).as_slice())?;

        Ok(())
    }


    /*
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
        //println!("{:?}", nodes);
        self.next_id += 1;

        self.file.rewind().unwrap();
        self.file.set_len(0).unwrap();
        let writer = BufWriter::new(self.file.try_clone().unwrap());
        serde_json::to_writer(writer, &nodes).unwrap();

        self.file.sync_data().unwrap();
        self.file.rewind().unwrap();
    }*/
}

