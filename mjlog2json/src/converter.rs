use futures::stream::{FuturesUnordered, StreamExt};
use glob::glob;
use mjlog::parser::*;
use mjlog2json_core::conv::*;
use std::error::Error;
use std::path::{Path, PathBuf};
use tenhou_json::exporter::*;
use tenhou_json::model::*;

fn read_contents(input_path: &Path, content_xml: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mjlog = &parse_mjlogs(&content_xml)?[0];
    let reference = input_path.file_stem().unwrap().to_string_lossy().to_string();
    let converted_tenhou_json = TenhouJson { reference, ..conv_to_tenhou_json(mjlog)? };

    Ok(export_tenhou_json(&converted_tenhou_json)?)
}

pub fn read_mjlog(input_path: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
    let content_xml = std::fs::read_to_string(input_path)?;
    read_contents(input_path, content_xml)
}

async fn async_conv_file(input_path: PathBuf, output_dir: PathBuf) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let file_stem: &str = input_path.file_stem().unwrap().to_str().unwrap();
    let output_path = output_dir.join(format!("{}.json", file_stem));

    let content_xml = async_std::fs::read_to_string(&input_path).await?;
    let content_json = read_contents(&input_path, content_xml)?;

    async_std::fs::write(output_path, &content_json).await?;
    Ok(input_path)
}

pub async fn async_conv_dir(input_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
    std::fs::create_dir_all(output_dir)?;

    let pattern_binding = input_dir.join("*.xml");
    let pattern = pattern_binding.to_string_lossy();

    let mut tasks = FuturesUnordered::new();

    println!("Registering tasks...");
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        tasks.push(async_std::task::spawn(async_conv_file(entry.unwrap().to_path_buf(), output_dir.to_path_buf())));
    }

    while let Some(ret) = tasks.next().await {
        match ret {
            Ok(x) => println!("{}", x.to_string_lossy().into_owned()),
            Err(x) => return Err(x),
        }
    }
    Ok(())
}
