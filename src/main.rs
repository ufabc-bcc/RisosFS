extern crate fuse;

use fuse::Filesystem;
use std::env;

struct RisosFS;

impl Filesystem for RisosFS {
}

fn main() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(RisosFS, &mountpoint, &[]);
}