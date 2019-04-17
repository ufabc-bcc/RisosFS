extern crate fuse;

use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite};
use std::env;
use std::ffi::OsStr;

struct RisosFS {
    disk: Box<[u8]>
}

impl RisosFS {
    fn new() -> RisosFS {
        const MEMORY_SIZE: usize = 1024 * 1024 * 1024;
        let disk: Vec<u8> = vec![0; MEMORY_SIZE];
        let disk = disk.into_boxed_slice();

        RisosFS {
            disk: disk
        }
    }
}

impl Filesystem for RisosFS {
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
        _ino: u64,
        reply: ReplyAttr
    ) { }

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
    ) { }

    fn readdir(
        &mut self, 
        _req: &Request, 
        _ino: u64, 
        _fh: u64, 
        _offset: i64, 
        reply: ReplyDirectory
    ) { }

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
    fuse::mount(fs, &mountpoint, &[]);
}