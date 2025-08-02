use std::path::Path;
use serde_yaml_ng::Value;
use umya_spreadsheet::*;
use crate::write;

use indexmap::{IndexMap, IndexSet};

#[tauri::command]
pub fn export_frontmatter_to_xlsx(
    data: IndexMap<String, IndexMap<String, Value>>,
    output_path: String,
) -> Result<String, String> {
    // 创建新的工作簿
    let mut book = new_file();
    
    // 获取第一个工作表
    let worksheet = book.get_sheet_mut(&0).unwrap();
    
    // 设置第一个单元格为空（左上角）
    worksheet.get_cell_mut((1, 1)).set_value("");
    
    // 收集所有可能的字段名
    let mut all_keys: IndexSet<String> = IndexSet::new();
    for frontmatter in data.values() {
        for key in frontmatter.keys() {
            all_keys.insert(key.clone());
        }
    }
    
    // 设置文件路径作为列标题
    let mut col_index = 2u32;
    let mut path_to_col: IndexMap<String, u32> = IndexMap::new();
    for file_path in data.keys() {
        worksheet.get_cell_mut((1, col_index)).set_value(file_path);
        path_to_col.insert(file_path.clone(), col_index);
        col_index += 1;
    }
    
    // 设置字段名作为行标题并填充数据
    let mut row_index = 2u32;
    for key in &all_keys {
        // 设置字段名作为行标题
        worksheet.get_cell_mut((row_index, 1)).set_value(key);
        
        // 为每个文件填充该字段的值
        for (file_path, frontmatter) in &data {
            if let Some(&col) = path_to_col.get(file_path) {
                let value_str = if let Some(value) = frontmatter.get(key) {
                    // 将Value转换为字符串
                    match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Sequence(seq) => {
                            // 将数组转换为逗号分隔的字符串
                            seq.iter()
                                .map(|v| value_to_string(v))
                                .collect::<Vec<String>>()
                                .join(", ")
                        },
                        Value::Mapping(_) => {
                            // 将对象转换为JSON字符串
                            serde_yaml_ng::to_string(&value).unwrap_or_else(|_| "{}".to_string())
                        },
                        Value::Null => String::new(),
                        _ => format!("{:?}", value),
                    }
                } else {
                    String::new() // 如果该文件没有这个字段，则为空
                };
                
                worksheet.get_cell_mut((row_index, col)).set_value(&value_str);
            }
        }
        
        row_index += 1;
    }
    
    // 保存文件
    match writer::xlsx::write(&book, &output_path) {
        Ok(_) => Ok(format!("Excel文件已成功保存到: {}", output_path)),
        Err(e) => Err(format!("保存Excel文件时出错: {}", e)),
    }
}

#[tauri::command]
pub fn import_frontmatter_from_xlsx(
    xlsx_path: String,
    base_path: String,
) -> Result<String, String> {
    // 读取XLSX文件
    let book = match reader::xlsx::read(&xlsx_path) {
        Ok(book) => book,
        Err(e) => return Err(format!("读取Excel文件时出错: {}", e)),
    };
    
    // 获取第一个工作表
    let worksheet = book.get_sheet(&0).unwrap();
    
    // 读取文件路径（第一行，从第2列开始）
    let mut file_paths: Vec<String> = Vec::new();
    let mut col_index = 2u32;
    loop {
        if let Some(cell) = worksheet.get_cell((1, col_index)) {
            let value = cell.get_value();
            let path_str = value.to_string();
            if !path_str.is_empty() {
                // 处理路径拼接，确保相对路径正确处理
                let full_path = if path_str.starts_with('/') || path_str.starts_with('\\') {
                    // 如果路径以 / 或 \ 开头，去掉开头的分隔符，作为相对路径处理
                    let relative_path = path_str.trim_start_matches(['/', '\\']);
                    Path::new(&base_path).join(relative_path)
                } else {
                    // 直接拼接相对路径
                    Path::new(&base_path).join(&path_str)
                };
                file_paths.push(full_path.to_string_lossy().to_string());
            } else {
                break;
            }
        } else {
            break;
        }
        col_index += 1;
    }
    
    // 读取字段名（第一列，从第2行开始）
    let mut field_names: Vec<String> = Vec::new();
    let mut row_index = 2u32;
    loop {
        if let Some(cell) = worksheet.get_cell((row_index, 1)) {
            let value = cell.get_value();
            let field_name = value.to_string();
            if !field_name.is_empty() {
                field_names.push(field_name);
            } else {
                break;
            }
        } else {
            break;
        }
        row_index += 1;
    }
    
    // 构建数据结构
    let mut file_data: IndexMap<String, IndexMap<String, Value>> = IndexMap::new();
    
    for (col_idx, file_path) in file_paths.iter().enumerate() {
        let mut frontmatter: IndexMap<String, Value> = IndexMap::new();
        
        for (row_idx, field_name) in field_names.iter().enumerate() {
            let actual_col = (col_idx + 2) as u32; // 从第2列开始
            let actual_row = (row_idx + 2) as u32; // 从第2行开始
            
            if let Some(cell) = worksheet.get_cell((actual_row, actual_col)) {
                let cell_value = cell.get_value();
                let value_str = cell_value.to_string();
                if !value_str.is_empty() {
                    // 尝试解析值的类型
                    let parsed_value = parse_cell_value(&value_str);
                    frontmatter.insert(field_name.clone(), parsed_value);
                }
            }
        }
        
        if !frontmatter.is_empty() {
            file_data.insert(file_path.clone(), frontmatter);
        }
    }
    
    // 使用write函数批量写入
    let write_results = write::write_multiple_frontmatter(file_data);
    
    // 统计结果
    let total_files = write_results.len();
    let successful_files = write_results.values().filter(|&&success| success).count();
    let failed_files = total_files - successful_files;
    
    if failed_files > 0 {
        Ok(format!(
            "导入完成：成功 {} 个文件，失败 {} 个文件",
            successful_files, failed_files
        ))
    } else {
        Ok(format!("导入完成：成功处理 {} 个文件", successful_files))
    }
}

// 辅助函数：解析单元格值为合适的YAML值类型
fn parse_cell_value(value_str: &str) -> Value {
    // 尝试解析为布尔值
    if value_str.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if value_str.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    
    // 尝试解析为数字
    if let Ok(int_val) = value_str.parse::<i64>() {
        return Value::Number(serde_yaml_ng::Number::from(int_val));
    }
    if let Ok(float_val) = value_str.parse::<f64>() {
        return Value::Number(serde_yaml_ng::Number::from(float_val));
    }
    
    // 检查是否是逗号分隔的数组
    if value_str.contains(", ") {
        let items: Vec<Value> = value_str
            .split(", ")
            .map(|s| parse_cell_value(s.trim()))
            .collect();
        return Value::Sequence(items);
    }
    
    // 尝试解析为YAML对象（如果以{开始）
    if value_str.starts_with('{') && value_str.ends_with('}') {
        if let Ok(parsed) = serde_yaml_ng::from_str::<Value>(value_str) {
            return parsed;
        }
    }
    
    // 默认作为字符串处理
    Value::String(value_str.to_string())
}

// 辅助函数：将Value转换为字符串
fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => format!("{:?}", value),
    }
}
