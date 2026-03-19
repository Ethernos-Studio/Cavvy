//! 语法检查模块
//!
//! 集成cay-check功能，提供实时代码检查

use std::sync::mpsc::Sender;
use std::thread;
use crate::idle::editor::Diagnostic;

pub type CheckResult = Result<Vec<Diagnostic>, String>;

/// 语法检查器
pub struct SyntaxChecker {
    /// 是否正在检查
    is_checking: bool,
}

impl SyntaxChecker {
    pub fn new() -> Self {
        Self {
            is_checking: false,
        }
    }
    
    /// 异步检查代码
    pub fn check_async(&mut self, code: String, sender: Sender<CheckResult>) {
        if self.is_checking {
            return;
        }
        
        self.is_checking = true;
        
        thread::spawn(move || {
            let result = Self::check_code(&code);
            let _ = sender.send(result);
        });
    }
    
    /// 同步检查代码（内部使用）
    fn check_code(code: &str) -> CheckResult {
        // 创建临时文件
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("cay_check_{}.cay", std::process::id()));
        
        // 写入代码
        if let Err(e) = std::fs::write(&temp_file, code) {
            return Err(format!("无法创建临时文件: {}", e));
        }
        
        // 调用内部检查函数
        let result = Self::check_file_internal(&temp_file);
        
        // 清理临时文件
        let _ = std::fs::remove_file(&temp_file);
        
        result
    }
    
    /// 内部检查实现（使用编译器API）
    fn check_file_internal(path: &std::path::Path) -> CheckResult {
        use crate::lexer;
        use crate::parser;
        use crate::semantic;
        
        // 读取文件
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(format!("无法读取文件: {}", e)),
        };
        
        let mut diagnostics = Vec::new();
        
        // 1. 词法分析
        let tokens = match lexer::lex(&source) {
            Ok(t) => t,
            Err(e) => {
                // 解析错误信息
                let (line, col, msg) = Self::parse_error_message(&e.to_string());
                diagnostics.push(Diagnostic {
                    line,
                    column: col,
                    message: msg,
                    is_error: true,
                });
                return Ok(diagnostics);
            }
        };
        
        // 2. 语法分析
        let ast = match parser::parse(tokens) {
            Ok(a) => a,
            Err(e) => {
                let (line, col, msg) = Self::parse_error_message(&e.to_string());
                diagnostics.push(Diagnostic {
                    line,
                    column: col,
                    message: msg,
                    is_error: true,
                });
                return Ok(diagnostics);
            }
        };
        
        // 3. 语义分析
        let mut analyzer = semantic::SemanticAnalyzer::new();
        if let Err(e) = analyzer.analyze(&ast) {
            let (line, col, msg) = Self::parse_error_message(&e.to_string());
            diagnostics.push(Diagnostic {
                line,
                column: col,
                message: msg,
                is_error: true,
            });
        }
        
        Ok(diagnostics)
    }
    
    /// 解析错误消息提取行号和列号
    fn parse_error_message(error_msg: &str) -> (usize, usize, String) {
        let mut line = 1;
        let mut col = 1;
        
        // 尝试解析常见的错误格式
        // 例如: "Error at line 5, column 10: ..."
        if let Some(line_start) = error_msg.find("line") {
            let after_line = &error_msg[line_start + 4..];
            if let Some(num_end) = after_line.find(|c: char| !c.is_ascii_digit() && c != ' ') {
                let num_str = after_line[..num_end].trim();
                if let Ok(n) = num_str.parse::<usize>() {
                    line = n;
                }
            }
        }
        
        if let Some(col_start) = error_msg.find("column") {
            let after_col = &error_msg[col_start + 6..];
            if let Some(num_end) = after_col.find(|c: char| !c.is_ascii_digit() && c != ' ') {
                let num_str = after_col[..num_end].trim();
                if let Ok(n) = num_str.parse::<usize>() {
                    col = n;
                }
            }
        }
        
        // 清理错误消息
        let msg = error_msg
            .replace("Error: ", "")
            .replace("error: ", "")
            .trim()
            .to_string();
        
        (line, col, msg)
    }
    
    /// 标记检查完成
    pub fn mark_done(&mut self) {
        self.is_checking = false;
    }
    
    /// 是否正在检查
    pub fn is_checking(&self) -> bool {
        self.is_checking
    }
}

/// 快速检查函数（用于外部调用）
pub fn quick_check(code: &str) -> Vec<Diagnostic> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("cay_quick_check_{}.cay", std::process::id()));
    
    if std::fs::write(&temp_file, code).is_err() {
        return vec![Diagnostic {
            line: 1,
            column: 1,
            message: "无法创建临时文件".to_string(),
            is_error: true,
        }];
    }
    
    let result = SyntaxChecker::check_file_internal(&temp_file);
    let _ = std::fs::remove_file(&temp_file);
    
    match result {
        Ok(diagnostics) => diagnostics,
        Err(e) => vec![Diagnostic {
            line: 1,
            column: 1,
            message: e,
            is_error: true,
        }],
    }
}
