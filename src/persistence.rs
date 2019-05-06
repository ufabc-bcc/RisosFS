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
    pub attributes: FileAttr,
    #[serde(with = "BigArray")]
    pub references: [Option<usize>; 128]
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
        // O -1 é referente ao "superblock", que possui o mesmo tamanho de um MemoryBlock
        let memory_block_quantity: usize = (memory_size_in_bytes / block_size) - 1;
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

            File::open(&inode_table_file_path).unwrap().read_to_end(&mut ser_inodes).unwrap();
            File::open(&disk_file_path).unwrap().read_to_end(&mut ser_disk).unwrap();

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
            if memory_block_quantity < memory_blocks.len() {
                panic!("O disco existente e maior que o disco atual! Tente inicializar com um disco de tamanho maior!");
            }
        } else {
            File::create(&disk_file_path).expect("Erro criando arquivos para persistencia!");
            File::create(&inode_table_file_path).expect("Erro criando arquivos para persistencia!");

            super_block = Vec::with_capacity(1);
            memory_blocks = Vec::new();

            let ts = time::now().to_timespec();
            let attr = FileAttr {
                ino: 1,
                size: 0,
                blocks: 0,
                atime: ts,
                mtime: ts,
                ctime: ts,
                crtime: ts,
                kind: FileType::Directory,
                perm: 0o755,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            };

            let mut name = ['\0'; 64];
            name[0] = '.';

            let initial_inode = Inode {
                name,
                attributes: attr,
                references: [None; 128]
            };

            super_block.push(Some(initial_inode));
        };

        // Instanciando em branco outras posiçoes possiveis para maior velocidade
        for _ in super_block.len()..max_files {
            let value: Option<Inode> = Option::None;
            super_block.push(value);
        }

        for _ in memory_blocks.len()..memory_block_quantity {
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

    /// Procura pelo vetor `super_block` um espaço de memória vazio (com `None`) e retorna o número `ino` disponível, caso haja algum.
    /// Por convenção, o número de inode `ino` é o número do indíce que ele ocupa no vetor `super_block` + 1.
    pub fn find_ino_available(&self) -> Option<u64> {
        for index in 0..self.super_block.len() - 1 {
            if let Option::None = self.super_block[index] {
                let ino = (index as u64) + 1;
                return Option::Some(ino);
            }
        }

        Option::None
    }

    /// Procura pelo vetor `memory_blocks` um espaço de memória vazio (com `None`) e retorna o índice do bloco, caso haja algum.
    pub fn find_index_of_empty_memory_block(&self) -> Option<usize> {
        for index in 0..self.memory_blocks.len() - 1 {
            if let Option::None = self.memory_blocks[index].data {
                return Option::Some(index);
            }
        }

        Option::None
    }

    /// Procura pelo vetor de `references` de um inode identificado pelo seu número `ino` o primeiro espaço vazio e retorna seu índice.
    pub fn find_index_of_empty_reference_in_inode(&self, ino: u64) -> Option<usize> {
        let index = (ino as usize) - 1;
        match &self.super_block[index] {
            Some(inode) => inode.references.iter().position(|r| r == &None),
            None => panic!("Tentativa inválida de memória")
        }
    }

    /// Salva o `inode` no vetor de `super_block`. Caso o número `ino` de Inode já exista, o dado é sobrescrito.
    pub fn write_inode(&mut self, inode: Inode) {
        if mem::size_of_val(&inode) > self.block_size {
            println!("Não foi possível salvar o inode: tamanho maior que o tamanho do bloco de memória");
            return;
        }

        let index = (inode.attributes.ino - 1) as usize;
        self.super_block[index] = Some(inode);
    }

    pub fn clear_memory_block(&mut self, index: usize) {
        self.memory_blocks[index] = MemoryBlock { data: None };
    }

    pub fn clear_inode(&mut self, ino: u64) {
        let index = (ino - 1) as usize;
        self.super_block[index] = None;
    }

    /// Remove a referência do vetor de references de um Inode
    pub fn clear_reference_in_inode(&mut self, ino: u64, ref_value: usize) {
        let index = (ino - 1) as usize;
        let inode: &mut Option<Inode> = &mut self.super_block[index];
        
        match inode {
            Some(inode) => {
                let reference_index: Option<usize> = inode.references.iter().position(|r| match r {
                    Some(reference) => *reference == ref_value,
                    None => false
                });

                match reference_index {
                    Some(reference_index) => inode.references[reference_index] = None,
                    None => panic!("fn clear_reference_in_inode: Referência não encontrada no Inode.")
                }
            },
            None => panic!("fn clear_reference_in_inode: Tentativa de remoção de referência em um Inode vazio.")
        }
    }

    /// Retorna a referência mutável de memória do `Inode`.
    pub fn get_inode_as_mut(&mut self, ino: u64) -> Option<&mut Inode> {
        let index = (ino as usize) - 1;
        match &mut self.super_block[index] {
            Some(inode) => Some(inode),
            None => None
        }
    }

    /// Retorna o `Inode` especificado pelo seu número `ino`.
    pub fn get_inode(&self, ino: u64) -> Option<&Inode> {
        let index = (ino as usize) - 1;
        match &self.super_block[index] {
            Some(inode) => Some(inode),
            None => None
        }
    }
    
    /// Procura o Inode pelo nome dentro de um vetor de referências do Inode pai.
    pub fn find_inode_in_references_by_name(&self, parent_inode_ino: u64, name: &str) -> Option<&Inode> {
        let index = (parent_inode_ino as usize) - 1;
        let parent_inode = &self.super_block[index];

        match parent_inode {
            Some(parent_inode) => {
                // Procura pelo vetor de references do Inode
                for ino_ref in parent_inode.references.iter() {
                    // Se houver algum dado dentro de ino_ref, então entra no bloco e pega esse conteúdo
                    if let Some(ino) = ino_ref {
                        let index: usize = (ino.clone() as usize) - 1;
                        let inode_ref = &self.super_block[index];

                        match inode_ref {
                            Some(inode) => {
                                let name_from_inode: String = inode.name.iter().collect::<String>();
                                let name_from_inode: &str = name_from_inode.as_str().trim_matches(char::from(0)); // Remoção de caracteres '\0'
                                let name = name.trim();
                                println!("    - lookup(name={:?}, name_from_inode={:?}, equals={})", name, name_from_inode, name_from_inode == name);
                                
                                if name_from_inode == name {
                                    return Some(inode);
                                }
                            },
                            None => panic!("fn get_inode_by_name: Inode reference não encontrado")
                        }
                    }
                }
            },
            None => panic!("fn get_inode_by_name: Inode parent não encontrado")
        }

        return None;
    }

    /// Retorna o vetor de references do Inode
    pub fn get_references_from_inode(&self, ino: u64) -> &[Option<usize>; 128] {
        let index = (ino as usize) - 1;
        match &self.super_block[index] {
            Some(inode) => &inode.references,
            None => panic!("fn get_references_from_inode: Inode não encontrado")
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

    /// Escreve dados em bytes em um bloco de memória
    ///
    ///  # Exemplos
    /// 
    /// ```
    /// let content: Box<[u8]> = Box::from(content.as_bytes());
    /// let disk: Disk = Disk::new(args);
    /// disk.write_content_as_bytes(1, content);
    /// ```
    /// 
    /// Somente é gravado se for um local de memória válido
    pub fn write_content_as_bytes(&mut self, block_index: usize, content: Box<[u8]>) {
        if content.len() > self.block_size {
            panic!("Não foi possível salvar o conteúdo do arquivo, pois excede o tamanho do bloco de memória {}", self.block_size);
        }

        let memory_block = MemoryBlock { data: Some(content) };
        self.memory_blocks[block_index] = memory_block;
    }

    /// Escreve uma referência no vetor de references de um Inode de número ino
    pub fn write_reference_in_inode(&mut self, ino: u64, ref_index: usize, ref_content: usize) {
        let index = (ino as usize) - 1;
        match &mut self.super_block[index] {
            Some(inode) => {
                inode.references[ref_index] = Some(ref_content);
            },
            None => panic!("fn write_reference_in_inode: Inode não encontrado!")
        }
    }

    pub fn write_to_disk(&mut self) {
        match serialize(&self.super_block) {
            Err(e) => {
                print!("Erro ao tentar escrever para arquivo de inodes! {}", e);
                return;
            },
            Ok(v) => {
                let inode_file = format!("{}/.inode.risos", &self.root_path);
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