//! 类型工具函数

use crate::ast::Expr;
use crate::types::{Type, ParameterInfo};
use crate::error::cayResult;
use super::analyzer::SemanticAnalyzer;

impl SemanticAnalyzer {
    /// 检查类型兼容性
    pub fn types_compatible(&self, from: &Type, to: &Type) -> bool {
        if from == to {
            return true;
        }

        // null 可以赋值给任何引用类型（包括 string）
        if let Type::Object(obj_name) = from {
            if obj_name == "Object" {
                // null 是 Object 类型，可以赋值给 String 或其他引用类型
                return true;
            }
        }

        // 基本类型之间的兼容
        match (from, to) {
            (Type::Int32, Type::Int64) => true,
            (Type::Int32, Type::Float32) => true,
            (Type::Int32, Type::Float64) => true,
            (Type::Int64, Type::Float64) => true,
            (Type::Float32, Type::Float64) => true,
            (Type::Float64, Type::Float32) => true, // 允许double到float转换（可能有精度损失）
            (Type::Object(_), Type::Object(_)) => true, // TODO: 继承检查
            // char 可以赋值给 int (ASCII 码值)
            (Type::Char, Type::Int32) => true,
            (Type::Char, Type::Int64) => true,
            // 数组类型：检查元素类型兼容性
            (Type::Array(from_elem), Type::Array(to_elem)) => {
                self.types_compatible(from_elem, to_elem)
            }
            _ => false,
        }
    }

    /// 类型提升规则
    pub fn promote_types(&self, left: &Type, right: &Type) -> Type {
        match (left, right) {
            (Type::Float64, _) | (_, Type::Float64) => Type::Float64,
            (Type::Float32, _) | (_, Type::Float32) => Type::Float32,
            (Type::Int64, _) | (_, Type::Int64) => Type::Int64,
            // char 类型在算术运算中提升为 int32
            (Type::Char, Type::Char) => Type::Int32,
            (Type::Char, Type::Int32) | (Type::Int32, Type::Char) => Type::Int32,
            (Type::Int32, Type::Int32) => Type::Int32,
            _ => left.clone(),
        }
    }

    /// 整数类型提升
    pub fn promote_integer_types(&self, left: &Type, right: &Type) -> Type {
        match (left, right) {
            (Type::Int64, _) | (_, Type::Int64) => Type::Int64,
            _ => Type::Int32,
        }
    }

    /// 检查参数是否与参数定义兼容（支持可变参数）
    pub fn check_arguments_compatible(&mut self, args: &[Expr], params: &[ParameterInfo], _line: usize, _column: usize) -> Result<(), String> {
        if params.is_empty() {
            if args.is_empty() {
                return Ok(());
            } else {
                return Err(format!("Expected 0 arguments, got {}", args.len()));
            }
        }

        // 检查最后一个参数是否是可变参数
        let last_idx = params.len() - 1;
        if params[last_idx].is_varargs {
            // 可变参数：至少需要 params.len() - 1 个参数
            if args.len() < last_idx {
                return Err(format!("Expected at least {} arguments, got {}", last_idx, args.len()));
            }

            // 检查固定参数
            for i in 0..last_idx {
                let arg_type = self.infer_expr_type(&args[i]).map_err(|e| e.to_string())?;
                if !self.types_compatible(&arg_type, &params[i].param_type) {
                    return Err(format!("Argument {} type mismatch: expected {}, got {}",
                        i + 1, params[i].param_type, arg_type));
                }
            }

            // 检查可变参数
            // 可变参数类型是 Array(ElementType)，需要匹配 ElementType
            let vararg_element_type = match &params[last_idx].param_type {
                Type::Array(elem) => elem.as_ref(),
                _ => &params[last_idx].param_type,
            };
            for i in last_idx..args.len() {
                let arg_type = self.infer_expr_type(&args[i]).map_err(|e| e.to_string())?;
                if !self.types_compatible(&arg_type, vararg_element_type) {
                    return Err(format!("Varargs argument {} type mismatch: expected {}, got {}",
                        i + 1, vararg_element_type, arg_type));
                }
            }
        } else {
            // 非可变参数：参数数量必须完全匹配
            if params.len() != args.len() {
                return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
            }

            for (i, (arg, param)) in args.iter().zip(params.iter()).enumerate() {
                let arg_type = self.infer_expr_type(arg).map_err(|e| e.to_string())?;
                if !self.types_compatible(&arg_type, &param.param_type) {
                    return Err(format!("Argument {} type mismatch: expected {}, got {}",
                        i + 1, param.param_type, arg_type));
                }
            }
        }

        Ok(())
    }

    /// 推断 String 方法调用的返回类型
    pub fn infer_string_method_call(&mut self, method_name: &str, args: &[Expr], line: usize, column: usize) -> cayResult<Type> {
        use crate::error::semantic_error;
        
        match method_name {
            "length" => {
                if !args.is_empty() {
                    return Err(semantic_error(line, column, "String.length() takes no arguments".to_string()));
                }
                Ok(Type::Int32)
            }
            "substring" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(semantic_error(line, column, "String.substring() takes 1 or 2 arguments".to_string()));
                }
                // 检查参数类型
                for (i, arg) in args.iter().enumerate() {
                    let arg_type = self.infer_expr_type(arg)?;
                    if !arg_type.is_integer() {
                        return Err(semantic_error(line, column, format!("Argument {} of substring() must be integer, got {}", i + 1, arg_type)));
                    }
                }
                Ok(Type::String)
            }
            "indexOf" => {
                if args.len() != 1 {
                    return Err(semantic_error(line, column, "String.indexOf() takes 1 argument".to_string()));
                }
                let arg_type = self.infer_expr_type(&args[0])?;
                if arg_type != Type::String {
                    return Err(semantic_error(line, column, format!("Argument of indexOf() must be string, got {}", arg_type)));
                }
                Ok(Type::Int32)
            }
            "charAt" => {
                if args.len() != 1 {
                    return Err(semantic_error(line, column, "String.charAt() takes 1 argument".to_string()));
                }
                let arg_type = self.infer_expr_type(&args[0])?;
                if !arg_type.is_integer() {
                    return Err(semantic_error(line, column, format!("Argument of charAt() must be integer, got {}", arg_type)));
                }
                Ok(Type::Char)
            }
            "replace" => {
                if args.len() != 2 {
                    return Err(semantic_error(line, column, "String.replace() takes 2 arguments".to_string()));
                }
                for (i, arg) in args.iter().enumerate() {
                    let arg_type = self.infer_expr_type(arg)?;
                    if arg_type != Type::String {
                        return Err(semantic_error(line, column, format!("Argument {} of replace() must be string, got {}", i + 1, arg_type)));
                    }
                }
                Ok(Type::String)
            }
            _ => Err(semantic_error(line, column, format!("Unknown String method '{}'", method_name))),
        }
    }
}
