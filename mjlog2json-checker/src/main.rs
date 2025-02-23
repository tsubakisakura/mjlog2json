//! # mjlog2json-checker
//!
//! Verify that mjlog2json conversion matches the official xml and json.
//!
//! # Usage
//!
//! 1. Download official xml and json to same folder.
//! 2. Run ```cargo run --release -p mjlog2json-checker async <<folder_name>>```
//! 3. Check the difference between ```actual.txt``` and ```expected.txt``` using a diff tool.

mod checker;

use crate::checker::*;
use std::env;
use std::path::Path;

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone(); // "sync" or "async"
    let target_dir = if args.len() < 3 { Path::new("data") } else { Path::new(&args[2]) };
    let glob_pattern = target_dir.join("*.xml");

    match command.as_str() {
        "sync" => sync_check_glob(&glob_pattern.to_string_lossy()),
        "async" => async_check_glob(&glob_pattern.to_string_lossy()).await,
        _ => println!("command: sync | async"),
    }
}
