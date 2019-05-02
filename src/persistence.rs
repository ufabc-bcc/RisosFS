use fuse::{FileAttr};
use std::str;
use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use crate::serialization::FileAttrDef;
use bincode::{serialize, deserialize};
use time::{Timespec, Tm};
use fuse::{FileType};

big_array! { BigArray; }

pub struct Disk {
    super_block: Box<[Option<Inode>]>,
    memory_blocks: Box<[MemoryBlock]>,
    max_files: usize,
    block_size: usize,
    root_path: String
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
    data: Option<Box<[u8]>>
}

impl Disk {

    /// Inicializa um disco virtual com o tamanho total especificado em `memory_size_in_bytes` e com cada bloco contendo um tamanho fixo definido em `block_size`.
    /// O número de blocos alocados é definido pela expressão `memory_size_in_bytes / block_size`.
    pub fn new(
        root_path: String,
        memory_size_in_bytes: usize,
        block_size: usize
    ) -> Disk {
        // Quantidade de blocos de memória
        let block_quantity: usize = (memory_size_in_bytes / block_size) - 1;
        // Está sendo considerado o tamanho do ponteiro do Box além do tamanho da struct de Inode
        let inode_size = mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>();
        let max_files = block_size / inode_size;

        let disk_file_path = format!("{}/.disco.risos", &root_path);
        let inode_table_file_path = format!("{}/.inode.risos", &root_path);

        // Tenta ler o arquivo do disco, se nao existir cria um novo
        let mut memory_blocks: Vec<MemoryBlock>;
        let mut super_block: Vec<Option<Inode>>;

        if Path::new(&disk_file_path).exists() && Path::new(&inode_table_file_path).exists() {
            println!("Disco existente encontrado! Carregando...");

            let mut ser_inodes: Vec<u8> = Vec::new();
            let mut ser_disk: Vec<u8> = Vec::new();

            File::open(&inode_table_file_path).unwrap().read(&mut ser_inodes).unwrap();
            File::open(&disk_file_path).unwrap().read(&mut ser_disk).unwrap();

            super_block = if &ser_inodes.len() > &0 {
                deserialize(&ser_inodes).expect("Erro lendo disco persistido!")
             } else {
                Vec::new()
            };

            memory_blocks = if &ser_disk.len() > &0 {
                deserialize(&ser_disk).expect("Erro lendo disco persistido!")
            } else {
                Vec::new()
            };

            // Se o numero de blocos do disco existente for maior que o do disco a ser criado, termina a execuçao
            if (block_quantity - 1) < memory_blocks.len() {
                panic!("O disco existente e maior que o disco atual! Tente inicializar com um disco de tamanho maior!");
            }
        } else {
            File::create(&disk_file_path).expect("Erro criando arquivos para persistencia!");
            File::create(&inode_table_file_path).expect("Erro criando arquivos para persistencia!");

            super_block = Vec::new();
            memory_blocks = Vec::new();            
        };

        // Instanciando em branco outras posiçoes possiveis para maior velocidade
        for _ in super_block.len()..max_files {
            let value: Option<Inode> = Option::None;
            super_block.push(value);
        }

        for _ in memory_blocks.len()..block_quantity {
            let value: MemoryBlock = MemoryBlock { data: Option::None };
            memory_blocks.push(value);
        }

        println!("Done =)");

        println!("\nTamanho do disco (kbytes): {}", memory_size_in_bytes / 1024);
        println!("Tamanho do bloco de memória (kbytes): {}", block_size / 1024);
        println!("Quantidade máxima de arquivos (Inode {} bytes): {}", inode_size, max_files);

        Disk {
            memory_blocks: memory_blocks.into_boxed_slice(),
            super_block: super_block.into_boxed_slice(),
            max_files,
            block_size,
            root_path
        }
    }

    pub fn find_index_of_empty_inode(&self) -> Option<usize> {
        for (index, inode) in self.super_block.iter().enumerate() {
            if let Option::None = inode {
                return Option::Some(index);
            }
        }

        Option::None
    }

    pub fn find_index_of_empty_memory_block(&self) -> Option<usize> {
        for (index, memory_block) in self.memory_blocks.iter().enumerate() {
            if let Option::None = memory_block.data {
                return Option::Some(index);
            }
        }

        Option::None
    }

    pub fn write_inode(&mut self, index: usize, inode: Inode) {
        if mem::size_of_val(&inode) > self.block_size {
            println!("Não foi possível salvar o inode: tamanho maior que o tamanho do bloco de memória");
            return;
        }

        self.super_block[index] = Some(inode);
    }

    pub fn get_inode(&self, index: usize) -> Option<&Inode> {
        match &self.super_block[index] {
            Some(inode) => Some(inode),
            None => None
        }
    }

    /// Recupera o conteúdo de um bloco de memória convertido para `str`
    pub fn get_content(&self, block_index: usize) -> Option<&str> {
        let data = self.get_content_as_bytes(block_index);
        
        match &data {
            Some(data) => {
                Option::Some(str::from_utf8(&data).unwrap())
            },
            None => None
        }
    }

    /// Recupera um array de bytes borrowed de um bloco especificado.
    ///
    /// # Exemplos
    ///
    /// ```.
    /// let disk = Disk::new(args);
    /// let content: [u8] = disk.get_content_as_bytes(1);
    /// ```
    pub fn get_content_as_bytes(&self, block_index: usize) -> &Option<Box<[u8]>> {
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

        let memory_block = MemoryBlock { data: Option::Some(content) };
        self.memory_blocks[block_index] = memory_block;
    }

    pub fn write_to_disk(&mut self) {
        match serialize(&self.super_block) {
            Err(e) => {
                print!("Erro ao tentar escrever para arquivo de inodes! {}", e);
                return;
            },
            Ok(v) => {
                let inode_file = format!("{}/.inodes.risos", &self.root_path);
                let mut inode_file = OpenOptions::new().write(true).open(inode_file).unwrap();
                match inode_file.write(&v) {
                    Err(e) => {
                        print!("Erro ao tentar escrever para arquivo de inodes! {}", e);
                        return;
                    },
                    Ok(v) => v,
                };
            },
        };

        match serialize(&self.memory_blocks) {
            Err(e) => {
                print!("Erro ao tentar escrever para arquivo de disco! {}", e);
                return;
            },
            Ok(v) => {
                let disk_file = format!("{}/.disco.risos", &self.root_path);
                let mut disk_file = OpenOptions::new().write(true).open(disk_file).unwrap();
                match disk_file.write(&v) {
                    Err(e) => {
                        print!("Erro ao tentar escrever para arquivo de inodes! {}", e);
                        return;
                    },
                    Ok(v) => v,
                };
            },
        };
    }
}