use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read},
    path::Path,
};

type AHashHashMap<K, V> = HashMap<K, V, ahash::RandomState>;

#[tauri::command]
pub fn backup_files_as_tar_zst(
    file_list: Vec<String>,
    base_path: String,
    output_path: String,
    zstd_level: i32,
) -> Result<(), String> {
    let base_dir = Path::new(&base_path);

    let tar_zst_path = Path::new(&output_path);
    let tar_zst_file = File::create(tar_zst_path).map_err(|e| e.to_string())?;
    let buf_writer = BufWriter::new(tar_zst_file);

    let encoder = zstd::stream::Encoder::new(buf_writer, zstd_level).map_err(|e| e.to_string())?;
    let mut tar_builder = tar::Builder::new(encoder);

    let mut file_hashes: AHashHashMap<String, String> = AHashHashMap::default();

    for rel_path in &file_list {
        // 规范化相对路径，确保它是相对路径
        let normalized_rel_path = if rel_path.starts_with('/') || rel_path.starts_with('\\') {
            // 如果路径以 / 或 \ 开头，去掉开头的分隔符
            rel_path.trim_start_matches(['/', '\\']).to_string()
        } else {
            rel_path.clone()
        };

        // 构建完整路径用于读取文件
        let full_path = base_dir.join(&normalized_rel_path);
        println!("Processing file: {}", full_path.display());

        let hash = calculate_sha256(&full_path)?;
        file_hashes.insert(normalized_rel_path.clone(), hash);

        // 使用规范化的相对路径作为归档内的路径
        tar_builder
            .append_path_with_name(&full_path, &normalized_rel_path)
            .map_err(|e| e.to_string())?;
    }

    // 生成验证文件内容
    if !file_hashes.is_empty() {
        let verify_content = serde_yaml_ng::to_string(&file_hashes).map_err(|e| e.to_string())?;
        let verify_bytes = verify_content.as_bytes();

        // 添加验证文件到 tar 归档
        let mut header = tar::Header::new_gnu();
        header.set_path("verify.yml").map_err(|e| e.to_string())?;
        header.set_size(verify_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();

        tar_builder
            .append(&header, verify_bytes)
            .map_err(|e| e.to_string())?;
    }

    tar_builder.finish().map_err(|e| e.to_string())?;

    let encoder = tar_builder.into_inner().map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())?;

    Ok(())
}

// 计算文件的 SHA256 值
fn calculate_sha256(file_path: &Path) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

// 计算字节数据的 SHA256 值
fn calculate_sha256_from_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[derive(serde::Serialize)]
pub struct RestoreResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub failed_files: Vec<String>,
}

#[tauri::command]
pub fn restore_files_from_tar_zst(
    backup_path: String,
    restore_base_path: String,
) -> Result<RestoreResult, String> {
    use std::fs::create_dir_all;
    use std::io::{BufReader, Write};
    
    let backup_file = File::open(&backup_path).map_err(|e| e.to_string())?;
    let buf_reader = BufReader::new(backup_file);
    let decoder = zstd::stream::Decoder::new(buf_reader).map_err(|e| e.to_string())?;
    let mut tar_archive = tar::Archive::new(decoder);

    let restore_base_dir = Path::new(&restore_base_path);
    
    // 首先提取所有文件到临时位置，并找到 verify.yml
    let mut temp_files: AHashHashMap<String, Vec<u8>> = AHashHashMap::default();
    let mut verify_content: Option<String> = None;

    // 读取归档中的所有条目
    for entry in tar_archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path().map_err(|e| e.to_string())?;
        let path_str = path.to_string_lossy().to_string();

        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if path_str == "verify.yml" {
            verify_content = Some(String::from_utf8(buffer).map_err(|e| e.to_string())?);
        } else {
            temp_files.insert(path_str, buffer);
        }
    }

    // 解析验证文件
    let file_hashes: AHashHashMap<String, String> = match verify_content {
        Some(content) => serde_yaml_ng::from_str(&content).map_err(|e| e.to_string())?,
        None => {
            return Err("未找到验证文件 verify.yml".to_string());
        }
    };

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut failed_files = Vec::new();

    // 恢复每个文件并进行校验
    for (rel_path, expected_hash) in file_hashes {
        match temp_files.get(&rel_path) {
            Some(file_data) => {
                // 计算文件的 SHA256 哈希值
                let actual_hash = calculate_sha256_from_bytes(file_data);
                
                if actual_hash == expected_hash {
                    // 哈希值匹配，恢复文件
                    let full_restore_path = restore_base_dir.join(&rel_path);
                    
                    // 确保父目录存在
                    if let Some(parent) = full_restore_path.parent() {
                        create_dir_all(parent).map_err(|e| e.to_string())?;
                    }
                    
                    // 写入文件
                    let mut output_file = File::create(&full_restore_path).map_err(|e| e.to_string())?;
                    output_file.write_all(file_data).map_err(|e| e.to_string())?;
                    
                    success_count += 1;
                    println!("成功恢复文件: {}", rel_path);
                } else {
                    // 哈希值不匹配，跳过此文件
                    failed_count += 1;
                    failed_files.push(rel_path.clone());
                    println!("文件校验失败，跳过: {} (期望: {}, 实际: {})", rel_path, expected_hash, actual_hash);
                }
            }
            None => {
                // 在归档中未找到文件
                failed_count += 1;
                failed_files.push(rel_path.clone());
                println!("在归档中未找到文件: {}", rel_path);
            }
        }
    }

    Ok(RestoreResult {
        success_count,
        failed_count,
        failed_files,
    })
}
