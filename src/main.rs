extern crate fuse;
mod persistence;

use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT};
use time::Timespec;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use crate::persistence::{Disk, Inode};

struct RisosFS {
    disk: Disk
}

impl RisosFS {
    /// Inicializa o FS com o tamanho especificado em `memory_size` com blocos de memória de tamanho
    /// `block_size`.
    fn new() -> RisosFS {
        let memory_size: usize = 1024 * 1024 * 10;
        let block_size: usize = 1024;

        RisosFS {
            disk: Disk::new(memory_size, block_size)
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

        // Procura pelo `path` do arquivo na tabela de inode
        let hashmap_item = self.disk.get_inode_table().iter()
            .find(|(_, inode)| inode.path == file_name);

        match hashmap_item {
            // Se houver um item com o `path`, então ele dá um `reply` com os atributos do arquivo
            Some ((_, inode)) => {
                let ttl = Timespec::new(1, 0);
                reply.entry(&ttl, &inode.attributes, 0);
            }
            // Caso não seja encontrado, retorna um código de erro
            None => reply.error(ENOSYS)
        }
    }

    fn create(
        &mut self, 
        _req: &Request, 
        _parent: u64, 
        _name: &OsStr, 
        _mode: u32, 
        _flags: u32, 
        reply: ReplyCreate
    ) { }

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
        let ts = Timespec::new(0, 0);

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

        let ttl = Timespec::new(1, 0);
        if ino == 1 {
            reply.attr(&ttl, &attr);
        } else {
            reply.error(ENOSYS);
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
        _fh: u64, 
        offset: i64, 
        mut reply: ReplyDirectory
    ) {
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
            }
            reply.ok();
        } else {
            reply.error(ENOSYS);
        }
        
        // TODO: pesquisa de arquivos na tabela
    }

    fn write(
        &mut self, 
        _req: &Request, 
        _ino: u64, 
        _fh: u64, 
        _offset: i64, 
        _data: &[u8], 
        _flags: u32, 
        reply: ReplyWrite
    ) { }

    // fn truncate
    // fn utimens
}

fn main() {
    let fs = RisosFS::new();

    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };

    println!("RisosFS started!");
    fuse::mount(fs, &mountpoint, &[]).unwrap();
}