use std::collections::HashMap;
use crate::ast::*;
use crate::types::{Type, ParameterInfo, ClassInfo, MethodInfo, FieldInfo, FunctionType, TypeRegistry};
use crate::error::{EolResult, semantic_error};

pub struct SemanticAnalyzer {
    type_registry: TypeRegistry,
    symbol_table: SemanticSymbolTable,
    current_class: Option<String>,
    current_method: Option<String>,
    errors: Vec<String>,
}

pub struct SemanticSymbolTable {
    scopes: Vec<HashMap<String, SemanticSymbolInfo>>,
}

#[derive(Debug, Clone)]
pub struct SemanticSymbolInfo {
    pub name: String,
    pub symbol_type: Type,
    pub is_final: bool,
    pub is_initialized: bool,
}

impl SemanticSymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: String, info: SemanticSymbolInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, info);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SemanticSymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn lookup_current(&self, name: &str) -> Option<&SemanticSymbolInfo> {
        self.scopes.last().and_then(|s| s.get(name))
    }
}

impl Default for SemanticSymbolTable {
    fn default() -> Self {
        Self::new()
    }
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
        
        // 第二遍：分析方法定义
        self.analyze_methods(program)?;
        
        // 第三遍：类型检查
        self.type_check_program(program)?;
        
        if !self.errors.is_empty() {
            return Err(semantic_error(0, 0, self.errors.join("\n")));
        }
        
