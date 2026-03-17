//! Cavvy 编译器诊断系统
//!
//! 提供全面的错误、警告和提示信息管理系统。
//! 支持多错误收集、错误代码、详细的上下文信息和修复建议。

use std::fmt;
use std::collections::HashMap;

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// 提示信息，不影响编译
    Note,
    /// 警告，编译继续但可能有问题
    Warning,
    /// 错误，编译失败
    Error,
    /// 致命错误，立即停止编译
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Note => write!(f, "提示"),
            Severity::Warning => write!(f, "警告"),
            Severity::Error => write!(f, "错误"),
            Severity::Fatal => write!(f, "致命错误"),
        }
    }
}

/// 编译阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPhase {
    /// 预处理器
    Preprocessor,
    /// 词法分析
    Lexer,
    /// 语法分析
    Parser,
    /// 语义分析
    Semantic,
    /// 代码生成
    CodeGen,
    /// 链接
    Linker,
}

impl fmt::Display for CompilationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilationPhase::Preprocessor => write!(f, "预处理器"),
            CompilationPhase::Lexer => write!(f, "词法分析"),
            CompilationPhase::Parser => write!(f, "语法分析"),
            CompilationPhase::Semantic => write!(f, "语义分析"),
            CompilationPhase::CodeGen => write!(f, "代码生成"),
            CompilationPhase::Linker => write!(f, "链接器"),
        }
    }
}

/// 源代码位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// 源代码范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: SourceLocation::new(start_line, start_col),
            end: SourceLocation::new(end_line, end_col),
        }
    }

    pub fn single(line: usize, column: usize) -> Self {
        Self {
            start: SourceLocation::new(line, column),
            end: SourceLocation::new(line, column),
        }
    }
}

/// 修复建议
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    /// 建议描述
    pub description: String,
    /// 替换的代码片段（如果有）
    pub replacement: Option<String>,
    /// 替换范围
    pub span: Option<SourceSpan>,
}

impl FixSuggestion {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            replacement: None,
            span: None,
        }
    }

    pub fn with_replacement(mut self, replacement: impl Into<String>, span: SourceSpan) -> Self {
        self.replacement = Some(replacement.into());
        self.span = Some(span);
        self
    }
}

/// 诊断信息（错误、警告、提示）
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 错误代码
    pub code: String,
    /// 严重程度
    pub severity: Severity,
    /// 编译阶段
    pub phase: CompilationPhase,
    /// 错误消息
    pub message: String,
    /// 详细说明
    pub details: Option<String>,
    /// 源代码位置
    pub location: SourceLocation,
    /// 源代码范围
    pub span: Option<SourceSpan>,
    /// 修复建议
    pub suggestions: Vec<FixSuggestion>,
    /// 相关上下文信息
    pub related_info: Vec<RelatedInfo>,
}

/// 相关信息（用于提供额外的上下文）
#[derive(Debug, Clone)]
pub struct RelatedInfo {
    pub message: String,
    pub location: SourceLocation,
}

impl Diagnostic {
    /// 创建新的诊断信息
    pub fn new(
        code: impl Into<String>,
        severity: Severity,
        phase: CompilationPhase,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            phase,
            message: message.into(),
            details: None,
            location,
            span: None,
            suggestions: Vec::new(),
            related_info: Vec::new(),
        }
    }

    /// 创建错误级别的诊断
    pub fn error(
        code: impl Into<String>,
        phase: CompilationPhase,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Self {
        Self::new(code, Severity::Error, phase, message, location)
    }

    /// 创建警告级别的诊断
    pub fn warning(
        code: impl Into<String>,
        phase: CompilationPhase,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Self {
        Self::new(code, Severity::Warning, phase, message, location)
    }

    /// 添加详细说明
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// 添加源代码范围
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// 添加修复建议
    pub fn with_suggestion(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// 添加相关信息
    pub fn with_related_info(mut self, message: impl Into<String>, location: SourceLocation) -> Self {
        self.related_info.push(RelatedInfo {
            message: message.into(),
            location,
        });
        self
    }
}

/// 诊断收集器
#[derive(Debug, Clone, Default)]
pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
    max_errors: usize,
    error_count: usize,
    warning_count: usize,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            max_errors: 100, // 默认最多收集100个错误
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// 添加诊断信息
    pub fn add(&mut self, diagnostic: Diagnostic) {
        match diagnostic.severity {
            Severity::Error | Severity::Fatal => {
                if self.error_count >= self.max_errors {
                    return;
                }
                self.error_count += 1;
            }
            Severity::Warning => {
                self.warning_count += 1;
            }
            _ => {}
        }
        self.diagnostics.push(diagnostic);
    }

    /// 检查是否有错误（不包括警告）
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// 检查是否有致命错误
    pub fn has_fatal_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Fatal)
    }

    /// 检查是否达到最大错误数
    pub fn is_max_errors_reached(&self) -> bool {
        self.error_count >= self.max_errors
    }

    /// 获取所有诊断信息
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// 获取错误数量
    pub fn error_count(&self) -> usize {
        self.error_count
    }

    /// 获取警告数量
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    /// 清空所有诊断信息
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }

    /// 合并另一个收集器的诊断信息
    pub fn merge(&mut self, other: DiagnosticCollector) {
        for diag in other.diagnostics {
            self.add(diag);
        }
    }
}

