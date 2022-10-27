#![allow(deprecated)]
#![allow(unused)]
use std::borrow::Borrow;
use std::env::home_dir;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::prelude::MetadataExt;
use std::path::{Path, PathBuf};

use file::hello_world;

mod file;

fn red_file() {
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

#[derive(Clone)]
struct FileStruct {
    name: OsString,
    is_dir: bool,
    #[allow(unused)]
    file_type: std::fs::FileType,
    size: u64,
    full_path: PathBuf,
    #[allow(unused)]
    content: Vec<FileStruct>,
}
fn read_file(path: &Path, recursive: bool) -> FileStruct {
    let file_read = fs::metadata(path).unwrap();
    let file = FileStruct {
        name: OsString::from("./"),
        is_dir: true,
        file_type: file_read.file_type(),
        size: file_read.size(),
        full_path: path.to_owned(),
        content: Vec::new(),
    };
    if file.is_dir == true {
        //println!("True");
    }
    file
}
//recursive calling of indexing directory
fn read_directory(path: &Path) -> Vec<FileStruct> {
    let mut all_files: Vec<FileStruct> = Vec::new();
    let dir_list = fs::read_dir(path).unwrap();
    for dir in dir_list {
        let dir = dir.unwrap();
        let file: FileStruct = FileStruct {
            name: dir.file_name(),
            is_dir: dir.metadata().unwrap().is_dir(),
            file_type: dir.metadata().unwrap().file_type(),
            size: dir.metadata().unwrap().len(),
            full_path: dir.path(),
            content: Vec::new(),
        };
       // println!(
       //     "Filename:{}    metadata:[IsDir:{}, Size:{}B], path:{}",
       //     file.name.to_str().unwrap(),
       //     file.is_dir,
       //     file.size,
       //     file.full_path.display(),
       // );
        all_files.push(file);
    }
    let mut output_files: Vec<FileStruct> = Vec::new();
    for mut file in all_files
    {
        //println!("Is file{} dir?{}", file.full_path.display(), file.is_dir);
        if file.is_dir {
            file.content = read_directory(file.full_path.borrow());
        }
        output_files.push(file);
    }
    output_files
}

fn call_index_file() -> FileStruct {
    //deprecated function home_dir() should work on linux so it's okey i guess :)
    let home_path = match home_dir() {
        Some(x) => x,
        None => {
            panic!("Couldn't get path to home directory")
        }
    };
    //println!("{}", home_path.display());
    let path_s = format!("{}/.config/nvim/", home_path.display());
    let path = Path::new(&path_s);
    let root_directory = read_directory(path);
    root_directory[0].clone()
}
fn show_content(dir: FileStruct)
{
    let mut i = 0;
    for file in dir.content {
        println!("{}:{}", i,file.name.to_string_lossy());
        i += 1;
    }
}
fn main () {
    let file = call_index_file();
    show_content(file.clone());
    hello_world();
}
