extern crate fuse;

use fuse::{Filesystem};
use std::env;

struct RisosFS {
    disk: Vec<u8>
}

impl RisosFS {
    fn new() -> RisosFS {
        const MEMORY_SIZE: usize = 1024 * 1024 * 1024;
        let disk: Vec<u8> = Vec::with_capacity(MEMORY_SIZE);

        RisosFS {
            disk: disk
        }
    }
}

impl Filesystem for RisosFS {
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