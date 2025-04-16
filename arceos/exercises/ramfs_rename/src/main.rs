#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

mod ramfs;

use std::io::{self, prelude::*};
use std::fs::{self, File};

fn create_file(fname: &str, text: &str) -> io::Result<()> {
    println!("Create '{}' and write [{}] ...", fname, text);
    let mut file = File::create(fname)?;
    file.write_all(text.as_bytes())
}

fn create_dir(dname: &str) -> io::Result<()> {
    println!("Create '{}' ...", dname);
    fs::create_dir(dname)
}

// Only support rename, NOT move.
fn rename_file(src: &str, dst: &str) -> io::Result<()> {
    println!("Rename '{}' to '{}' ...", src, dst);
    fs::rename(src, dst)
}

fn print_file(fname: &str) -> io::Result<()> {
    println!("Print '{}' content ...", fname);
    let mut buf = [0; 1024];
    let mut file = File::open(fname)?;
    loop {
        let n = file.read(&mut buf)?;
        if n > 0 {
            print!("Read '{}' content: [", fname);
            io::stdout().write_all(&buf[..n])?;
            println!("] ok!");
        } else {
            return Ok(());
        }
    }
}

fn process() -> io::Result<()> {
    create_dir("/tmp/f1")?;
    create_file("/tmp/f1/f2", "hello")?;
    // Just rename, NOT move.
    // So this must happen in the same directory.
    rename_file("/tmp/f1/f2", "/tmp/f1/f3")?;
    print_file("/tmp/f1/f3")
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    if let Err(e) = process() {
        panic!("Error: {}", e);
    }
    println!("\n[Ramfs-Rename]: ok!");
}