/// 错误代码定义
pub struct ErrorCodes;

impl ErrorCodes {
    // 预处理器错误 (E1xxx)
    pub const PREPROCESSOR_DEFINE_ERROR: &'static str = "E1001";
    pub const PREPROCESSOR_IFDEF_ERROR: &'static str = "E1002";
    pub const PREPROCESSOR_INCLUDE_ERROR: &'static str = "E1003";
    pub const PREPROCESSOR_UNCLOSED_DIRECTIVE: &'static str = "E1004";
    pub const PREPROCESSOR_CIRCULAR_INCLUDE: &'static str = "E1005";
    pub const PREPROCESSOR_INVALID_MACRO: &'static str = "E1006";

    // 词法错误 (E2xxx)
    pub const LEXER_INVALID_CHARACTER: &'static str = "E2001";
    pub const LEXER_UNTERMINATED_STRING: &'static str = "E2002";
    pub const LEXER_INVALID_ESCAPE_SEQUENCE: &'static str = "E2003";
    pub const LEXER_INVALID_NUMBER_LITERAL: &'static str = "E2004";
    pub const LEXER_UNTERMINATED_COMMENT: &'static str = "E2005";
    pub const LEXER_INVALID_IDENTIFIER: &'static str = "E2006";

    // 语法错误 (E3xxx)
    pub const PARSER_UNEXPECTED_TOKEN: &'static str = "E3001";
    pub const PARSER_EXPECTED_SEMICOLON: &'static str = "E3002";
    pub const PARSER_EXPECTED_BRACE: &'static str = "E3003";
    pub const PARSER_EXPECTED_PAREN: &'static str = "E3004";
    pub const PARSER_EXPECTED_IDENTIFIER: &'static str = "E3005";
    pub const PARSER_EXPECTED_TYPE: &'static str = "E3006";
    pub const PARSER_INVALID_STATEMENT: &'static str = "E3007";
    pub const PARSER_INVALID_EXPRESSION: &'static str = "E3008";
    pub const PARSER_MISSING_MAIN: &'static str = "E3009";
    pub const PARSER_MULTIPLE_MAIN: &'static str = "E3010";

