use futures::stream::{FuturesOrdered, StreamExt};
use glob::glob;
use mjlog::parser::*;
use mjlog2json_core::conv::*;
use serde_json::{to_string_pretty, Value};
use std::path::{Path, PathBuf};
use tenhou_json::exporter::*;
use tenhou_json::model::*;
use tenhou_json::parser::*;

fn replace_extension(x: &Path) -> PathBuf {
    let mut r = x.to_path_buf();
    r.set_extension("json");
    r
}

enum TaskResult {
    Same,
    Diff(String, String),
}

fn to_string_pretty_from_str(s: &str) -> String {
    let value: Value = serde_json::from_str(s).unwrap();
    to_string_pretty(&value).unwrap()
}

fn verify(content_xml: String, content_json: String) -> TaskResult {
    let mjlog = &parse_mjlogs(&content_xml).unwrap()[0];
    let tenhou_json = parse_tenhou_json(&content_json).unwrap();
    let converted_tenhou_json = TenhouJson {
        reference: tenhou_json.reference.clone(), // same as filebase
        ..conv_to_tenhou_json(mjlog).unwrap()
    };

    if tenhou_json != converted_tenhou_json {
        return TaskResult::Diff(std::format!("{:#?}", tenhou_json), std::format!("{:#?}", converted_tenhou_json));
    }

    let exported_json = export_tenhou_json(&converted_tenhou_json).unwrap();
    if content_json != exported_json {
        return TaskResult::Diff(to_string_pretty_from_str(&content_json), to_string_pretty_from_str(&exported_json));
    }

    TaskResult::Same
}

fn sync_check_xml(path_xml: PathBuf) -> (PathBuf, TaskResult) {
    let content_xml = std::fs::read_to_string(&path_xml).unwrap();
    let content_json = std::fs::read_to_string(replace_extension(&path_xml)).unwrap();

    (path_xml, verify(content_xml, content_json))
}

pub fn sync_check_glob(pattern: &str) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        let path_xml = entry.unwrap();

        // print log before check in sync mode
        println!("{}", path_xml.to_string_lossy().into_owned());

        match sync_check_xml(path_xml) {
            (_, TaskResult::Same) => {}
            (path_xml, TaskResult::Diff(expected, actual)) => {
                println!("detect difference: {}", path_xml.to_string_lossy());
                std::fs::write("expected.txt", expected).unwrap();
                std::fs::write("actual.txt", actual).unwrap();
                return;
            }
        }
    }

    // succeeded all test
    std::fs::write("expected.txt", "SUCCESS!").unwrap();
    std::fs::write("actual.txt", "SUCCESS!").unwrap();
}

async fn async_check_xml(path_xml: PathBuf) -> (PathBuf, TaskResult) {
    let content_xml = async_std::fs::read_to_string(&path_xml).await.unwrap();
    let content_json = async_std::fs::read_to_string(replace_extension(&path_xml)).await.unwrap();

    (path_xml, verify(content_xml, content_json))
}

pub async fn async_check_glob(pattern: &str) {
    let mut tasks = FuturesOrdered::new();

    println!("Registering tasks...");
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        tasks.push_back(async_std::task::spawn(async_check_xml(entry.unwrap())));
    }

    while let Some(ret) = tasks.next().await {
        match ret {
            (path_xml, TaskResult::Same) => {
                // print log after check in async mode
                println!("{}", path_xml.to_string_lossy().into_owned());
            }
            (path_xml, TaskResult::Diff(expected, actual)) => {
                println!("detect difference: {}", path_xml.to_string_lossy());
                std::fs::write("expected.txt", expected).unwrap();
                std::fs::write("actual.txt", actual).unwrap();
                return;
            }
        }
    }

    // succeeded all test
    std::fs::write("expected.txt", "SUCCESS!").unwrap();
    std::fs::write("actual.txt", "SUCCESS!").unwrap();
}
