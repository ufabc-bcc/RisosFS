use fuse::{FileAttr};
use std::str;
use std::mem;

pub struct Disk {
    super_block: Box<[Option<Inode>]>,
    memory_blocks: Box<[MemoryBlock]>,
    max_files: usize,
    block_size: usize
}

pub struct Inode {
    pub name: [char; 64],
    pub attributes: FileAttr
}

pub struct MemoryBlock {
    data: Box<[u8]>
}

impl Disk {

    /// Inicializa um disco virtual com o tamanho total especificado em `memory_size_in_bytes` e com cada bloco contendo um tamanho fixo definido em `block_size`.
    /// O número de blocos alocados é definido pela expressão `memory_size_in_bytes / block_size`.
    pub fn new(
        memory_size_in_bytes: usize,
        block_size: usize
    ) -> Disk {
        // Quantidade de blocos de memória
        let block_quantity: usize = memory_size_in_bytes / block_size;
        // Está sendo considerado o tamanho do ponteiro do Box além do tamanho da struct de Inode
        let max_files = block_size / (mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>());

        // Preenchendo todo o vetor de bloco de memória
        let mut memory_blocks: Vec<MemoryBlock> = Vec::with_capacity(block_quantity);

        for _ in 0..block_quantity {
            let value: MemoryBlock = MemoryBlock { data: Box::default() };
            memory_blocks.push(value);
        }

        // Preenchendo toda a tabela de Inode
        let mut super_block: Vec<Option<Inode>> = Vec::with_capacity(max_files);

        for _ in 0..max_files {
            let value: Option<Inode> = Option::None;
            super_block.push(value);
        }

        println!("Tamanho do disco (kbytes): {}", memory_size_in_bytes / 1024);
        println!("Tamanho do bloco de memória (kbytes): {}", block_size / 1024);
        println!("Quantidade máxima de arquivos (Inode {} bytes): {}", (mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>()), max_files);

        Disk {
            memory_blocks: memory_blocks.into_boxed_slice(),
            super_block: super_block.into_boxed_slice(),
            max_files,
            block_size
        }
    }

    /// Recupera o conteúdo de um bloco de memória convertido para `str`
    pub fn get_content(&self, block_index: usize) -> &str {
        let data = self.get_content_as_bytes(block_index);
        str::from_utf8(&data).unwrap()
    }

    /// Recupera um array de bytes borrowed de um bloco especificado.
    ///
    /// # Exemplos
    ///
    /// ```.
    /// let disk = Disk::new(args);
    /// let content: [u8] = disk.get_content_as_bytes(1);
    /// ```
    pub fn get_content_as_bytes(&self, block_index: usize) -> &Box<[u8]> {
        let memory_block = &self.memory_blocks[block_index];
        return &memory_block.data;
    }

    /// Escreve um conteúdo em string em um bloco de memória
    pub fn write_content(&mut self, block_index: usize, content: &str) {
        let content: Box<[u8]> = Box::from(content.as_bytes());
        self.write_content_as_bytes(block_index, content);
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
    pub fn write_content_as_bytes(&mut self, block_index: usize, content: Box<[u8]>) {
        if content.len() > self.block_size {
            panic!("Não foi possível salvar o conteúdo do arquivo, pois excede o tamanho do bloco de memória {}", self.block_size);
        }

        let memory_block = MemoryBlock { data: content };
        self.memory_blocks[block_index] = memory_block;
    }
}