    // 语义错误 (E4xxx)
    pub const SEMANTIC_UNDEFINED_IDENTIFIER: &'static str = "E4001";
    pub const SEMANTIC_DUPLICATE_DEFINITION: &'static str = "E4002";
    pub const SEMANTIC_TYPE_MISMATCH: &'static str = "E4003";
    pub const SEMANTIC_INVALID_CAST: &'static str = "E4004";
    pub const SEMANTIC_INCOMPATIBLE_TYPES: &'static str = "E4005";
    pub const SEMANTIC_UNINITIALIZED_VARIABLE: &'static str = "E4006";
    pub const SEMANTIC_INVALID_OPERATION: &'static str = "E4007";
    pub const SEMANTIC_ACCESS_VIOLATION: &'static str = "E4008";
    pub const SEMANTIC_STATIC_CONTEXT: &'static str = "E4009";
    pub const SEMANTIC_FINAL_REASSIGNMENT: &'static str = "E4010";
    pub const SEMANTIC_MISSING_RETURN: &'static str = "E4011";
    pub const SEMANTIC_RETURN_TYPE_MISMATCH: &'static str = "E4012";
    pub const SEMANTIC_BREAK_OUTSIDE_LOOP: &'static str = "E4013";
    pub const SEMANTIC_CONTINUE_OUTSIDE_LOOP: &'static str = "E4014";
    pub const SEMANTIC_INVALID_ARRAY_SIZE: &'static str = "E4015";
    pub const SEMANTIC_ARRAY_INDEX_TYPE: &'static str = "E4016";
    pub const SEMANTIC_METHOD_NOT_FOUND: &'static str = "E4017";
    pub const SEMANTIC_WRONG_ARGUMENT_COUNT: &'static str = "E4018";
    pub const SEMANTIC_ARGUMENT_TYPE_MISMATCH: &'static str = "E4019";
    pub const SEMANTIC_ABSTRACT_CLASS_INSTANCE: &'static str = "E4020";
    pub const SEMANTIC_OVERRIDE_ERROR: &'static str = "E4021";
    pub const SEMANTIC_INHERITANCE_ERROR: &'static str = "E4022";
    pub const SEMANTIC_CIRCULAR_INHERITANCE: &'static str = "E4023";
    pub const SEMANTIC_FINAL_CLASS_INHERITANCE: &'static str = "E4024";
    pub const SEMANTIC_INTERFACE_IMPL_ERROR: &'static str = "E4025";
    pub const SEMANTIC_VOID_ASSIGNMENT: &'static str = "E4026";
    pub const SEMANTIC_DIVISION_BY_ZERO: &'static str = "E4027";
    pub const SEMANTIC_UNREACHABLE_CODE: &'static str = "E4028";
    pub const SEMANTIC_UNUSED_VARIABLE: &'static str = "E4029";

    // 代码生成错误 (E5xxx)
    pub const CODEGEN_UNSUPPORTED_FEATURE: &'static str = "E5001";
    pub const CODEGEN_TYPE_CONVERSION_ERROR: &'static str = "E5002";
    pub const CODEGEN_SYMBOL_NOT_FOUND: &'static str = "E5003";
    pub const CODEGEN_INVALID_OPERATION: &'static str = "E5004";
    pub const CODEGEN_LLVM_ERROR: &'static str = "E5005";

    // 链接错误 (E6xxx)
    pub const LINKER_SYMBOL_NOT_FOUND: &'static str = "E6001";
    pub const LINKER_MULTIPLE_DEFINITION: &'static str = "E6002";
    pub const LINKER_LIBRARY_NOT_FOUND: &'static str = "E6003";

