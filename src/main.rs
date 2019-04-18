extern crate fuse;
mod persistence;

use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite};
use std::env;
use std::ffi::OsStr;
use std::str;
use crate::persistence::Disk;

struct RisosFS {
    disk: Disk
}

impl RisosFS {
    fn new() -> RisosFS {
        let memory_size: usize = 1024 * 1024 * 1024;
        let block_size: usize = 1024;

        let mut disk = Disk::new(memory_size, block_size);

        let content = "Teste teste testE";
        disk.write_content(3usize, &content);
        
        let block = disk.get_content_from_block(3usize);
        let test = str::from_utf8(block);

        println!("{}", test.unwrap());

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

    //fuse::mount(fs, &mountpoint, &[]);
}