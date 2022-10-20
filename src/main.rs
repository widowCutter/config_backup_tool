#![allow(deprecated)]
use std::env::home_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file() {
    let path = Path::new("./input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read{}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    };
}

enum FileType {
    Directory = 32,
    File,
}
struct OneFile {
    name: String,
    file_type: FileType,
}
fn read_directory(path: &Path) {
    let dir_list = fs::read_dir(path).unwrap();
    for dir in dir_list {
        let dir = dir.unwrap();
        println!(
            "Filename:{}metadata:[IsDir:{}, Size(B):{}], path:{}",
            dir.file_name().to_string_lossy(),
            dir.metadata().unwrap().is_dir().to_string(),
            dir.metadata().unwrap().len(),
            dir.path().display()
        );
    }
}

fn main() {
    //deprecated function home_dir() should work on linux so its okey i guess :)
    let home_path = match home_dir() {
        Some(x) => x,
        None => {
            panic!("Couldn't get path to home directory")
        }
    };
    println!("{}", home_path.display());
    let path_s = format!("{}/.config/nvim/", home_path.display());
    let path = Path::new(&path_s);
    read_directory(path);
}
