//! Cavvy 预处理器模块
//! 
//! 实现 0.3.5.0 版本的预处理指令系统：
//! - #include "path"  - 文件包含（隐式 #pragma once）
//! - #define NAME value  - 常量定义（无参数宏）
//! - #ifdef / #ifndef / #endif  - 条件编译
//! - #error "message"  - 编译期错误
//! - #warning "message"  - 编译期警告
//! 
//! 设计约束：
//! - 仅支持简单常量定义，禁止宏函数
//! - 不支持 #else / #elif，简化条件逻辑
//! - 隐式 #pragma once 基于绝对路径哈希
//! - 预处理在词法分析之前执行，生成纯源代码

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use crate::error::{cayResult, cayError};

/// 预处理器状态
pub struct Preprocessor {
    /// 已定义的宏常量 (name -> value)
    defines: HashMap<String, String>,
    /// 已包含的文件路径集合（用于 #pragma once 语义）
    included_files: HashSet<String>,
    /// 基础目录（用于解析相对路径）
    base_dir: PathBuf,
    /// 当前条件编译栈
    conditional_stack: Vec<ConditionalState>,
    /// 是否处于被跳过的代码块中
    skipping: bool,
}

/// 条件编译状态
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConditionalState {
    /// 当前条件为真，正在处理代码
    Active,
    /// 当前条件为假，跳过代码
    Skipping,
}

/// 预处理指令类型
#[derive(Debug, Clone)]
enum Directive {
    /// #include "path"
    Include(String),
    /// #define name value
    Define(String, String),
    /// #ifdef name
    Ifdef(String),
    /// #ifndef name
    Ifndef(String),
    /// #endif
    Endif,
    /// #error "message"
    Error(String),
    /// #warning "message"
    Warning(String),
}

