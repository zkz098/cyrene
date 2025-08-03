// 常用的字符串常量，避免重复分配
pub const FRONTMATTER_DELIMITER: &str = "---";
pub const EMPTY_YAML_OBJECT: &str = "{}";
pub const COMMA_SEPARATOR: &str = ", ";

// 文件扩展名
pub const MD_EXTENSION: &str = "md";

// 布尔值字符串
pub const TRUE_VALUES: &[&str] = &["true", "True", "TRUE"];
pub const FALSE_VALUES: &[&str] = &["false", "False", "FALSE"];

// 错误信息常量
pub const ERROR_EXCEL_SAVE: &str = "保存Excel文件时出错: ";
pub const ERROR_EXCEL_READ: &str = "读取Excel文件时出错: ";
pub const SUCCESS_EXCEL_SAVE: &str = "Excel文件已成功保存到: ";
