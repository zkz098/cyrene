use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_yaml_ng::Value;

use indexmap::IndexMap;

// 使用ahash作为哈希器的类型别名
type AHashHashMap<K, V> = HashMap<K, V, ahash::RandomState>;
type AHashIndexMap<K, V> = IndexMap<K, V, ahash::RandomState>;

fn write_frontmatter(file_path: &str, frontmatter_content: &str) -> Result<(), Error> {
    // 读取原文件内容
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut frontmatter_started = false;
    let mut frontmatter_ended = false;
    let mut content_after_frontmatter = String::new();

    // 跳过原有的frontmatter，保存后续内容
    for line_res in reader.lines() {
        let line = line_res?;
        if line.starts_with("---") {
            if frontmatter_started && !frontmatter_ended {
                frontmatter_ended = true;
                continue;
            } else if !frontmatter_started {
                frontmatter_started = true;
                continue;
            }
        }

        if frontmatter_ended || !frontmatter_started {
            content_after_frontmatter.push_str(&line);
            content_after_frontmatter.push('\n');
        }
    }

    // 写入新内容
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    // 写入frontmatter
    writeln!(file, "---")?;
    write!(file, "{}", frontmatter_content)?;
    writeln!(file, "---")?;

    // 写入原有内容
    write!(file, "{}", content_after_frontmatter)?;

    Ok(())
}

fn serialize_yaml_frontmatter(data: &AHashIndexMap<String, Value>) -> Result<String, Error> {
    match serde_yaml_ng::to_string(data) {
        Ok(yaml_string) => Ok(yaml_string),
        Err(e) => {
            eprintln!("Error serializing YAML frontmatter: {}", e);
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to serialize YAML frontmatter",
            ))
        }
    }
}

fn write_yaml_frontmatter(
    file_path: &str,
    data: &AHashIndexMap<String, Value>,
) -> Result<(), Error> {
    let yaml_content = serialize_yaml_frontmatter(data)?;
    write_frontmatter(file_path, &yaml_content)
}

#[tauri::command]
pub fn write_multiple_frontmatter(
    file_data: AHashHashMap<String, AHashIndexMap<String, Value>>,
) -> AHashIndexMap<String, bool> {
    file_data
        .par_iter()
        .map(|(file_path, frontmatter)| {
            let success = write_yaml_frontmatter(file_path, frontmatter).is_ok();
            if !success {
                eprintln!("Failed to write frontmatter to file: {}", file_path);
            }
            (file_path.clone(), success)
        })
        .collect()
}
