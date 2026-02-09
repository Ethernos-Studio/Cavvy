//! 类定义和主类冲突分析

use crate::ast::{Program, ClassMember, Modifier};
use crate::types::{ClassInfo, FieldInfo, MethodInfo, ParameterInfo};
use crate::error::cayResult;
use super::analyzer::SemanticAnalyzer;

impl SemanticAnalyzer {
    /// 检查主类冲突
    /// 规则：
    /// 1. 如果只有一个类有 main 方法，自动选为主类
    /// 2. 如果有多个类有 main 方法：
    ///    - 如果只有一个类标记了 @main，选该类为主类
    ///    - 如果有多个类标记了 @main，报错
    ///    - 如果没有类标记 @main，报错并提示使用 @main
    pub fn check_main_class_conflicts(&mut self, program: &Program) -> cayResult<()> {
        // 收集所有有 main 方法的类
        let mut main_classes: Vec<(String, bool)> = Vec::new(); // (类名, 是否有@main标记)

        for class in &program.classes {
            let has_main = class.members.iter().any(|m| {
                if let crate::ast::ClassMember::Method(method) = m {
                    method.name == "main"
                        && method.modifiers.contains(&crate::ast::Modifier::Public)
                        && method.modifiers.contains(&crate::ast::Modifier::Static)
                } else {
                    false
                }
            });

            if has_main {
                let has_main_marker = class.modifiers.contains(&crate::ast::Modifier::Main);
                main_classes.push((class.name.clone(), has_main_marker));
            }
        }

        // 分析冲突
        match main_classes.len() {
            0 => {
                // 没有主类，这是允许的（可能是库文件）
                Ok(())
            }
            1 => {
                // 只有一个主类，没有冲突
                Ok(())
            }
            _ => {
                // 多个类有 main 方法，需要检查 @main 标记
                let marked_classes: Vec<&(String, bool)> = main_classes.iter()
                    .filter(|(_, marked)| *marked)
                    .collect();

                match marked_classes.len() {
                    0 => {
                        // 多个类有 main，但没有标记 @main
                        let class_names: Vec<String> = main_classes.iter()
                            .map(|(name, _)| name.clone())
                            .collect();
                        Err(crate::error::semantic_error(
                            0, 0,
                            format!(
                                "多个类包含 main 方法: {}。请使用 @main 标记指定主类，例如：\n@main public class {} {{ ... }}",
                                class_names.join(", "),
                                class_names[0]
                            )
                        ))
                    }
                    1 => {
                        // 只有一个类标记了 @main，这是正确的
                        Ok(())
                    }
                    _ => {
                        // 多个类标记了 @main
                        let marked_names: Vec<String> = marked_classes.iter()
                            .map(|(name, _)| name.clone())
                            .collect();
                        Err(crate::error::semantic_error(
                            0, 0,
                            format!(
                                "多个类标记了 @main: {}。只能有一个主类。",
                                marked_names.join(", ")
                            )
                        ))
                    }
                }
            }
        }
    }

    /// 收集类定义
    pub fn collect_classes(&mut self, program: &Program) -> cayResult<()> {
        for class in &program.classes {
            let mut class_info = ClassInfo {
                name: class.name.clone(),
                methods: std::collections::HashMap::new(),
                fields: std::collections::HashMap::new(),
                parent: class.parent.clone(),
            };
            
            // 收集字段信息
            for member in &class.members {
                if let ClassMember::Field(field) = member {
                    let field_info = FieldInfo {
                        name: field.name.clone(),
                        field_type: field.field_type.clone(),
                        is_public: field.modifiers.contains(&Modifier::Public),
                        is_static: field.modifiers.contains(&Modifier::Static),
                    };
                    class_info.fields.insert(field.name.clone(), field_info);
                }
            }
            
            self.type_registry.register_class(class_info)?;
        }
        Ok(())
    }

    /// 分析方法定义
    pub fn analyze_methods(&mut self, program: &Program) -> cayResult<()> {
        for class in &program.classes {
            self.current_class = Some(class.name.clone());

            for member in &class.members {
                if let ClassMember::Method(method) = member {
                    let method_info = MethodInfo {
                        name: method.name.clone(),
                        class_name: class.name.clone(),
                        params: method.params.clone(),
                        return_type: method.return_type.clone(),
                        is_public: method.modifiers.contains(&Modifier::Public),
                        is_static: method.modifiers.contains(&Modifier::Static),
                        is_native: method.modifiers.contains(&Modifier::Native),
                    };

                    if let Some(class_info) = self.type_registry.classes.get_mut(&class.name) {
                        class_info.add_method(method_info);
                    }
                }
            }
        }
        Ok(())
    }
}
