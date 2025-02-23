//! # mjlog2json
//!
//! Convert mjlog-XML to tenhou-JSON.
//!
//! # Usage
//!
//! ```
//! mjlog2json 2025010203gm-0000-0000-01234567.xml
//! mjlog2json 2025010203gm-0000-0000-01234567.xml -o 2025010203gm-0000-0000-01234567.json
//! mjlog2json input_dir
//! mjlog2json input_dir -o output_dir
//! ```
//!
//! # Install
//!
//! ```
//! cargo install mjlog2json
//! ```

mod converter;

use crate::converter::*;
use argh::FromArgs;
use std::error::Error;
use std::path::PathBuf;

/// Convert mjlog-XML to tenhou-JSON.
#[derive(FromArgs, Debug)]
struct Args {
    /// input XML file or directory.
    #[argh(positional)]
    input: String,

    /// output JSON file or directory.
    #[argh(option, short = 'o')]
    output: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Args = argh::from_env();
    let input_path = PathBuf::from(args.input.clone());

    if input_path.is_file() {
        // file conversion mode
        let s = read_mjlog(&input_path)?;
        if let Some(x) = args.output {
            std::fs::write(x, s)?;
            Ok(())
        } else {
            println!("{}", s);
            Ok(())
        }
    } else if input_path.is_dir() {
        // directory conversion mode
        let output_path = if let Some(x) = args.output { PathBuf::from(x) } else { input_path.clone() };
        async_conv_dir(&input_path, &output_path).await
    } else {
        // file does not exist
        Err(format!("{} does not exist.", args.input).into())
    }
}
