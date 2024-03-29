extern crate notify;

mod file_actions;

use std::{
    env,
    path::Path,
    process::ExitCode,
    sync::mpsc::channel,
    time::Duration,
    thread
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
                    println!("Copying {:?} to {:?}", path, &target_dir);

                    // sleep to allow time for the item to be completely moved after downloading
                    thread::sleep(Duration::from_secs(10));

                    file_actions::copy(&target_dir, path);
                }
            },
            // Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
            _ => println!("An unknown error has occured")
        }
    }
}
