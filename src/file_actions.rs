extern crate fs_extra;

use crossbeam::channel::unbounded;

use std::{
    thread,
    fs::metadata,
    path::PathBuf
};

use fs_extra::{
    dir::copy as dir_copy,
    file::copy as file_copy
};

pub fn copy(target_dir: &PathBuf, item_path: PathBuf) {
    // Used to determine if path is to a file or directory.
    let md = metadata(&item_path).unwrap();

    let target_dir_clone = target_dir.clone();
    let item_path_clone = item_path.clone();

    // Set up a channel for communication between threads
    let (sender, receiver) = unbounded();

    // If directory then copy.
    if md.is_dir() {
        let mut dir_options = fs_extra::dir::CopyOptions::new();
        dir_options.copy_inside = true;

        thread::spawn(move || {
            match dir_copy(&item_path_clone, &target_dir_clone, &dir_options) {
                Ok(_event) => {
                    println!("copying directory {:?} to {:?}", &item_path_clone, target_dir_clone);
                    sender.send(()).unwrap();
                },
                Err(e) => println!("copy directory error: {:?}", e)
            }
        });

        // Wait for the thread to finish and check if it was successful or errored
        match receiver.recv() {
            Ok(_) => println!("Thread successfully completed"),
            Err(e) => println!("Thread errored: {:?}", e),
        };
    } else if md.is_file() {
        // Spawn a new thread to copy the file
        thread::spawn(move || {
            let file_name = item_path_clone.file_name().unwrap_or_default();
            let mut new_file_path = PathBuf::new();
            new_file_path.push(&target_dir_clone);
            new_file_path.push(file_name);

            let file_options = fs_extra::file::CopyOptions::new();

            match file_copy(&item_path_clone, &new_file_path, &file_options) {
                Ok(_event) => {
                    println!("copying file {:?} to {:?}", &item_path_clone, target_dir_clone);
                    sender.send(()).unwrap();
                },
                Err(e) => println!("copy file error: {:?}", e)
            }
        });

        // Wait for the thread to finish and check if it was successful or errored
        match receiver.recv() {
            Ok(_) => println!("Thread successfully completed"),
            Err(e) => println!("Thread errored: {:?}", e),
        }
    }
}