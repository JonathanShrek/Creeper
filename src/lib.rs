extern crate fs_extra;

use std::path::PathBuf;
use std::fs::metadata;
use fs_extra::dir::copy as dir_copy;
use fs_extra::file::copy as file_copy;

pub fn copy_file(target_dir: &PathBuf, item_path: PathBuf) {
    // Used to determine if path is to a file or directory.
    let md = metadata(&item_path).unwrap();

    // Directory copy options.
    let mut dir_options = fs_extra::dir::CopyOptions::new();
    dir_options.copy_inside = true;

    // File copy options.
    let file_options = fs_extra::file::CopyOptions::new();

    // If directory then copy.
    if md.is_dir() {
        match dir_copy(&item_path, target_dir, &dir_options) {
            Ok(_event) => println!("copied {:?} to {:?}", &item_path, target_dir),
            Err(e) => println!("copy error: {:?}", e)
        }
    }

    // If file then copy.
    if md.is_file() {
        let file_name = item_path.file_name().unwrap();

        // Create the new file path.
        let mut new_file_path = PathBuf::new();
        new_file_path.push(target_dir);
        new_file_path.push(file_name);

        match file_copy(&item_path, new_file_path, &file_options) {
            Ok(_event) => println!("copied {:?} to {:?}", &item_path, target_dir),
            Err(e) => println!("copy error: {:?}", e)
        }
    }
}
