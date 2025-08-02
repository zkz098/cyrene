mod xlsx;
mod write;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_yaml_ng::Value;
use walkdir::WalkDir;

use indexmap::IndexMap;

// 使用ahash作为哈希器的类型别名
type AHashIndexMap<K, V> = IndexMap<K, V, ahash::RandomState>;

fn read_frontmatter(file_path: &str) -> Result<String, Error> {
    // todo:remove unwrap
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut frontmatter_started = false;
    let mut res = String::new();
    for line_res in reader.lines() {
        let line = line_res?;
        if line.starts_with("---") {
            if frontmatter_started {
                break; // End of frontmatter
            } else {
                frontmatter_started = true; // Start of frontmatter
                continue;
            }
        } else if frontmatter_started {
            res.push_str(&line);
            res.push('\n');
        }
    }

    Ok(res)
}

fn parse_yaml_frontmatter(content: &str) -> Result<AHashIndexMap<String, Value>, Error> {
    let data = serde_yaml_ng::from_str::<AHashIndexMap<String, Value>>(content);
    match data {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            eprintln!("Error parsing YAML frontmatter: {}", e);
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse YAML frontmatter",
            ))
        }
    }
}

#[tauri::command]
fn read_and_parse_yaml_frontmatter(file_path: &str) -> AHashIndexMap<String, Value> {
    let content = read_frontmatter(file_path);

    match content {
        Ok(content) => parse_yaml_frontmatter(&content).unwrap_or_else(|_| AHashIndexMap::default()),
        Err(e) => {
            eprintln!("Error reading frontmatter: {}", e);
            AHashIndexMap::default() // Return an empty map on error
        }
    }
}

#[tauri::command]
fn read_and_parse_multiple_frontmatter(
    file_paths: Vec<&str>,
) -> AHashIndexMap<String, AHashIndexMap<String, Value>> {
    file_paths
        .par_iter()
        .map(|&file_path| {
            let frontmatter = read_and_parse_yaml_frontmatter(file_path);
            (file_path.to_string(), frontmatter)
        })
        .collect()
}

fn get_file_ext(s: &str) -> Option<&str> {
    s.rfind('.').map(|pos| &s[pos + 1..])
}

#[tauri::command]
fn get_all_files_of_dir(dir: &str) -> Vec<String> {
    let mut md_files: Vec<String> = vec![];
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && get_file_ext(entry.file_name().to_str().unwrap()) == Some("md")
        {
            md_files.push(entry.path().to_string_lossy().to_string());
        };
    }
    md_files
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_and_parse_yaml_frontmatter,
            read_and_parse_multiple_frontmatter,
            get_all_files_of_dir,
            xlsx::export_frontmatter_to_xlsx,
            xlsx::import_frontmatter_from_xlsx,
            write::write_multiple_frontmatter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
