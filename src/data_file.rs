use std::{
    error::Error,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
    u64,
};

use crate::{
    consts::{MAX_RECORD_LENGTH, PADDING_CHAR},
    data::Data,
    record::Record,
};

#[derive(Debug)]
pub struct DataFile {
    pub file: File,
    pub next_id: u64,
    buffer: Vec<u8>,
    curr_buf_position: u64,
    curr_buf_len: u64,
    buffer_size: usize,
    pub file_write_ctr: u32,
    pub file_read_ctr: u32,
}

impl PartialEq for DataFile {
    fn eq(&self, _: &Self) -> bool {
        //self.file == other.file
        true
    }
}

impl DataFile {
    pub fn new(file_name: &Path, buffer_size: usize) -> Self {
        Self {
            file: File::options()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true) // TODO: switch to reading the node count and updating next_id accordingly
                .open(file_name)
                .unwrap(),
            next_id: 0,
            buffer: vec![PADDING_CHAR; buffer_size * MAX_RECORD_LENGTH],
            curr_buf_position: 0,
            curr_buf_len: 0,
            buffer_size,
            file_write_ctr: 0,
            file_read_ctr: 0,
        }
    }

    pub fn get_data(&mut self, record: &Record) -> Result<Data, Box<dyn Error>> {
        self.file.sync_data().unwrap();
        let record_buf_position = self.get_buf_pos(record.data_id);
        let data_buf_offset: usize = record.data_address() as usize - record_buf_position as usize;

        if record_buf_position != self.curr_buf_position
            || data_buf_offset as u64 >= self.curr_buf_len
        {
            self.read_buffer(record.data_id)?;
        }

        Data::try_from(&self.buffer[data_buf_offset..data_buf_offset + MAX_RECORD_LENGTH])
    }

    pub fn write_data(&mut self, record: &Record, data: &Data) -> Result<(), Box<dyn Error>> {
        let record_buf_position = self.get_buf_pos(record.data_id);
        let data_buf_offset: usize = record.data_address() as usize - record_buf_position as usize;

        if record_buf_position != self.curr_buf_position {
            self.read_buffer(record.data_id)?;
        }

        self.buffer[data_buf_offset..data_buf_offset + MAX_RECORD_LENGTH]
            .copy_from_slice(Vec::from(data).as_slice());
        self.next_id += 1;

        Ok(())
    }

    pub fn update_data(&mut self, record: &Record, data: &Data) -> Result<(), Box<dyn Error>> {
        let record_buf_position = self.get_buf_pos(record.data_id);
        let data_buf_offset: usize = record.data_address() as usize - record_buf_position as usize;

        if record_buf_position != self.curr_buf_position {
            self.read_buffer(record.data_id)?;
        }

        self.buffer[data_buf_offset..data_buf_offset + MAX_RECORD_LENGTH]
            .copy_from_slice(Vec::from(data).as_slice());
        Ok(())
    }

    fn write_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.seek(SeekFrom::Start(self.curr_buf_position))?;
        self.file.write(&self.buffer)?;
        self.file.sync_data()?;
        self.file_write_ctr+=1;
        Ok(())
    }

    fn get_buf_pos(&self, data_id: u64) -> u64 {
        data_id / self.buffer_size as u64 * self.buffer_size as u64 * MAX_RECORD_LENGTH as u64
    }

    fn read_buffer(&mut self, data_id: u64) -> Result<(), Box<dyn Error>> {
        self.write_buffer()?;
        self.buffer.fill(PADDING_CHAR);

        let buf_pos = self.get_buf_pos(data_id);

        self.file.seek(SeekFrom::Start(buf_pos))?;
        self.curr_buf_len = self.file.read(&mut self.buffer).unwrap_or(0) as u64;
        self.curr_buf_position = buf_pos;

        self.file_read_ctr+=1;

        Ok(())
    }
}

impl Drop for DataFile {
    fn drop(&mut self) {
        self.write_buffer().unwrap();
    }
}
