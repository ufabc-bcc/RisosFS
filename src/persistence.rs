use fuse::{FileAttr};
use std::str;
use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Error;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::serialization::FileAttrDef;
use bincode::{serialize, deserialize};

big_array! { BigArray; }

pub struct Disk {
    super_block: Box<[Option<Inode>]>,
    memory_blocks: Box<[MemoryBlock]>,
    max_files: usize,
    block_size: usize
}

#[derive(Serialize, Deserialize)]
pub struct Inode {
    #[serde(with = "BigArray")]
    pub name: [char; 64],
    #[serde(with = "FileAttrDef")]
    pub attributes: FileAttr
}

#[derive(Serialize, Deserialize)]
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
        let block_quantity: usize = (memory_size_in_bytes / block_size) - 1;
        // Está sendo considerado o tamanho do ponteiro do Box além do tamanho da struct de Inode
        let max_files = (block_size + 1) / (mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>());

        // Vetor de blocos de memoria
        let mut super_block: Vec<Option<Inode>> = Vec::with_capacity(max_files);

        // Tabela de Inodes
        let mut memory_blocks: Vec<MemoryBlock> = Vec::with_capacity(block_quantity);

        // Tenta ler o arquivo do disco, se nao existir cria um novo
        if Path::new(".disco.txt").exists() && Path::new(".inodes.txt").exists() {
            println!("Disco existente encontrado! Carregando...");
            let mut ser_inodes: Vec<u8> = Vec::new();
            let mut ser_disk: Vec<u8> = Vec::new();
            File::open(".inodes.risos").unwrap().read(&mut ser_inodes).unwrap();
            File::open(".disco.risos").unwrap().read(&mut ser_disk).unwrap();

            let mut super_block: Vec<Option<Inode>> = match deserialize(&ser_inodes) {
                Err(e) => panic!("Erro lendo disco persistido! {}", e),
                Ok(v) => v,
            };
            let mut memory_blocks: Vec<MemoryBlock> = match deserialize(&ser_disk) {
                Err(e) => panic!("Erro lendo disco persistido! {}", e),
                Ok(v) => v,
            };

            // Se o numero de blocos do disco existente for maior que o do disco a ser criado, termina a execuçao
            if (block_quantity - 1) < memory_blocks.len() {
                panic!("O disco existente e maior que o disco atual! Tente inicializar com um disco de tamanho maior!");
            }

            // Instanciando em branco outras posiçoes possiveis para maior velocidade
            for _ in (super_block.len() + 1)..max_files {
                let value: Option<Inode> = Option::None;
                super_block.push(value);
            }

            for _ in (memory_blocks.len() + 1)..block_quantity {
                let value: MemoryBlock = MemoryBlock { data: Box::default() };
                memory_blocks.push(value);
            }

            println!("Done =)");
        } else {
            match File::create(".disco.risos") {
                Err(e) => panic!("Erro criando arquivos para persistencia!"),
                Ok(v) => v,
            };
            match File::create(".inodes.risos") {
                Err(e) => panic!("Erro criando arquivos para persistencia!"),
                Ok(v) => v,
            };

            for _ in 0..block_quantity {
                let value: MemoryBlock = MemoryBlock { data: Box::default() };
                memory_blocks.push(value);
            }

            for _ in 0..max_files {
                let value: Option<Inode> = Option::None;
                super_block.push(value);
            }
        };

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