//! 语义分析器核心实现

use crate::ast::*;
use crate::types::{Type, ParameterInfo, ClassInfo, MethodInfo, FieldInfo, TypeRegistry};
use crate::error::{EolResult, semantic_error};
use super::symbol_table::{SemanticSymbolTable, SemanticSymbolInfo};

/// 语义分析器
pub struct SemanticAnalyzer {
    pub(super) type_registry: TypeRegistry,
    pub(super) symbol_table: SemanticSymbolTable,
    pub(super) current_class: Option<String>,
    pub(super) current_method: Option<String>,
    pub(super) errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            type_registry: TypeRegistry::new(),
            symbol_table: SemanticSymbolTable::new(),
            current_class: None,
            current_method: None,
            errors: Vec::new(),
        };
        
        // 注册内置函数
        analyzer.register_builtin_functions();
        
        analyzer
    }

    fn register_builtin_functions(&mut self) {
        // 注册 print 函数 - 作为特殊处理
        // print 可以接受任意类型参数
    }

    pub fn analyze(&mut self, program: &Program) -> EolResult<()> {
        // 第一遍：收集所有类定义
        self.collect_classes(program)?;

        // 检查主类冲突（在收集类之后，类型检查之前）
        self.check_main_class_conflicts(program)?;

        // 第二遍：分析方法定义
        self.analyze_methods(program)?;

        // 第三遍：类型检查
        self.type_check_program(program)?;

        if !self.errors.is_empty() {
            return Err(semantic_error(0, 0, self.errors.join("\n")));
        }

        Ok(())
    }

    /// 获取类型注册表（用于代码生成）
    pub fn get_type_registry(&self) -> &TypeRegistry {
        &self.type_registry
    }
}
