//! 类型检查实现

use crate::ast::*;
use crate::types::{Type, ParameterInfo};
use crate::error::{EolResult, semantic_error};
use super::analyzer::SemanticAnalyzer;
use super::symbol_table::SemanticSymbolInfo;

impl SemanticAnalyzer {
    /// 类型检查程序
    pub fn type_check_program(&mut self, program: &Program) -> EolResult<()> {
        for class in &program.classes {
            self.current_class = Some(class.name.clone());
            
            for member in &class.members {
                match member {
                    ClassMember::Method(method) => {
                        self.current_method = Some(method.name.clone());
                        self.symbol_table.enter_scope();
                        
                        // 添加参数到符号表
                        for param in &method.params {
                            self.symbol_table.declare(
                                param.name.clone(),
                                SemanticSymbolInfo {
                                    name: param.name.clone(),
                                    symbol_type: param.param_type.clone(),
                                    is_final: false,
                                    is_initialized: true,
                                }
                            );
                        }
                        
                        // 类型检查方法体
                        if let Some(body) = &method.body {
                            self.type_check_statement(&Stmt::Block(body.clone()), Some(&method.return_type))?;
                        }
                        
                        self.symbol_table.exit_scope();
                        self.current_method = None;
                    }
                    ClassMember::Field(_) => {
                        // 字段类型检查暂不实现
                    }
                }
            }
            
            self.current_class = None;
        }
        Ok(())
    }

    /// 类型检查语句
    pub fn type_check_statement(&mut self, stmt: &Stmt, expected_return: Option<&Type>) -> EolResult<()> {
        match stmt {
            Stmt::Expr(expr) => {
                self.infer_expr_type(expr)?;
            }
            Stmt::VarDecl(var) => {
                let var_type = var.var_type.clone();
                if let Some(init) = &var.initializer {
                    let init_type = self.infer_expr_type(init)?;
                    if !self.types_compatible(&init_type, &var_type) {
                        self.errors.push(format!(
                            "Cannot assign {} to {} at line {}",
                            init_type, var_type, var.loc.line
                        ));
                    }
                }
                
                self.symbol_table.declare(
                    var.name.clone(),
                    SemanticSymbolInfo {
                        name: var.name.clone(),
                        symbol_type: var_type,
                        is_final: var.is_final,
                        is_initialized: var.initializer.is_some(),
                    }
                );
            }
            Stmt::Return(expr) => {
                let return_type = if let Some(e) = expr {
                    self.infer_expr_type(e)?
                } else {
                    Type::Void
                };
                
                if let Some(expected) = expected_return {
                    if !self.types_compatible(&return_type, expected) {
                        self.errors.push(format!(
                            "Return type mismatch: expected {}, got {}",
                            expected, return_type
                        ));
                    }
                }
            }
            Stmt::Block(block) => {
                self.symbol_table.enter_scope();
                for stmt in &block.statements {
                    self.type_check_statement(stmt, expected_return)?;
                }
                self.symbol_table.exit_scope();
            }
            _ => {}
        }
        
        Ok(())
    }
}
