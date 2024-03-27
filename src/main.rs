extern crate notify;
extern crate glob;

mod file_actions;
mod report;

use glob::Pattern;

use std::{
    env,
    path::Path,
    process::ExitCode,
    sync::mpsc::channel
};

use notify::{
    Watcher, 
    RawEvent, 
    raw_watcher,
    RecursiveMode
};

fn main() -> ExitCode {
    // Get the target directory path from the arguments.
    let args: Vec<String> = env::args().collect();

    // Check that args were provided.
    if args.len() < 2 {
        println!("You must provide a watch directory");
        return ExitCode::FAILURE;
    }

    let watch_dir = Path::new(&args[1]).to_path_buf();

    // Check to determine provided paths exist.
    if !watch_dir.exists() {
        println!("Please provide a valid watch directory.");
        return ExitCode::FAILURE;
    }

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_dir, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                // if file created and extension is php
                if op == notify::op::CREATE {
                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_string_lossy();
                        let pattern_str = "*.php*";

                        if Pattern::new(pattern_str).unwrap().matches(&file_name_str) {
                            // send email to notify of a found php file
                            report::send_email(&path);

                            println!("Found php file. Deleting file.");
                            file_actions::delete(path);
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
            _ => println!("An unknown error has occured")
        }
    }
}