impl Preprocessor {
    /// 创建新的预处理器实例
    /// 
    /// # Arguments
    /// * `base_dir` - 源代码基础目录，用于解析相对路径
    /// 
    /// # Returns
    /// 初始化后的预处理器
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            defines: HashMap::new(),
            included_files: HashSet::new(),
            base_dir: base_dir.as_ref().to_path_buf(),
            conditional_stack: Vec::new(),
            skipping: false,
        }
    }

    /// 预处理源文件，返回处理后的源代码
    /// 
    /// # Arguments
    /// * `source` - 原始源代码
    /// * `file_path` - 源文件路径（用于错误报告）
    /// 
    /// # Returns
    /// 预处理后的源代码字符串
    /// 
    /// # Errors
    /// 当遇到无效指令或文件无法读取时返回错误
    pub fn process(&mut self, source: &str, file_path: &str) -> cayResult<String> {
        let lines: Vec<&str> = source.lines().collect();
        let mut output_lines = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // 检查是否是预处理指令行
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                match self.parse_directive(trimmed, line_number, file_path) {
                    Ok(Some(directive)) => {
                        self.process_directive(directive, &mut output_lines, file_path)?;
                    }
                    Ok(None) => {
                        // 跳过空指令（如纯注释）
                    }
                    Err(e) => return Err(e),
                }
            } else if self.skipping {
                // 处于条件编译跳过状态，不输出代码行
                // 但仍需跟踪行号以保持行号映射（用于调试信息）
                output_lines.push("".to_string());
            } else {
                // 普通代码行，进行宏替换后输出
                let processed = self.expand_macros(line);
                output_lines.push(processed);
            }
        }
        
        // 检查条件编译栈是否为空
        if !self.conditional_stack.is_empty() {
            return Err(cayError::Preprocessor {
                line: lines.len(),
                column: 1,
                message: "未闭合的条件编译指令，缺少 #endif".to_string(),
                suggestion: "请为每个 #ifdef 或 #ifndef 添加对应的 #endif".to_string(),
            });
        }
        
        Ok(output_lines.join("\n"))
    }

    /// 解析单行预处理指令
    /// 
    /// # Arguments
    /// * `line` - 已去除前导空白的行内容
    /// * `line_num` - 行号（用于错误报告）
    /// * `file_path` - 文件路径（用于错误报告）
    /// 
    /// # Returns
    /// 解析出的指令或 None
    fn parse_directive(&self, line: &str, line_num: usize, _file_path: &str) -> cayResult<Option<Directive>> {
        // 去除 # 后面的空白
        let content = line[1..].trim_start();
        
        if content.is_empty() {
            return Ok(None);
        }
        
        // 提取指令名和参数
        let mut parts = content.splitn(2, |c: char| c.is_whitespace());
        let directive_name = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("").trim();
        
        match directive_name {
            "include" => {
                // 解析 #include "path"
                let path = self.parse_string_literal(args, line_num)?;
                Ok(Some(Directive::Include(path)))
            }
            "define" => {
                // 解析 #define name value
                let (name, value) = self.parse_define_args(args, line_num)?;
                Ok(Some(Directive::Define(name, value)))
            }
            "ifdef" => {
                let name = self.parse_identifier(args, line_num)?;
                Ok(Some(Directive::Ifdef(name)))
            }
            "ifndef" => {
                let name = self.parse_identifier(args, line_num)?;
                Ok(Some(Directive::Ifndef(name)))
            }
            "endif" => {
                if !args.is_empty() {
                    return Err(cayError::Preprocessor {
                        line: line_num,
                        column: 1,
                        message: "#endif 指令不接受参数".to_string(),
                        suggestion: "使用 #endif 而不是 #endif CONDITION".to_string(),
                    });
                }
                Ok(Some(Directive::Endif))
            }
            "error" => {
                let message = self.parse_string_literal(args, line_num)?;
                Ok(Some(Directive::Error(message)))
            }
            "warning" => {
                let message = self.parse_string_literal(args, line_num)?;
                Ok(Some(Directive::Warning(message)))
            }
            _ => {
                Err(cayError::Preprocessor {
                    line: line_num,
                    column: 1,
                    message: format!("未知的预处理指令: {}", directive_name),
                    suggestion: "支持的指令: #include, #define, #ifdef, #ifndef, #endif, #error, #warning".to_string(),
                })
            }
        }
    }

    /// 解析字符串字面量（用于 #include, #error, #warning）
    fn parse_string_literal(&self, args: &str, line_num: usize) -> cayResult<String> {
        let trimmed = args.trim();
        if trimmed.len() < 2 {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: "缺少字符串参数".to_string(),
                suggestion: "使用双引号包围字符串，例如: \"path/to/file.cay\"".to_string(),
            });
        }
        
        // 只支持双引号
        if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: "参数必须是双引号字符串".to_string(),
                suggestion: "使用 \"path\" 而不是 <path>".to_string(),
            });
        }
        
        Ok(trimmed[1..trimmed.len()-1].to_string())
    }

    /// 解析标识符
    fn parse_identifier(&self, args: &str, line_num: usize) -> cayResult<String> {
        let trimmed = args.trim();
        if trimmed.is_empty() {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: "缺少标识符参数".to_string(),
                suggestion: "提供标识符名称，例如: #ifdef DEBUG".to_string(),
            });
        }
        
        // 检查是否是有效的标识符
        let first_char = trimmed.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: format!("无效的标识符: {}", trimmed),
                suggestion: "标识符必须以字母或下划线开头".to_string(),
            });
        }
        
        // 只取第一个标识符（后面的内容忽略）
        let ident: String = trimmed.chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        
        Ok(ident)
    }

    /// 解析 #define 的参数
    fn parse_define_args(&self, args: &str, line_num: usize) -> cayResult<(String, String)> {
        let trimmed = args.trim();
        if trimmed.is_empty() {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: "#define 缺少宏名称".to_string(),
                suggestion: "使用格式: #define NAME value 或 #define NAME".to_string(),
            });
        }
        
        // 查找第一个空白字符分隔名称和值
        let mut parts = trimmed.splitn(2, |c: char| c.is_whitespace());
        let name = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").trim().to_string();
        
        // 检查名称是否包含括号（禁止宏函数）
        if name.contains('(') {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: "不支持宏函数".to_string(),
                suggestion: "Cavvy 预处理器仅支持简单常量定义，使用 static final 方法代替".to_string(),
            });
        }
        
        // 验证标识符格式
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(cayError::Preprocessor {
                line: line_num,
                column: 1,
                message: format!("无效的宏名称: {}", name),
                suggestion: "宏名称必须以字母或下划线开头".to_string(),
            });
        }
        
        Ok((name, value))
    }

    /// 处理预处理指令
    fn process_directive(
        &mut self,
        directive: Directive,
        output_lines: &mut Vec<String>,
        file_path: &str,
    ) -> cayResult<()> {
        match directive {
            Directive::Include(path) => {
                if !self.skipping {
                    self.handle_include(&path, output_lines, file_path)?;
                }
            }
            Directive::Define(name, value) => {
                if !self.skipping {
                    self.defines.insert(name, value);
                }
            }
            Directive::Ifdef(name) => {
                let should_process = self.defines.contains_key(&name);
                self.push_conditional(should_process);
            }
            Directive::Ifndef(name) => {
                let should_process = !self.defines.contains_key(&name);
                self.push_conditional(should_process);
            }
            Directive::Endif => {
                self.pop_conditional()?;
            }
            Directive::Error(message) => {
                if !self.skipping {
                    return Err(cayError::Preprocessor {
                        line: 0,
                        column: 0,
                        message: format!("#error: {}", message),
                        suggestion: "根据编译条件移除此错误或修改预处理器条件".to_string(),
                    });
                }
            }
            Directive::Warning(message) => {
                if !self.skipping {
                    // 警告通过 eprintln 输出但不中断编译
                    eprintln!("warning: {}", message);
                }
            }
        }
        Ok(())
    }

    /// 处理 #include 指令
    fn handle_include(
        &mut self,
        path: &str,
        output_lines: &mut Vec<String>,
        current_file: &str,
    ) -> cayResult<()> {
        // 解析完整路径
        let include_path = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            // 相对于当前文件目录解析
            let current_dir = Path::new(current_file).parent()
                .unwrap_or(&self.base_dir);
            current_dir.join(path)
        };
        
        // 标准化路径用于去重检查
        let canonical_path = include_path.canonicalize()
            .map_err(|e| cayError::Io(
                format!("无法解析包含路径 '{}': {}", path, e)
            ))?;
        
        let path_key = canonical_path.to_string_lossy().to_string();
        
        // 隐式 #pragma once: 检查是否已包含
        if self.included_files.contains(&path_key) {
            return Ok(());
        }
        
        // 读取文件内容
        let content = std::fs::read_to_string(&canonical_path)
            .map_err(|e| cayError::Io(
                format!("无法读取包含文件 '{}': {}", path, e)
            ))?;
        
        // 标记为已包含
        self.included_files.insert(path_key.clone());
        
        // 递归处理被包含的文件
        let sub_path = canonical_path.to_string_lossy();
        let processed = self.process(&content, &sub_path)?;
        
        // 添加行标记（用于调试信息映射）
        output_lines.push(format!("// #line 1 {:?}", sub_path));
        output_lines.push(processed);
        output_lines.push(format!("// #line end {:?}", sub_path));
        
        Ok(())
    }

    /// 压入条件编译状态
    fn push_conditional(&mut self, should_process: bool) {
        self.conditional_stack.push(
            if self.skipping || !should_process {
                ConditionalState::Skipping
            } else {
                ConditionalState::Active
            }
        );
        self.skipping = self.conditional_stack.iter()
            .any(|state| *state == ConditionalState::Skipping);
    }

    /// 弹出条件编译状态
    fn pop_conditional(&mut self) -> cayResult<()> {
        if self.conditional_stack.pop().is_none() {
            return Err(cayError::Preprocessor {
                line: 0,
                column: 0,
                message: "多余的 #endif".to_string(),
                suggestion: "确保每个 #endif 都有对应的 #ifdef 或 #ifndef".to_string(),
            });
        }
        
        self.skipping = self.conditional_stack.iter()
            .any(|state| *state == ConditionalState::Skipping);
        
        Ok(())
    }

    /// 展开宏定义（简单的文本替换）
    fn expand_macros(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // 按名称长度降序排序，避免短名称替换干扰长名称
        let mut macros: Vec<(&String, &String)> = self.defines.iter().collect();
        macros.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        
        for (name, value) in macros {
            // 简单的字符串替换
            // 注意：这不处理注释、字符串字面量等边界情况
            // 对于 0.3.5.0 版本，这是可接受的简化
                    result = result.replace(name, value);
        }
        
        result
    }
}

/// 便捷的预处理函数
/// 
/// # Arguments
/// * `source` - 原始源代码
/// * `file_path` - 源文件路径
/// * `base_dir` - 基础目录
/// 
/// # Returns
/// 预处理后的源代码
pub fn preprocess(source: &str, file_path: &str, base_dir: impl AsRef<Path>) -> cayResult<String> {
    let mut preprocessor = Preprocessor::new(base_dir);
    preprocessor.process(source, file_path)
}

