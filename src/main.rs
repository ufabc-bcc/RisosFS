extern crate fuse;
#[macro_use]
extern crate serde_big_array;
mod persistence;
mod serialization;

use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
<<<<<<< Updated upstream
use libc::{ENOSYS, ENOENT, EIO, EISDIR};
=======
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
>>>>>>> Stashed changes
use time::{Timespec};
use std::env;
use std::mem;
use std::ffi::OsStr;
use crate::persistence::{Disk, Inode};

struct RisosFS {
    disk: Disk
}

impl RisosFS {
    /// Inicializa o FS com o tamanho especificado em `memory_size` com blocos de memória de tamanho
    /// `block_size`.
    fn new(root_path: String) -> Self {
        let max_files: usize = 1024;
        let memory_size: usize = 1024 * 1024 * 1024;
        let block_size: usize = max_files * (mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>());

        let disk = Disk::new(root_path, memory_size, block_size);

        RisosFS {
            disk
        }
    }
}

impl Drop for RisosFS {
    fn drop(&mut self) {
        println!("cleanup");
        &self.disk.write_to_disk();
    }
}

/// Implementação das funções disponíveis na lib `rust-fuse`
impl Filesystem for RisosFS {
    fn lookup(
        &mut self, 
        _req: &Request, 
        parent: u64, 
        name: &OsStr, 
        reply: ReplyEntry
    ) {
        println!("lookup(name={:?})", name);
        let file_name = name.to_str().unwrap();
        let inode = self.disk.get_inode_by_name(file_name);

<<<<<<< Updated upstream
=======
        let inode = self.disk.get_inode_by_name(parent as usize, file_name);

>>>>>>> Stashed changes
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                println!("        - lookup(attr={:?})", inode.attributes);
                reply.entry(&ttl, &inode.attributes, 0)
            },
            None => reply.error(ENOENT)
        }
    }

    fn create(
        &mut self, 
        _req: &Request, 
<<<<<<< Updated upstream
        _parent: u64, 
=======
        parent: u64, 
>>>>>>> Stashed changes
        name: &OsStr, 
        mode: u32, 
        flags: u32, 
        reply: ReplyCreate
    ) {
        println!("create(name={:?}, mode={}, flags={})", name, mode, flags);
<<<<<<< Updated upstream
=======

        let ref_index = self.disk.find_empty_reference(parent as usize);
        // Se não houver mais espaço no vetor de references, indica que não é possível alocar mais arquivos dentro da pasta
        if ref_index == None {
            println!("Não é possível criar mais arquivos nesse diretório!");
            reply.error(EIO);
            return;
        }

>>>>>>> Stashed changes
        let inode_index = self.disk.find_index_of_empty_inode().unwrap(); // TODO: necessário tratar
        let memory_block_index = self.disk.find_index_of_empty_memory_block().unwrap(); // TODO: necessário tratar

        let ts = time::now().to_timespec();

        let attr = FileAttr {
            ino: inode_index as u64,
            size: 0,
            blocks: 1,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::RegularFile,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags,
        };
        
        let name = name.to_str().unwrap();
        let name: Vec<char> = name.chars().collect();

        let mut name_char = ['\0'; 64];
        name_char[..name.len()].clone_from_slice(&name);

        let inode = Inode {
            name: name_char,
<<<<<<< Updated upstream
            attributes: attr
=======
            attributes: attr,
            references: [None; 128]
>>>>>>> Stashed changes
        };

        self.disk.write_inode(inode_index, inode);
        self.disk.write_content(memory_block_index, &"");

<<<<<<< Updated upstream
=======
        // Adiciona a referência de inode criado no vetor references do inode "pai" (do diretório)
        let ref_index = ref_index.unwrap();
        let parent_inode = self.disk.get_inode(parent as usize).unwrap();
        parent_inode.references[ref_index] = Some(inode_index);

>>>>>>> Stashed changes
        reply.created(&ts, &attr, 1, inode_index as u64, flags)
    }

    fn fsync(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        fh: u64, 
        datasync: bool, 
        reply: ReplyEmpty
    ) { 
        println!("fsync(ino={}, fh={}, datasync={})", ino, fh, datasync);
        reply.error(ENOSYS);
    }

    fn setattr(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        _mode: Option<u32>, 
        uid: Option<u32>, 
        gid: Option<u32>, 
        size: Option<u64>, 
        atime: Option<Timespec>, 
        mtime: Option<Timespec>, 
        _fh: Option<u64>, 
        crtime: Option<Timespec>, 
        _chgtime: Option<Timespec>, 
        _bkuptime: Option<Timespec>, 
        flags: Option<u32>, 
        reply: ReplyAttr
    ) {
        println!("setattr(ino={})", ino);
        let inode = self.disk.get_inode(ino as usize);
        
        match inode {
            Some(inode) => {
                if let Some(size) = size { inode.attributes.size = size; }
                if let Some(atime) = atime { inode.attributes.atime = atime; }
                if let Some(mtime) = mtime { inode.attributes.mtime = mtime; }
                if let Some(crtime) = crtime { inode.attributes.crtime = crtime; }
                if let Some(gid) = gid { inode.attributes.gid = gid; }
                if let Some(uid) = uid { inode.attributes.uid = uid; }
                if let Some(flags) = flags { inode.attributes.flags = flags; }

                let ttl = time::now().to_timespec();

                reply.attr(&ttl, &inode.attributes)
            },
            None => reply.error(ENOENT)
        }
    }

    fn getattr(
        &mut self,
        _req: &Request,
        ino: u64,
        reply: ReplyAttr
    ) {
        println!("getattr(ino={})", ino);

        match self.disk.get_inode(ino as usize) {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                reply.attr(&ttl, &inode.attributes);
            },
            None => reply.error(ENOENT)
<<<<<<< Updated upstream
=======
        }
    }

    fn mkdir(
        &mut self, 
        _req: &Request, 
        parent: u64, 
        name: &OsStr, 
        mode: u32, 
        reply: ReplyEntry
    ) {
        let reference_index = self.disk.find_empty_reference(parent as usize);
        
        match reference_index {
            Some(reference_index) => {

                let ino = self.disk.find_index_of_empty_inode();
                match ino {
                    Some(ino) => {
                        let ts = time::now().to_timespec();
                        let attr = FileAttr {
                            ino: ino as u64,
                            size: 0,
                            blocks: 1,
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

                        let name = name.to_str().unwrap();
                        let name: Vec<char> = name.chars().collect();

                        let mut name_char = ['\0'; 64];
                        name_char[..name.len()].clone_from_slice(&name);

                        let inode = Inode {
                            name: name_char,
                            attributes: attr,
                            references: [None; 128]
                        };

                        self.disk.write_inode(ino, inode);
                        self.disk.write_reference_in_inode(parent as usize, reference_index, ino);

                        reply.entry(&ts, &attr, 0);
                    },
                    None => reply.error(ENOSPC) // “No space left on device.”
                }
            },
            None => { println!("Limite de arquivos dentro da pasta atingido!"); reply.error(EIO); }
>>>>>>> Stashed changes
        }
    }

    // TODO: implementar mknod
    fn mknod(
        &mut self, 
        _req: &Request, 
        _parent: u64, 
        name: &OsStr, 
        mode: u32, 
        rdev: u32, 
        reply: ReplyEntry
    ) { 
<<<<<<< Updated upstream
        println!("mknod(name={:?}, mode={}, rdev={})", name, mode, rdev);
=======
        println!("mknod(name={:?}, mode={}, rdev={}) -> NOT IMPLEMENTED", name, mode, rdev);
>>>>>>> Stashed changes
        reply.error(ENOSYS);
    }

    fn open(
        &mut self,
        _req: &Request,
        ino: u64,
        flags: u32,
        reply: ReplyOpen
    ) {
        println!("open(ino={}, flags={})", ino, flags);

        let inode = self.disk.get_inode(ino as usize);

        match inode {
            Some(_) => reply.opened(ino, flags),
            None => reply.error(ENOSYS)
        }
    }

    fn read(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        fh: u64, 
        offset: i64, 
        size: u32, 
        reply: ReplyData
    ) {
        println!("read(ino={}, fh={}, offset={}, size={})", ino, fh, offset, size);

        let memory_block = self.disk.get_content_as_bytes(ino as usize);
        
        match memory_block {
            Some(memory_block) => reply.data(memory_block),
            None => reply.error(EIO)
        }
    }

    fn readdir(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        fh: u64, 
        offset: i64, 
        mut reply: ReplyDirectory
    ) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);

        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, ".");
