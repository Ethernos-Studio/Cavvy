//! 语义分析器核心实现

use crate::ast::*;
use crate::types::{Type, ParameterInfo, ClassInfo, MethodInfo, FieldInfo, TypeRegistry};
use crate::error::{cayResult, semantic_error};
use super::symbol_table::{SemanticSymbolTable, SemanticSymbolInfo};

/// 语义分析器
pub struct SemanticAnalyzer {
    pub(super) program: Option<std::rc::Rc<Program>>,  // 保存 AST 以供类型推断使用
    pub(super) type_registry: TypeRegistry,
    pub(super) symbol_table: SemanticSymbolTable,
    pub(super) current_class: Option<String>,
    pub(super) current_method: Option<String>,
    pub(super) current_method_is_static: bool,  // 当前方法是否是静态方法
    pub(super) current_method_is_constructor: bool,  // 当前是否是构造函数
    pub(super) errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            program: None,
            type_registry: TypeRegistry::new(),
            symbol_table: SemanticSymbolTable::new(),
            current_class: None,
            current_method: None,
            current_method_is_static: false,
            current_method_is_constructor: false,
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

    pub fn analyze(&mut self, program: &Program) -> cayResult<()> {
        // 保存 program 引用以供类型推断使用
        self.program = Some(std::rc::Rc::new(program.clone()));

        // 第一遍：收集所有类定义
        self.collect_classes(program)?;

        // 注册运行时函数到 NetworkUtils 类
        self.register_runtime_functions();

        // 检查主类冲突（在收集类之后，类型检查之前）
        self.check_main_class_conflicts(program)?;

        // 第二遍：分析方法定义
        self.analyze_methods(program)?;

        // 第三遍：检查继承关系（包括 @Override 验证）
        self.check_inheritance(program)?;

        // 第四遍：类型检查
        self.type_check_program(program)?;

        if !self.errors.is_empty() {
            return Err(semantic_error(0, 0, self.errors.join("\n")));
        }

        Ok(())
    }

    /// 注册运行时函数到相应的类
    fn register_runtime_functions(&mut self) {
        // 向 NetworkUtils 类添加 __cay_buffer_to_string 方法
        if let Some(class_info) = self.type_registry.get_class_mut("NetworkUtils") {
            // 创建方法信息: String __cay_buffer_to_string(long buffer, int length)
            let method = MethodInfo {
                name: "__cay_buffer_to_string".to_string(),
                class_name: "NetworkUtils".to_string(),
                params: vec![
                    ParameterInfo::new("buffer".to_string(), Type::Int64),
                    ParameterInfo::new("length".to_string(), Type::Int32),
                ],
                return_type: Type::String,
                is_public: true,
                is_private: false,
                is_protected: false,
                is_static: true,
                is_native: true,
                is_override: false,
                is_final: false,
            };

            class_info.add_method(method);
        }
    }

    /// 获取类型注册表（用于代码生成）
    pub fn get_type_registry(&self) -> &TypeRegistry {
        &self.type_registry
    }
}
