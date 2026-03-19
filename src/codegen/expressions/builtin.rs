//! 内置函数调用代码生成
//!
//! 处理 print/println/readInt/readFloat/readLine 等内置函数。

use crate::codegen::context::IRGenerator;
use crate::ast::*;
use crate::error::{cayResult, codegen_error};

impl IRGenerator {
    /// 生成 print/println 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表
    /// * `newline` - 是否打印换行符
    pub fn generate_print_call(&mut self, args: &[Expr], newline: bool) -> cayResult<String> {
        if args.is_empty() {
            // 无参数，仅打印换行符（如果是 println）或什么都不做（如果是 print）
            if newline {
                // 打印一个空字符串加上换行符
                let fmt_str = "\n";
                let fmt_name = self.get_or_create_string_constant(fmt_str);
                let fmt_len = fmt_str.len() + 1;
                let fmt_ptr = self.new_temp();
                self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    fmt_ptr, fmt_len, fmt_len, fmt_name));
                self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {})", fmt_ptr));
            }
            // 对于 print 无参数，什么都不做
            return Ok("void".to_string());
        }
        
        let first_arg = &args[0];
        
        match first_arg {
            Expr::Literal(LiteralValue::String(s)) => {
                let global_name = self.get_or_create_string_constant(s);
                let fmt_str = if newline { "%s\n" } else { "%s" };
                let fmt_name = self.get_or_create_string_constant(fmt_str);
                let len = s.len() + 1;
                let fmt_len = fmt_str.len() + 1; // 加上null终止符
                
                let str_ptr = self.new_temp();
                let fmt_ptr = self.new_temp();
                
                self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    str_ptr, len, len, global_name));
                self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    fmt_ptr, fmt_len, fmt_len, fmt_name));
                
                self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, i8* {})",
                    fmt_ptr, str_ptr));
            }
            Expr::Literal(LiteralValue::Int32(_)) | Expr::Literal(LiteralValue::Int64(_)) => {
                let value = self.generate_expression(first_arg)?;
                let (type_str, val) = self.parse_typed_value(&value);
                let i64_fmt = self.get_i64_format_specifier();
                let fmt_str = if newline { format!("{}\n", i64_fmt) } else { i64_fmt.to_string() };
                let fmt_name = self.get_or_create_string_constant(&fmt_str);
                let fmt_len = fmt_str.len() + 1;

                let fmt_ptr = self.new_temp();
                self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    fmt_ptr, fmt_len, fmt_len, fmt_name));

                // 如果类型不是 i64，需要扩展
                let final_val = if type_str != "i64" {
                    let ext_temp = self.new_temp();
                    self.emit_line(&format!("  {} = sext {} {} to i64", ext_temp, type_str, val));
                    ext_temp
                } else {
                    val.to_string()
                };

                self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, i64 {})",
                    fmt_ptr, final_val));
            }
            _ => {
                // 根据类型决定格式字符串
                let value = self.generate_expression(first_arg)?;
                let (type_str, val) = self.parse_typed_value(&value);
                
                if type_str == "i8*" {
                    // 字符串指针类型
                    let fmt_str = if newline { "%s\n" } else { "%s" };
                    let fmt_name = self.get_or_create_string_constant(fmt_str);
                    let fmt_len = fmt_str.len() + 1;
                    let fmt_ptr = self.new_temp();
                    self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        fmt_ptr, fmt_len, fmt_len, fmt_name));
                    self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, i8* {})",
                        fmt_ptr, val));
                } else if type_str.starts_with("i") && type_str != "i8*" {
                    // 整数类型（排除i8*）
                    // 需要将整数扩展为 i64 以匹配格式
                    let i64_fmt = self.get_i64_format_specifier();
                    let fmt_str = if newline { format!("{}\n", i64_fmt) } else { i64_fmt.to_string() };
                    let fmt_name = self.get_or_create_string_constant(&fmt_str);
                    let fmt_len = fmt_str.len() + 1;
                    let fmt_ptr = self.new_temp();
                    self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        fmt_ptr, fmt_len, fmt_len, fmt_name));

                    // 如果类型不是 i64，需要扩展
                    let final_val = if type_str != "i64" {
                        let ext_temp = self.new_temp();
                        self.emit_line(&format!("  {} = sext {} {} to i64", ext_temp, type_str, val));
                        ext_temp
                    } else {
                        val.to_string()
                    };

                    self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, i64 {})",
                        fmt_ptr, final_val));
                } else if type_str == "double" || type_str == "float" {
                    // 浮点数类型
                    let fmt_str = if newline { "%f\n" } else { "%f" };
                    let fmt_name = self.get_or_create_string_constant(fmt_str);
                    let fmt_len = fmt_str.len() + 1;
                    let fmt_ptr = self.new_temp();
                    self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        fmt_ptr, fmt_len, fmt_len, fmt_name));
                    
                    // 如果类型是float，需要转换为double
                    let final_val = if type_str == "float" {
                        let ext_temp = self.new_temp();
                        self.emit_line(&format!("  {} = fpext float {} to double", ext_temp, val));
                        ext_temp
                    } else {
                        val.to_string()
                    };
                    
                    self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, double {})",
                        fmt_ptr, final_val));
                } else {
                    // 默认作为字符串处理
                    let fmt_str = if newline { "%s\n" } else { "%s" };
                    let fmt_name = self.get_or_create_string_constant(fmt_str);
                    let fmt_len = fmt_str.len() + 1;
                    let fmt_ptr = self.new_temp();
                    self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        fmt_ptr, fmt_len, fmt_len, fmt_name));
                    self.emit_line(&format!("  call i32 (i8*, ...) @printf(i8* {}, {})",
                        fmt_ptr, value));
                }
            }
        }
        
        Ok("i64 0".to_string())
    }

    /// 生成 readInt 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_int_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readInt 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readInt() takes no arguments".to_string()));
        }
        
        // 为输入缓冲区分配空间
        let buffer_size = 32; // 足够存储整数
        let buffer_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca [{} x i8], align 1", buffer_temp, buffer_size));
        
        // 获取缓冲区指针
        let buffer_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            buffer_ptr, buffer_size, buffer_size, buffer_temp));
        
        // 调用 scanf 读取整数
        let fmt_str = self.get_i64_format_specifier();
        let fmt_name = self.get_or_create_string_constant(fmt_str);
        let fmt_len = fmt_str.len() + 1;
        let fmt_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            fmt_ptr, fmt_len, fmt_len, fmt_name));
        
        // 为整数结果分配空间
        let int_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca i64, align 8", int_temp));
        
        // 调用 scanf
        self.emit_line(&format!("  call i32 (i8*, ...) @scanf(i8* {}, i64* {})",
            fmt_ptr, int_temp));
        
        // 加载读取的整数值
        let result_temp = self.new_temp();
        self.emit_line(&format!("  {} = load i64, i64* {}, align 8", result_temp, int_temp));
        
        Ok(format!("i64 {}", result_temp))
    }

    /// 生成 readFloat 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_float_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readFloat 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readFloat() takes no arguments".to_string()));
        }
        
        // 为浮点数结果分配空间
        let float_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca double, align 8", float_temp));
        
        // 调用 scanf 读取浮点数
        let fmt_str = "%lf";
        let fmt_name = self.get_or_create_string_constant(fmt_str);
        let fmt_len = fmt_str.len() + 1;
        let fmt_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            fmt_ptr, fmt_len, fmt_len, fmt_name));
        
        // 调用 scanf
        self.emit_line(&format!("  call i32 (i8*, ...) @scanf(i8* {}, double* {})",
            fmt_ptr, float_temp));
        
        // 加载读取的浮点数值
        let result_temp = self.new_temp();
        self.emit_line(&format!("  {} = load double, double* {}, align 8", result_temp, float_temp));
        
        Ok(format!("double {}", result_temp))
    }

    /// 生成 readLine 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_line_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readLine 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readLine() takes no arguments".to_string()));
        }

        // 为输入缓冲区分配空间（假设最大256字符）
        let buffer_size = 256;
        let buffer_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca [{} x i8], align 1", buffer_temp, buffer_size));

        // 获取缓冲区指针
        let buffer_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            buffer_ptr, buffer_size, buffer_size, buffer_temp));

        // 根据目标平台获取stdin
        let stdin_ref = if self.target_triple.contains("windows") || self.target_triple.contains("mingw") {
            // Windows: 使用 __acrt_iob_func(0) 获取stdin
            let stdin_ptr = self.new_temp();
            self.emit_line(&format!("  {} = call i8* @__acrt_iob_func(i32 0)", stdin_ptr));
            stdin_ptr
        } else {
            // Linux/macOS: 使用 @stdin 全局变量
            "@stdin".to_string()
        };

        // 调用 fgets 读取一行
        self.emit_line(&format!("  call i8* @fgets(i8* {}, i32 {}, i8* {})",
            buffer_ptr, buffer_size, stdin_ref));

        // 移除换行符（如果需要）
        // 这里我们直接返回缓冲区指针
        Ok(format!("i8* {}", buffer_ptr))
    }

    /// 生成 readLong 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_long_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readLong 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readLong() takes no arguments".to_string()));
        }

        // 为输入缓冲区分配空间
        let buffer_size = 32;
        let buffer_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca [{} x i8], align 1", buffer_temp, buffer_size));

        // 获取缓冲区指针
        let buffer_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            buffer_ptr, buffer_size, buffer_size, buffer_temp));

        // 调用 scanf 读取长整数
        let fmt_str = "%lld";
        let fmt_name = self.get_or_create_string_constant(fmt_str);
        let fmt_len = fmt_str.len() + 1;
        let fmt_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            fmt_ptr, fmt_len, fmt_len, fmt_name));

        // 为长整数结果分配空间
        let long_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca i64, align 8", long_temp));

        // 调用 scanf
        self.emit_line(&format!("  call i32 (i8*, ...) @scanf(i8* {}, i64* {})",
            fmt_ptr, long_temp));

        // 加载读取的长整数值
        let result_temp = self.new_temp();
        self.emit_line(&format!("  {} = load i64, i64* {}, align 8", result_temp, long_temp));

        Ok(format!("i64 {}", result_temp))
    }

    /// 生成 readDouble 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_double_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readDouble 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readDouble() takes no arguments".to_string()));
        }

        // 为双精度浮点数结果分配空间
        let double_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca double, align 8", double_temp));

        // 调用 scanf 读取双精度浮点数
        let fmt_str = "%lf";
        let fmt_name = self.get_or_create_string_constant(fmt_str);
        let fmt_len = fmt_str.len() + 1;
        let fmt_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            fmt_ptr, fmt_len, fmt_len, fmt_name));

        // 调用 scanf
        self.emit_line(&format!("  call i32 (i8*, ...) @scanf(i8* {}, double* {})",
            fmt_ptr, double_temp));

        // 加载读取的双精度浮点数值
        let result_temp = self.new_temp();
        self.emit_line(&format!("  {} = load double, double* {}, align 8", result_temp, double_temp));

        Ok(format!("double {}", result_temp))
    }

    /// 生成 readChar 调用代码
    ///
    /// # Arguments
    /// * `args` - 参数列表（应该为空）
    pub fn generate_read_char_call(&mut self, args: &[Expr]) -> cayResult<String> {
        // readChar 应该没有参数
        if !args.is_empty() {
            return Err(codegen_error("readChar() takes no arguments".to_string()));
        }

        // 为字符结果分配空间
        let char_temp = self.new_temp();
        self.emit_line(&format!("  {} = alloca i8, align 1", char_temp));

        // 调用 scanf 读取字符（跳过空白字符）
        let fmt_str = " %c";
        let fmt_name = self.get_or_create_string_constant(fmt_str);
        let fmt_len = fmt_str.len() + 1;
        let fmt_ptr = self.new_temp();
        self.emit_line(&format!("  {} = getelementptr [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            fmt_ptr, fmt_len, fmt_len, fmt_name));

        // 调用 scanf
        self.emit_line(&format!("  call i32 (i8*, ...) @scanf(i8* {}, i8* {})",
            fmt_ptr, char_temp));

        // 加载读取的字符值
        let result_temp = self.new_temp();
        self.emit_line(&format!("  {} = load i8, i8* {}, align 1", result_temp, char_temp));

        Ok(format!("i8 {}", result_temp))
    }

}
