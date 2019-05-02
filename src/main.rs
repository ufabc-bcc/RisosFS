extern crate fuse;
#[macro_use]
extern crate serde_big_array;
mod persistence;
mod serialization;

use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS};
use time::{Timespec, Tm};
use std::env;
use std::mem;
use std::ffi::OsStr;
use std::path::Path;
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

/// Implementação das funções disponíveis na lib `rust-fuse`
impl Filesystem for RisosFS {
    fn lookup(
        &mut self, 
        _req: &Request, 
        _parent: u64, 
        name: &OsStr, 
        reply: ReplyEntry
    ) {
        let file_name = name.to_str().unwrap();
    }

    fn create(
        &mut self, 
        _req: &Request, 
        _parent: u64, 
        name: &OsStr, 
        _mode: u32, 
        flags: u32, 
        reply: ReplyCreate
    ) {
        let inode_index = self.disk.find_index_of_empty_inode().unwrap(); // TODO: necessário tratar
        let memory_block_index = self.disk.find_index_of_empty_memory_block().unwrap(); // TODO: necessário tratar

        let ts = time::now().to_timespec();

        let attr = FileAttr {
            ino: inode_index as u64,
            size: 0,
            blocks: 0,
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
            attributes: attr
        };

        self.disk.write_inode(inode_index, inode);
        self.disk.write_content(memory_block_index, &"");

        reply.created(&ts, &attr, 1, 1, flags)
    }

    fn fsync(
        &mut self, 
        _req: &Request, 
        _ino: u64, 
        _fh: u64, 
        _datasync: bool, 
        reply: ReplyEmpty
    ) { }

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
            None => reply.error(ENOSYS)
        }
    }

    fn mknod(
        &mut self, 
        _req: &Request, 
        _parent: u64, 
        _name: &OsStr, 
        _mode: u32, 
        _rdev: u32, 
        reply: ReplyEntry
    ) { }

    fn open(
        &mut self,
        _req: &Request,
        _ino: u64,
        _flags: u32,
        reply: ReplyOpen
    ) { }

    fn read(
        &mut self, 
        _req: &Request, 
        _ino: u64, 
        _fh: u64, 
        _offset: i64, 
        _size: u32, 
        reply: ReplyData
    ) { println!("fn read"); }

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
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
            }
            reply.ok();
        } else {
            reply.error(ENOSYS);
        }
    }

    fn write(
        &mut self, 
        _req: &Request, 
        ino: u64, 
        _fh: u64, 
        _offset: i64, 
        data: &[u8], 
        _flags: u32, 
        reply: ReplyWrite
    ) {
    }

    fn destroy(&mut self, req: &Request) {
        self.disk.write_to_disk();
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
    fuse::mount(fs, &mountpoint, &options).unwrap();
}