    /// 获取错误代码的详细说明
    pub fn get_description(code: &str) -> &'static str {
        match code {
            // 预处理器
            Self::PREPROCESSOR_DEFINE_ERROR => "宏定义错误",
            Self::PREPROCESSOR_IFDEF_ERROR => "条件编译指令错误",
            Self::PREPROCESSOR_INCLUDE_ERROR => "文件包含错误",
            Self::PREPROCESSOR_UNCLOSED_DIRECTIVE => "未闭合的预处理器指令",
            Self::PREPROCESSOR_CIRCULAR_INCLUDE => "循环包含错误",
            Self::PREPROCESSOR_INVALID_MACRO => "无效的宏定义",

            // 词法
            Self::LEXER_INVALID_CHARACTER => "非法字符",
            Self::LEXER_UNTERMINATED_STRING => "未闭合的字符串",
            Self::LEXER_INVALID_ESCAPE_SEQUENCE => "无效的转义序列",
            Self::LEXER_INVALID_NUMBER_LITERAL => "无效的数字字面量",
            Self::LEXER_UNTERMINATED_COMMENT => "未闭合的注释",
            Self::LEXER_INVALID_IDENTIFIER => "无效的标识符",

            // 语法
            Self::PARSER_UNEXPECTED_TOKEN => "意外的标记",
            Self::PARSER_EXPECTED_SEMICOLON => "缺少分号",
            Self::PARSER_EXPECTED_BRACE => "缺少大括号",
            Self::PARSER_EXPECTED_PAREN => "缺少括号",
            Self::PARSER_EXPECTED_IDENTIFIER => "缺少标识符",
            Self::PARSER_EXPECTED_TYPE => "缺少类型",
            Self::PARSER_INVALID_STATEMENT => "无效的语句",
            Self::PARSER_INVALID_EXPRESSION => "无效的表达式",
            Self::PARSER_MISSING_MAIN => "缺少主函数",
            Self::PARSER_MULTIPLE_MAIN => "多个主函数",

            // 语义
            Self::SEMANTIC_UNDEFINED_IDENTIFIER => "未定义的标识符",
            Self::SEMANTIC_DUPLICATE_DEFINITION => "重复定义",
            Self::SEMANTIC_TYPE_MISMATCH => "类型不匹配",
            Self::SEMANTIC_INVALID_CAST => "无效的类型转换",
            Self::SEMANTIC_INCOMPATIBLE_TYPES => "不兼容的类型",
            Self::SEMANTIC_UNINITIALIZED_VARIABLE => "未初始化的变量",
            Self::SEMANTIC_INVALID_OPERATION => "无效的操作",
            Self::SEMANTIC_ACCESS_VIOLATION => "访问权限错误",
            Self::SEMANTIC_STATIC_CONTEXT => "静态上下文错误",
            Self::SEMANTIC_FINAL_REASSIGNMENT => "final变量重新赋值",
            Self::SEMANTIC_MISSING_RETURN => "缺少返回值",
            Self::SEMANTIC_RETURN_TYPE_MISMATCH => "返回值类型不匹配",
            Self::SEMANTIC_BREAK_OUTSIDE_LOOP => "break在循环外",
            Self::SEMANTIC_CONTINUE_OUTSIDE_LOOP => "continue在循环外",
            Self::SEMANTIC_INVALID_ARRAY_SIZE => "无效的数组大小",
            Self::SEMANTIC_ARRAY_INDEX_TYPE => "数组索引类型错误",
            Self::SEMANTIC_METHOD_NOT_FOUND => "方法未找到",
            Self::SEMANTIC_WRONG_ARGUMENT_COUNT => "参数数量错误",
            Self::SEMANTIC_ARGUMENT_TYPE_MISMATCH => "参数类型不匹配",
            Self::SEMANTIC_ABSTRACT_CLASS_INSTANCE => "抽象类实例化",
            Self::SEMANTIC_OVERRIDE_ERROR => "重写错误",
            Self::SEMANTIC_INHERITANCE_ERROR => "继承错误",
            Self::SEMANTIC_CIRCULAR_INHERITANCE => "循环继承",
            Self::SEMANTIC_FINAL_CLASS_INHERITANCE => "final类继承错误",
            Self::SEMANTIC_INTERFACE_IMPL_ERROR => "接口实现错误",
            Self::SEMANTIC_VOID_ASSIGNMENT => "void赋值错误",
            Self::SEMANTIC_DIVISION_BY_ZERO => "除零错误",
            Self::SEMANTIC_UNREACHABLE_CODE => "不可达代码",
            Self::SEMANTIC_UNUSED_VARIABLE => "未使用的变量",

            // 代码生成
            Self::CODEGEN_UNSUPPORTED_FEATURE => "不支持的功能",
            Self::CODEGEN_TYPE_CONVERSION_ERROR => "类型转换错误",
            Self::CODEGEN_SYMBOL_NOT_FOUND => "符号未找到",
            Self::CODEGEN_INVALID_OPERATION => "无效的操作",
            Self::CODEGEN_LLVM_ERROR => "LLVM错误",

            // 链接
            Self::LINKER_SYMBOL_NOT_FOUND => "链接符号未找到",
            Self::LINKER_MULTIPLE_DEFINITION => "重复定义",
            Self::LINKER_LIBRARY_NOT_FOUND => "库未找到",

            _ => "未知错误",
        }
    }

    /// 获取错误代码的修复建议
    pub fn get_suggestion(code: &str) -> &'static str {
        match code {
            Self::LEXER_INVALID_CHARACTER => "请删除非法字符或使用支持的字符",
            Self::LEXER_UNTERMINATED_STRING => "请在字符串末尾添加双引号",
            Self::LEXER_INVALID_ESCAPE_SEQUENCE => "请使用有效的转义序列: \\n, \\t, \\\", \\\\",
            Self::PARSER_EXPECTED_SEMICOLON => "请在语句末尾添加分号 ';'",
            Self::PARSER_EXPECTED_BRACE => "请添加大括号 '{' 或 '}'",
            Self::PARSER_EXPECTED_PAREN => "请添加括号 '(' 或 ')'",
            Self::SEMANTIC_UNDEFINED_IDENTIFIER => "请检查拼写或声明该标识符",
            Self::SEMANTIC_TYPE_MISMATCH => "请确保类型兼容或进行显式转换",
            Self::SEMANTIC_BREAK_OUTSIDE_LOOP => "break只能在循环或switch中使用",
            Self::SEMANTIC_CONTINUE_OUTSIDE_LOOP => "continue只能在循环中使用",
            _ => "请检查代码并修复错误",
        }
    }
}