<<<<<<< Updated upstream
                reply.add(1, 1, FileType::Directory, "..");

                for inode in self.disk.get_inode_table().iter() {
                    if let Some(inode_data) = inode {
                        if inode_data.attributes.ino != 1 {
                            let name = inode_data.name.iter().collect::<String>();
                            let offset = ino as i64;
                            println!("    - readdir(ino={}, name={})", inode_data.attributes.ino, name);
                            reply.add(inode_data.attributes.ino, offset, inode_data.attributes.kind, name);
                        }
                    }
                }
            }

            reply.ok();

        } else {
            reply.error(ENOENT);
=======
                reply.add(1, 0, FileType::Directory, "..");
            }
        }

        let inodes = self.disk.get_inode_table();
        let inode = &inodes[ino as usize];

        if mem::size_of_val(inode) == offset as usize {
            reply.ok();
            return;
        }

        match inode {
            Some(inode) => {
                let references = inode.references;
                for ino in references.iter() {

                    if let Some(ino) = ino {
                        let inode = &inodes[*ino];

                        if let Some(inode_data) = inode {
                            if inode_data.attributes.ino == 1 {
                                continue;
                            }

                            let name = inode_data.name.iter().collect::<String>();
                            println!("    - readdir(ino={}, name={})", inode_data.attributes.ino, name);
                            let offset = mem::size_of_val(inode) as i64;
                            reply.add(inode_data.attributes.ino, offset, inode_data.attributes.kind, name);
                        }

                    }
                }

                reply.ok()
            },
            None => { println!("ERROR ino={:?}", ino); reply.error(ENOENT) }
>>>>>>> Stashed changes
        }
    }

    fn write(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        _fh: u64, 
        offset: i64, 
        data: &[u8], 
        _flags: u32, 
        reply: ReplyWrite
    ) {
        println!("write(ino={}, offset={}, data={})", ino, offset, data.len());
        let inode = self.disk.get_inode(ino as usize);
        let content: Box<[u8]> = data.to_vec().into_boxed_slice();

        match inode {
            Some(inode) => {
                inode.attributes.size = data.len() as u64;
                self.disk.write_content_as_bytes(ino as usize, content);
                reply.written(data.len() as u32);
            },
            None => {
                println!("Inode não foi encontrado");
                reply.error(ENOENT);
            }
        }
    }

    fn unlink(
        &mut self, 
        _req: &Request, 
<<<<<<< Updated upstream
        _parent: u64, 
=======
        parent: u64, 
>>>>>>> Stashed changes
        name: &OsStr, 
        reply: ReplyEmpty
    ) {
        let name = name.to_str().unwrap();
<<<<<<< Updated upstream
        let inode = self.disk.get_inode_by_name(name);
=======
        let inode = self.disk.get_inode_by_name(parent as usize, name);
>>>>>>> Stashed changes

        match inode {
            Some(inode) => {
                if inode.attributes.kind == FileType::Directory {
                    reply.error(EISDIR);
                } else {
                    let ino = inode.attributes.ino as usize;
                    self.disk.clear_inode(ino);
                    self.disk.clear_memory_block(ino);
                    reply.ok()
                }
            },
            None => reply.error(EIO)
        }
    }

    // fn truncate
    // fn utimens
}

fn main() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };

    let fs = RisosFS::new(mountpoint.clone());

    let options = ["-o", "nonempty"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    println!("RisosFS started!");
<<<<<<< Updated upstream
=======

>>>>>>> Stashed changes
    fuse::mount(fs, &mountpoint, &options).unwrap();
}