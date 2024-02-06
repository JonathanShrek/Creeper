extern crate fs_extra;

use std::{
    path::PathBuf,
    fs::remove_file,
};

pub fn delete(item_path: PathBuf) {
    match remove_file(item_path) {
        Ok(()) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {:?}", e),
    }
}