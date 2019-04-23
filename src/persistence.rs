use fuse::{FileAttr};
use std::collections::HashMap;
use std::str;
use std::mem;

pub struct Disk {
    memory_blocks: Box<[MemoryBlock]>,
    block_size: usize
}

pub struct Inode {
    pub path: String,
    pub content_location: usize,
    pub attributes: FileAttr
}

/// Enumerate de MemoryBlock. Aceita dois tipos: um `Vec<Inode>` e será representado como uma estrutura de `InodeTable`, 
/// ou então um `Box<[u8]>`, que será representado como uma estrutura de `Data`
pub enum MemoryBlock {
    InodeTable(Vec<Inode>),
    Data(Box<[u8]>)
}

impl Disk {

    /// Inicializa um disco virtual com o tamanho total especificado em `memory_size_in_bytes` e com cada bloco contendo um tamanho fixo definido em `block_size`.
    /// O número de blocos alocados é definido pela expressão `memory_size_in_bytes / block_size`.
    pub fn new(
        memory_size_in_bytes: usize,
        block_size: usize
    ) -> Disk {
        let block_quantity: usize = memory_size_in_bytes / block_size;
        let mut memory_blocks: Vec<MemoryBlock> = Vec::with_capacity(block_quantity);

        let max_files = block_size / mem::size_of::<Inode>();
        let inode_table: Vec<Inode> = Vec::with_capacity(max_files);

        // O primeiro índice do memory_block é dedicado para a tabela (HashMap) de Inode
        memory_blocks.push(MemoryBlock::InodeTable(inode_table));

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

    pub fn get_inode_table(&mut self) -> &mut Vec<Inode> {
        match &mut self.memory_blocks[0] {
            MemoryBlock::Data(_) => { panic!("Can not return data from memory allocation specified") },
            MemoryBlock::InodeTable(inode_table) => inode_table
        }
    }

    /// Recupera o conteúdo de um bloco de memória convertido para `str`

    pub fn get_content(&self, block_index: usize) -> &str {
        let data = self.get_content_as_bytes(block_index);
        str::from_utf8(data).unwrap()
    }

    /// Recupera um array de bytes borrowed de um bloco especificado.
    ///
    /// # Exemplos
    ///
    /// ```
    /// let disk = Disk::new(args);
    /// let content: [u8] = disk.get_content_as_bytes(1);
    /// ```

    pub fn get_content_as_bytes(&self, block_index: usize) -> &[u8] {
        match &self.memory_blocks[block_index] {
            MemoryBlock::Data(data) => data,
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }

    /// Escreve um conteúdo em string em um bloco de memória

    pub fn write_content(&mut self, block_index: usize, content: &str) {
        let content: Box<[u8]> = Box::from(content.as_bytes());
        self.write_content_as_bytes(block_index, &content);
    }

    /// Escreve dados em bytes em um bloco de memória
    ///
    ///  # Exemplos
    /// 
    /// ```
    /// let content: Box<[u8]> = Box::from(content.as_bytes());
    /// let disk: Disk = Disk::new(1024 * 1024, 1024);
    /// disk.write_content_as_bytes(1, &content);
    /// ```
    /// 
    /// Somente é gravado se for um local de memória válido
    pub fn write_content_as_bytes(&mut self, block_index: usize, content: &Box<[u8]>) {
        match &mut self.memory_blocks[block_index] {
            MemoryBlock::Data(_) => {
                let content = content.clone(); // TODO:  verificar se há método melhor para gravar conteúdo
                let content = MemoryBlock::Data(content);
                self.memory_blocks[block_index] = content;
            },
            MemoryBlock::InodeTable(_) => { panic!("Can not return data from memory allocation specified") }
        }
    }

}