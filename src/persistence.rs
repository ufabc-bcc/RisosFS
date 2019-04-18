use fuse::{FileAttr};
use std::collections::HashMap;
use std::str;

pub struct Disk {
    memory_blocks: Box<[MemoryBlock]>,
    block_size: usize
}

pub struct Inode {
    pub path: String,
    pub content_location: usize,
    pub attributes: FileAttr
}

pub enum MemoryBlock {
    InodeTable(HashMap<String, Inode>),
    Data(Box<[u8]>)
}

impl Disk {
    pub fn new(
        memory_size_in_bytes: usize,
        block_size: usize
    ) -> Disk {
        let block_quantity: usize = memory_size_in_bytes / block_size;
        let mut memory_blocks: Vec<MemoryBlock> = Vec::with_capacity(block_quantity);

        memory_blocks.push(MemoryBlock::InodeTable(HashMap::new()));

        for _ in 1..block_quantity - 1 {
            let data: Vec<u8> = Vec::with_capacity(block_size);
            let data: Box<[u8]> = data.into_boxed_slice();
            memory_blocks.push(MemoryBlock::Data(data));
        }

        Disk {
            memory_blocks: memory_blocks.into_boxed_slice(),
            block_size: block_size
        }
    }

    pub fn get_inode_table(&mut self) -> &mut HashMap<String, Inode> {
        match &mut self.memory_blocks[0] {
            MemoryBlock::Data(_) => { panic!("Can not return data from memory allocation specified") },
            MemoryBlock::InodeTable(inode_table) => inode_table
        }
    }

    pub fn get_content(&self, block_index: usize) -> &str {
        match &self.memory_blocks[block_index] {
            MemoryBlock::Data(data) => str::from_utf8(data).unwrap(),
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }

    pub fn get_content_as_bytes(&self, block_index: usize) -> &[u8] {
        match &self.memory_blocks[block_index] {
            MemoryBlock::Data(data) => data,
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }

    pub fn write_content(&mut self, block_index: usize, content: &str) {
        match &mut self.memory_blocks[block_index] {
            MemoryBlock::Data(_) => {
                let content: Box<[u8]> = content.as_bytes().to_vec().into_boxed_slice();
                let content = MemoryBlock::Data(content);
                self.memory_blocks[block_index] = content;
            },
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }
}