use fuse::{FileAttr};
use std::collections::HashMap;

pub struct Disk {
    memory_blocks: Box<[MemoryBlock]>,
    block_size: usize
}

pub enum MemoryBlock {
    InodeTable(HashMap<u64, FileAttr>),
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

    pub fn get_content_from_block(&mut self, block_index: usize) -> &mut Box<[u8]> {
        match &mut self.memory_blocks[block_index] {
            MemoryBlock::Data(data) => data,
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }
}