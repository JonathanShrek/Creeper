extern crate notify;
extern crate fs_extra;

use std::process::ExitCode;
use std::sync::mpsc::channel;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs::metadata;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use fs_extra::dir::copy as dir_copy;
use fs_extra::file::copy as file_copy;

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
        println!("Please provide a valid target directory.");
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
                if op == notify::op::CREATE
                {
                    copy_file(&target_dir, path);
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

///
fn copy_file(target_dir: &PathBuf, item_path: PathBuf) {
    // Used to determine if path is to a file or directory.
    let md = metadata(&item_path).unwrap();

    // Directory copy options.
    let mut dir_options = fs_extra::dir::CopyOptions::new();
    dir_options.copy_inside = true;

    // File copy options.
    let file_options = fs_extra::file::CopyOptions::new();

    // If directory then copy.
    if md.is_dir()
    {
        match dir_copy(&item_path, target_dir, &dir_options) {
            Ok(_event) => println!("copied {:?} to {:?}", &item_path, target_dir),
            Err(e) => println!("copy error: {:?}", e)
        }
    }

    // If file then copy.
    if md.is_file()
    {
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