/// 格式化诊断信息为字符串
pub fn format_diagnostic(diagnostic: &Diagnostic, source: &str, filename: &str) -> String {
    let mut output = String::new();

    // 标题
    output.push_str(&format!(
        "\n[{}] {} ({})",
        diagnostic.severity, diagnostic.code, ErrorCodes::get_description(&diagnostic.code)
    ));
    output.push_str(&format!("\n文件: {}", filename));
    output.push_str(&format!(
        "\n位置: 第 {} 行, 第 {} 列",
        diagnostic.location.line, diagnostic.location.column
    ));

    // 源代码上下文
    if diagnostic.location.line > 0 {
        output.push_str("\n\n源代码上下文:");
        let lines: Vec<&str> = source.lines().collect();
        let start = diagnostic.location.line.saturating_sub(3).max(1);
        let end = (diagnostic.location.line + 1).min(lines.len());

        for i in start..=end {
            if i <= lines.len() {
                output.push_str(&format!("\n{:4} | {}", i, lines[i - 1]));
                if i == diagnostic.location.line {
                    let spaces = " ".repeat(diagnostic.location.column.saturating_sub(1) + 6);
                    output.push_str(&format!("\n{}^ {}", spaces, diagnostic.message));
                }
            }
        }
    }

    // 详细说明
    if let Some(details) = &diagnostic.details {
        output.push_str(&format!("\n\n详细说明: {}", details));
    }

    // 修复建议
    if !diagnostic.suggestions.is_empty() {
        output.push_str("\n\n修复建议:");
        for (i, suggestion) in diagnostic.suggestions.iter().enumerate() {
            output.push_str(&format!("\n  {}. {}", i + 1, suggestion.description));
            if let Some(replacement) = &suggestion.replacement {
                output.push_str(&format!("\n     建议代码: {}", replacement));
            }
        }
    }

    // 相关信息
    if !diagnostic.related_info.is_empty() {
        output.push_str("\n\n相关信息:");
        for info in &diagnostic.related_info {
            output.push_str(&format!(
                "\n  第 {} 行: {}",
                info.location.line, info.message
            ));
        }
    }

    output.push('\n');
    output
}

/// 格式化所有诊断信息
pub fn format_all_diagnostics(collector: &DiagnosticCollector, source: &str, filename: &str) -> String {
    let mut output = String::new();

    for diagnostic in collector.diagnostics() {
        output.push_str(&format_diagnostic(diagnostic, source, filename));
    }

    // 统计信息
    if collector.error_count() > 0 || collector.warning_count() > 0 {
        output.push_str(&format!(
            "\n编译结果: {} 个错误, {} 个警告\n",
            collector.error_count(),
            collector.warning_count()
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_collector() {
        let mut collector = DiagnosticCollector::new();

        let diag = Diagnostic::error(
            ErrorCodes::SEMANTIC_TYPE_MISMATCH,
            CompilationPhase::Semantic,
            "类型不匹配",
            SourceLocation::new(10, 5),
        );

        collector.add(diag);
        assert!(collector.has_errors());
        assert_eq!(collector.error_count(), 1);
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorCodes::get_description("E4001"), "未定义的标识符");
        assert_eq!(ErrorCodes::get_description("E9999"), "未知错误");
    }
}
