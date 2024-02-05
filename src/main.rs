extern crate notify;

use std::{
    env,
    thread,
    fs::metadata,
    process::ExitCode,
    sync::mpsc::channel,
    path::{Path, PathBuf}
};

use notify::{
    Watcher, 
    RawEvent, 
    raw_watcher,
    RecursiveMode
};

use fs_extra::{
    dir::copy as dir_copy,
    file::copy as file_copy
};

fn copy(target_dir: &PathBuf, item_path: PathBuf) {
    // Used to determine if path is to a file or directory.
    let md = metadata(&item_path).unwrap();

    let target_dir_clone = target_dir.clone();
    let item_path_clone = item_path.clone();

    // If directory then copy.
    if md.is_dir() {
        // Spawn a new thread to copy the directory
        let handler = thread::spawn(move || {
            let mut dir_options = fs_extra::dir::CopyOptions::new();
            dir_options.copy_inside = true;

            match dir_copy(&item_path_clone, &target_dir_clone, &dir_options) {
                Ok(_event) => println!("copied directory {:?} to {:?}", &item_path_clone, target_dir_clone),
                Err(e) => println!("copy directory error: {:?}", e)
            }
        });

        // Wait for the thread to finish and check if it was successful or errored
        match handler.join() {
            Ok(_) => println!("Thread successfully completed"),
            Err(e) => println!("Thread errored: {:?}", e),
        }
    } else if md.is_file() {
        // Spawn a new thread to copy the file
        let handler = thread::spawn(move || {
            let file_name = item_path_clone.file_name().unwrap_or_default();
            let mut new_file_path = PathBuf::new();
            new_file_path.push(&target_dir_clone);
            new_file_path.push(file_name);

            let file_options = fs_extra::file::CopyOptions::new();

            match file_copy(&item_path_clone, &new_file_path, &file_options) {
                Ok(_event) => println!("copied file {:?} to {:?}", &item_path_clone, target_dir_clone),
                Err(e) => println!("copy file error: {:?}", e)
            }
        });

        // Wait for the thread to finish and check if it was successful or errored
        match handler.join() {
            Ok(_) => println!("Thread successfully completed"),
            Err(e) => println!("Thread errored: {:?}", e),
        }
    }
}

fn main() -> ExitCode {
    // Get the target directory path from the arguments.
    let args: Vec<String> = env::args().collect();

    // Check that args were provided.
    if args.len() < 3 {
        println!("You must provide a watch directory and target directory.");
        return ExitCode::FAILURE;
    }

    let watch_dir = Path::new(&args[1]).to_path_buf();
    let target_dir = Path::new(&args[2]).to_path_buf();

    // Check to determine provided paths exist.
    if !watch_dir.exists() {
        println!("Please provide a valid watch directory.");
        return ExitCode::FAILURE;
    }

    if !target_dir.exists() {
        println!("Please provide a valid target directory");
        return ExitCode::FAILURE;
    }

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_dir, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                // Only copy file if we have a create op code.
                if op == notify::op::CREATE {
                    copy(&target_dir, path);
                }
            },
            // Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
            _ => println!("An unknown error has occured")
        }
    }
}