        Ok(())
    }

    fn collect_classes(&mut self, program: &Program) -> EolResult<()> {
        for class in &program.classes {
            let class_info = ClassInfo {
                name: class.name.clone(),
                methods: HashMap::new(),
                fields: HashMap::new(),
                parent: class.parent.clone(),
            };
            
            self.type_registry.register_class(class_info)?;
        }
        Ok(())
    }

    fn analyze_methods(&mut self, program: &Program) -> EolResult<()> {
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
                        class_info.methods.insert(method.name.clone(), method_info);
                    }
                }
            }
        }
        
        self.current_class = None;
        Ok(())
    }

    fn type_check_program(&mut self, program: &Program) -> EolResult<()> {
        for class in &program.classes {
            self.type_check_class(class)?;
        }
        Ok(())
    }

    fn type_check_class(&mut self, class: &ClassDecl) -> EolResult<()> {
        self.current_class = Some(class.name.clone());
        
        for member in &class.members {
            match member {
                ClassMember::Method(method) => {
                    self.type_check_method(method)?;
                }
                ClassMember::Field(field) => {
                    self.type_check_field(field)?;
                }
            }
        }
        
        self.current_class = None;
        Ok(())
    }

    fn type_check_field(&mut self, field: &FieldDecl) -> EolResult<()> {
        if let Some(ref initializer) = field.initializer {
            let init_type = self.infer_expr_type(initializer)?;
            if !self.types_compatible(&init_type, &field.field_type) {
                return Err(semantic_error(
                    field.loc.line,
                    field.loc.column,
                    format!(
                        "Type mismatch: cannot assign {} to field of type {}",
                        init_type, field.field_type
                    )
                ));
            }
        }
        Ok(())
    }

    fn type_check_method(&mut self, method: &MethodDecl) -> EolResult<()> {
        self.current_method = Some(method.name.clone());
        
        // 创建新的作用域
        self.symbol_table.enter_scope();
        
        // 添加参数到符号表
        for param in &method.params {
            self.symbol_table.declare(
                param.name.clone(),
                SemanticSymbolInfo {
                    name: param.name.clone(),
                    symbol_type: param.param_type.clone(),
                    is_final: true,
                    is_initialized: true,
                }
            );
        }
        
        // 检查方法体
        if let Some(ref body) = method.body {
            for stmt in &body.statements {
                self.type_check_statement(stmt, &method.return_type)?;
            }
        }
        
        // 退出作用域
        self.symbol_table.exit_scope();
        
        self.current_method = None;
        Ok(())
    }

    fn type_check_statement(&mut self, stmt: &Stmt, expected_return: &Type) -> EolResult<()> {
        match stmt {
            Stmt::Expr(expr) => {
                self.infer_expr_type(expr)?;
            }
            Stmt::VarDecl(var) => {
                let init_type = if let Some(ref init) = var.initializer {
                    self.infer_expr_type(init)?
                } else {
                    var.var_type.clone()
                };
                
                if !self.types_compatible(&init_type, &var.var_type) {
                    return Err(semantic_error(
                        var.loc.line,
                        var.loc.column,
                        format!(
                            "Type mismatch: cannot assign {} to variable of type {}",
                            init_type, var.var_type
                        )
                    ));
                }
                
                self.symbol_table.declare(
                    var.name.clone(),
                    SemanticSymbolInfo {
                        name: var.name.clone(),
                        symbol_type: var.var_type.clone(),
                        is_final: var.is_final,
                        is_initialized: var.initializer.is_some(),
                    }
                );
            }
            Stmt::Return(expr) => {
                let actual_return = if let Some(e) = expr {
                    self.infer_expr_type(e)?
                } else {
                    Type::Void
                };
                
                if !self.types_compatible(&actual_return, expected_return) {
                    return Err(semantic_error(
                        0, 0, // TODO: 获取位置信息
                        format!(
                            "Return type mismatch: expected {}, got {}",
                            expected_return, actual_return
                        )
                    ));
                }
            }
            Stmt::If(if_stmt) => {
                let cond_type = self.infer_expr_type(&if_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(semantic_error(
                        if_stmt.loc.line,
                        if_stmt.loc.column,
                        "If condition must be boolean"
                    ));
                }
                
                self.type_check_statement(&if_stmt.then_branch, expected_return)?;
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.type_check_statement(else_branch, expected_return)?;
                }
            }
            Stmt::While(while_stmt) => {
                let cond_type = self.infer_expr_type(&while_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(semantic_error(
                        while_stmt.loc.line,
                        while_stmt.loc.column,
                        "While condition must be boolean"
                    ));
                }
                
                self.type_check_statement(&while_stmt.body, expected_return)?;
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

    fn infer_expr_type(&self, expr: &Expr) -> EolResult<Type> {
        match expr {
            Expr::Literal(lit) => match lit {
                LiteralValue::Int32(_) => Ok(Type::Int32),
                LiteralValue::Int64(_) => Ok(Type::Int64),
                LiteralValue::Float32(_) => Ok(Type::Float32),
                LiteralValue::Float64(_) => Ok(Type::Float64),
                LiteralValue::String(_) => Ok(Type::String),
                LiteralValue::Bool(_) => Ok(Type::Bool),
                LiteralValue::Char(_) => Ok(Type::Char),
                LiteralValue::Null => Ok(Type::Object("Object".to_string())),
            }
            Expr::Identifier(name) => {
                if let Some(info) = self.symbol_table.lookup(name) {
                    Ok(info.symbol_type.clone())
                } else {
                    Err(semantic_error(0, 0, format!("Undefined variable: {}", name)))
                }
            }
            Expr::Binary(bin) => {
                let left_type = self.infer_expr_type(&bin.left)?;
                let right_type = self.infer_expr_type(&bin.right)?;
                
                match bin.op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if left_type.is_primitive() && right_type.is_primitive() {
                            // 类型提升
                            Ok(self.promote_types(&left_type, &right_type))
                        } else {
                            Err(semantic_error(
                                bin.loc.line,
                                bin.loc.column,
                                format!("Cannot apply {:?} to {} and {}", bin.op, left_type, right_type)
                            ))
                        }
                    }
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                        Ok(Type::Bool)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(semantic_error(
                                bin.loc.line,
                                bin.loc.column,
                                "Logical operators require boolean operands"
                            ))
                        }
                    }
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        if left_type.is_integer() && right_type.is_integer() {
                            Ok(self.promote_integer_types(&left_type, &right_type))
                        } else {
                            Err(semantic_error(
                                bin.loc.line,
                                bin.loc.column,
                                format!("Bitwise operator {:?} requires integer operands, got {} and {}",
                                       bin.op, left_type, right_type)
                            ))
                        }
                    }
                    BinaryOp::Shl | BinaryOp::Shr | BinaryOp::UnsignedShr => {
                        if left_type.is_integer() && right_type.is_integer() {
                            // 移位运算符的结果类型与左操作数相同（经过整数提升）
                            Ok(self.promote_integer_types(&left_type, &right_type))
                        } else {
                            Err(semantic_error(
                                bin.loc.line,
                                bin.loc.column,
                                format!("Shift operator {:?} requires integer operands, got {} and {}",
                                       bin.op, left_type, right_type)
                            ))
                        }
                    }
                    _ => Ok(left_type),
                }
            }
            Expr::Unary(unary) => {
                let operand_type = self.infer_expr_type(&unary.operand)?;
                match unary.op {
                    UnaryOp::Neg => Ok(operand_type),
                    UnaryOp::Not => {
                        if operand_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(semantic_error(
                                unary.loc.line,
                                unary.loc.column,
                                "Cannot apply '!' to non-boolean"
                            ))
                        }
                    }
                    UnaryOp::BitNot => Ok(operand_type),
                    _ => Ok(operand_type),
                }
            }
            Expr::Call(call) => {
                // 特殊处理 print 函数
                if let Expr::Identifier(name) = call.callee.as_ref() {
                    if name == "print" {
                        return Ok(Type::Void);
                    }
                }
                
                // TODO: 检查其他函数调用
                Ok(Type::Void)
            }
            Expr::MemberAccess(_) => {
                // TODO: 成员访问类型检查
                Ok(Type::Void)
            }
            Expr::New(new_expr) => {
                if self.type_registry.class_exists(&new_expr.class_name) {
                    Ok(Type::Object(new_expr.class_name.clone()))
                } else {
                    Err(semantic_error(
                        new_expr.loc.line,
                        new_expr.loc.column,
                        format!("Unknown class: {}", new_expr.class_name)
                    ))
                }
            }
            Expr::Assignment(assign) => {
                let target_type = self.infer_expr_type(&assign.target)?;
                let value_type = self.infer_expr_type(&assign.value)?;
                
                if self.types_compatible(&value_type, &target_type) {
                    Ok(target_type)
                } else {
                    Err(semantic_error(
                        assign.loc.line,
                        assign.loc.column,
                        format!("Cannot assign {} to {}", value_type, target_type)
                    ))
                }
            }
            Expr::Cast(cast) => {
                // TODO: 检查转换是否合法
                Ok(cast.target_type.clone())
            }
        }
    }

    fn types_compatible(&self, from: &Type, to: &Type) -> bool {
        if from == to {
            return true;
        }
        
        // 基本类型之间的兼容
        match (from, to) {
            (Type::Int32, Type::Int64) => true,
            (Type::Int32, Type::Float32) => true,
            (Type::Int32, Type::Float64) => true,
            (Type::Int64, Type::Float64) => true,
            (Type::Float32, Type::Float64) => true,
            (Type::Object(_), Type::Object(_)) => true, // TODO: 继承检查
            _ => false,
        }
    }

    fn promote_types(&self, t1: &Type, t2: &Type) -> Type {
        use Type::*;
        match (t1, t2) {
            (Float64, _) | (_, Float64) => Float64,
            (Float32, Float32) => Float32,
            (Float32, _) | (_, Float32) => Float64,
            (Int64, _) | (_, Int64) => Int64,
            (Int32, Int32) => Int32,
            _ => Int32,
        }
    }

    fn promote_integer_types(&self, t1: &Type, t2: &Type) -> Type {
        use Type::*;
        match (t1, t2) {
            (Int64, _) | (_, Int64) => Int64,
            (Int32, Int32) => Int32,
            _ => Int32,
        }
    }
}
