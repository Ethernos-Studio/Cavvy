/// JIT/AOT 编译器
/// 将Cavvy字节码编译为LLVM IR，然后编译为机器码

use super::*;

/// JIT编译错误
#[derive(Debug)]
pub enum JitError {
    IoError(std::io::Error),
    CompilationError(String),
    LinkingError(String),
    InvalidBytecode(String),
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::IoError(e) => write!(f, "IO error: {}", e),
            JitError::CompilationError(e) => write!(f, "Compilation error: {}", e),
            JitError::LinkingError(e) => write!(f, "Linking error: {}", e),
            JitError::InvalidBytecode(e) => write!(f, "Invalid bytecode: {}", e),
        }
    }
}

impl std::error::Error for JitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JitError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for JitError {
    fn from(e: std::io::Error) -> Self {
        JitError::IoError(e)
    }
}

/// JIT编译选项
#[derive(Debug, Clone)]
pub struct JitOptions {
    /// 优化级别
    pub optimization: String,
    /// 目标平台
    pub target: String,
    /// 保留中间文件
    pub keep_intermediate: bool,
    /// 输出目录
    pub output_dir: Option<String>,
    /// 链接的库
    pub link_libs: Vec<String>,
    /// 库搜索路径
    pub lib_paths: Vec<String>,
}

impl Default for JitOptions {
    fn default() -> Self {
        Self {
            optimization: "-O2".to_string(),
            target: get_default_target(),
            keep_intermediate: false,
            output_dir: None,
            link_libs: Vec::new(),
            lib_paths: Vec::new(),
        }
    }
}

/// 获取默认目标平台
fn get_default_target() -> String {
    if cfg!(target_os = "windows") {
        "x86_64-w64-mingw32".to_string()
    } else if cfg!(target_os = "linux") {
        "x86_64-unknown-linux-gnu".to_string()
    } else if cfg!(target_os = "macos") {
        "x86_64-apple-darwin".to_string()
    } else {
        "x86_64-unknown-linux-gnu".to_string()
    }
}

/// JIT编译器
pub struct JitCompiler {
    options: JitOptions,
}

impl JitCompiler {
    /// 创建新的JIT编译器
    pub fn new(options: JitOptions) -> Self {
        Self { options }
    }

    /// 编译字节码模块为可执行文件
    pub fn compile_to_executable(&self, module: &BytecodeModule, output_path: &str) -> Result<(), JitError> {
        // 1. 将字节码转换为LLVM IR
        let ir_code = self.bytecode_to_ir(module)?;

        // 2. 确定输出目录
        let output_dir = self.options.output_dir.as_ref()
            .map(|s| std::path::PathBuf::from(s))
            .unwrap_or_else(|| std::env::temp_dir().join("cavvy-jit"));

        std::fs::create_dir_all(&output_dir)?;

        // 3. 写入IR文件
        let ir_file = output_dir.join("output.ll");
        std::fs::write(&ir_file, &ir_code)?;

        // 4. 编译IR为对象文件
        let obj_file = output_dir.join("output.o");
        self.compile_ir_to_object(&ir_file, &obj_file)?;

        // 5. 链接对象文件为可执行文件
        self.link_executable(&obj_file, output_path)?;

        // 6. 清理中间文件
        if !self.options.keep_intermediate {
            let _ = std::fs::remove_file(&ir_file);
            let _ = std::fs::remove_file(&obj_file);
        }

        Ok(())
    }

    /// 将字节码转换为LLVM IR
    fn bytecode_to_ir(&self, module: &BytecodeModule) -> Result<String, JitError> {
        let mut ir = String::new();

        // 添加IR头部
        ir.push_str("; Cavvy Bytecode Compiled IR\n");
        ir.push_str(&format!("; Module: {}\n", module.header.name));
        ir.push_str(&format!("; Target: {}\n", module.header.target_platform));
        ir.push_str(&format!("; Obfuscated: {}\n\n", module.header.obfuscated));

        // 添加目标平台声明
        ir.push_str(self.generate_target_declarations());

        // 添加运行时声明
        ir.push_str(self.generate_runtime_declarations());

        // 添加外部库声明
        ir.push_str(&self.generate_external_lib_declarations(&module.header.external_libs));

        // 添加类型定义
        for type_def in &module.type_definitions {
            ir.push_str(&self.generate_type_definition(type_def, &module.constant_pool)?);
        }

        // 添加全局变量
        for global in &module.global_variables {
            ir.push_str(&self.generate_global_variable(global, &module.constant_pool)?);
        }

        // 添加函数定义
        for func in &module.functions {
            ir.push_str(&self.generate_function(func, &module.constant_pool)?);
        }

        // 添加类方法定义
        for type_def in &module.type_definitions {
            for method in &type_def.methods {
                if let Some(ref body) = method.body {
                    ir.push_str(&self.generate_method(type_def, method, body, &module.constant_pool)?);
                }
            }
        }

        Ok(ir)
    }

    /// 生成目标平台声明
    fn generate_target_declarations(&self) -> &'static str {
        r#"; Target declarations
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-gnu"

"#
    }

    /// 生成运行时声明
    fn generate_runtime_declarations(&self) -> &'static str {
        r#"; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @scanf(i8*, ...)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i8* @memcpy(i8*, i8*, i64)
declare i8* @memset(i8*, i32, i64)

; String runtime functions
declare i8* @cavvy_string_concat(i8*, i8*)
declare i8* @cavvy_string_substring(i8*, i32, i32)
declare i32 @cavvy_string_length(i8*)
declare i32 @cavvy_string_indexof(i8*, i8*)
declare i8* @cavvy_string_replace(i8*, i8*, i8*)
declare i8 @cavvy_string_charat(i8*, i32)

; Array runtime functions
declare i8* @cavvy_array_new(i32, i32)
declare i32 @cavvy_array_length(i8*)
declare i8* @cavvy_array_get(i8*, i32)
declare void @cavvy_array_set(i8*, i32, i8*)

"#
    }

    /// 生成外部库声明
    fn generate_external_lib_declarations(&self, libs: &[String]) -> String {
        let mut decls = String::new();
        for lib in libs {
            decls.push_str(&format!("; External library: {}\n", lib));
            // 这里可以根据库名添加特定的声明
            match lib.as_str() {
                "user32" => {
                    decls.push_str("declare i32 @MessageBoxA(i8*, i8*, i8*, i32)\n");
                }
                "kernel32" => {
                    decls.push_str("declare i32 @GetLastError()\n");
                }
                _ => {}
            }
        }
        decls.push('\n');
        decls
    }

    /// 生成类型定义
    fn generate_type_definition(&self, type_def: &TypeDefinition, pool: &ConstantPool) -> Result<String, JitError> {
        let name = pool.get_string(type_def.name_index)
            .ok_or_else(|| JitError::InvalidBytecode("Invalid type name index".to_string()))?;

        let mut ir = format!("; Type definition: {}\n", name);

        // 生成结构体类型
        ir.push_str(&format!("%struct.{} = type {{\n", name));

        // vtable指针（如果是类）
        if !type_def.modifiers.is_interface {
            ir.push_str("  %struct.vtable*,\n");
        }

        // 字段
        for field in &type_def.fields {
            let field_type = self.get_llvm_type_string(field.type_index, pool)?;
            ir.push_str(&format!("  {},\n", field_type));
        }

        ir.push_str("}\n\n");

        // 生成vtable类型
        if !type_def.modifiers.is_interface {
            ir.push_str(&format!("%struct.{}.vtable = type {{\n", name));
            for method in &type_def.methods {
                let method_sig = self.get_method_signature(method, pool)?;
                ir.push_str(&format!("  {},\n", method_sig));
            }
            ir.push_str("}\n\n");
        }

        Ok(ir)
    }

    /// 生成全局变量
    fn generate_global_variable(&self, global: &GlobalVariable, pool: &ConstantPool) -> Result<String, JitError> {
        let name = pool.get_string(global.name_index)
            .ok_or_else(|| JitError::InvalidBytecode("Invalid global name index".to_string()))?;
        let llvm_type = self.get_llvm_type_string(global.type_index, pool)?;

        let mut ir = format!("@{} = ", name);

        if global.modifiers.is_static {
            ir.push_str("internal ");
        }

        ir.push_str("global ");
        ir.push_str(&llvm_type);

        // 初始值
        if let Some(init_index) = global.initial_value {
            let init_val = self.get_constant_value(init_index, pool)?;
            ir.push_str(&format!(" {}", init_val));
        } else {
            ir.push_str(" zeroinitializer");
        }

        ir.push_str(", align 8\n");
        Ok(ir)
    }

    /// 生成函数
    fn generate_function(&self, func: &FunctionDefinition, pool: &ConstantPool) -> Result<String, JitError> {
        let name = pool.get_string(func.name_index)
            .ok_or_else(|| JitError::InvalidBytecode("Invalid function name index".to_string()))?;
        let ret_type = self.get_llvm_type_string(func.return_type_index, pool)?;

        let mut ir = format!("; Function: {}\n", name);
        ir.push_str(&format!("define {} @{}(", ret_type, name));

        // 参数
        for (i, param_type_idx) in func.param_type_indices.iter().enumerate() {
            if i > 0 {
                ir.push_str(", ");
            }
            let param_type = self.get_llvm_type_string(*param_type_idx, pool)?;
            ir.push_str(&format!("{} %arg{}", param_type, i));
        }

        ir.push_str(") {\n");

        // 函数体
        ir.push_str(&self.generate_code_body(&func.body, pool)?);

        ir.push_str("}\n\n");
        Ok(ir)
    }

    /// 生成方法
    fn generate_method(&self, type_def: &TypeDefinition, method: &MethodDefinition, body: &CodeBody, pool: &ConstantPool) -> Result<String, JitError> {
        let type_name = pool.get_string(type_def.name_index)
            .ok_or_else(|| JitError::InvalidBytecode("Invalid type name index".to_string()))?;
        let method_name = pool.get_string(method.name_index)
            .ok_or_else(|| JitError::InvalidBytecode("Invalid method name index".to_string()))?;
        let ret_type = self.get_llvm_type_string(method.return_type_index, pool)?;

        let mut ir = format!("; Method: {}.{}\n", type_name, method_name);

        let linkage = if method.modifiers.is_static {
            "internal"
        } else {
            "define"
        };

        ir.push_str(&format!("{} {} @{}_{}(", linkage, ret_type, type_name, method_name));

        // this指针（如果不是静态方法）
        if !method.modifiers.is_static {
            ir.push_str(&format!("%struct.{}* %this", type_name));
            if !method.param_type_indices.is_empty() {
                ir.push_str(", ");
            }
        }

        // 参数
        for (i, param_type_idx) in method.param_type_indices.iter().enumerate() {
            if i > 0 {
                ir.push_str(", ");
            }
            let param_type = self.get_llvm_type_string(*param_type_idx, pool)?;
            ir.push_str(&format!("{} %arg{}", param_type, i));
        }

        ir.push_str(") {\n");

        // 函数体
        ir.push_str(&self.generate_code_body(body, pool)?);

        ir.push_str("}\n\n");
        Ok(ir)
    }

    /// 生成代码体
    fn generate_code_body(&self, body: &CodeBody, pool: &ConstantPool) -> Result<String, JitError> {
        let mut ir = String::new();
        ir.push_str("entry:\n");

        let mut temp_counter: u32 = 0;

        for instr in &body.instructions {
            let instr_ir = self.generate_instruction(instr, pool, &mut temp_counter)?;
            ir.push_str(&instr_ir);
        }

        // 确保有返回指令
        ir.push_str("  ret void\n");

        Ok(ir)
    }

    /// 生成单条指令
    fn generate_instruction(&self, instr: &Instruction, pool: &ConstantPool, temp_counter: &mut u32) -> Result<String, JitError> {
        let mut ir = String::new();

        match instr.opcode {
            Opcode::Ldc => {
                let index = u16::from_le_bytes([instr.operands[0], instr.operands[1]]);
                let val = self.get_constant_value(index, pool)?;
                ir.push_str(&format!("  %t{} = add i32 {}, 0\n", temp_counter, val));
                *temp_counter += 1;
            }
            Opcode::Iconst0 => {
                ir.push_str(&format!("  %t{} = add i32 0, 0\n", temp_counter));
                *temp_counter += 1;
            }
            Opcode::Iconst1 => {
                ir.push_str(&format!("  %t{} = add i32 1, 0\n", temp_counter));
                *temp_counter += 1;
            }
            Opcode::Iadd => {
                ir.push_str(&format!("  %t{} = add i32 %t{}, %t{}\n",
                    temp_counter, *temp_counter - 2, *temp_counter - 1));
                *temp_counter += 1;
            }
            Opcode::Isub => {
                ir.push_str(&format!("  %t{} = sub i32 %t{}, %t{}\n",
                    temp_counter, *temp_counter - 2, *temp_counter - 1));
                *temp_counter += 1;
            }
            Opcode::Imul => {
                ir.push_str(&format!("  %t{} = mul i32 %t{}, %t{}\n",
                    temp_counter, *temp_counter - 2, *temp_counter - 1));
                *temp_counter += 1;
            }
            Opcode::Idiv => {
                ir.push_str(&format!("  %t{} = sdiv i32 %t{}, %t{}\n",
                    temp_counter, *temp_counter - 2, *temp_counter - 1));
                *temp_counter += 1;
            }
            Opcode::Irem => {
                ir.push_str(&format!("  %t{} = srem i32 %t{}, %t{}\n",
                    temp_counter, *temp_counter - 2, *temp_counter - 1));
                *temp_counter += 1;
            }
            Opcode::Return => {
                ir.push_str("  ret void\n");
            }
            Opcode::Ireturn => {
                ir.push_str(&format!("  ret i32 %t{}\n", *temp_counter - 1));
            }
            _ => {
                // 其他指令简化为注释
                ir.push_str(&format!("  ; Unhandled opcode: {:?}\n", instr.opcode));
            }
        }

        Ok(ir)
    }

    /// 获取LLVM类型字符串
    fn get_llvm_type_string(&self, type_index: ConstantIndex, pool: &ConstantPool) -> Result<String, JitError> {
        // 这里简化处理，实际应该根据类型索引查找类型
        // 返回LLVM IR类型字符串
        match type_index {
            1 => Ok("i32".to_string()),      // int
            2 => Ok("i64".to_string()),      // long
            3 => Ok("float".to_string()),    // float
            4 => Ok("double".to_string()),   // double
            5 => Ok("i1".to_string()),       // boolean
            6 => Ok("i8".to_string()),       // char
            7 => Ok("i8*".to_string()),      // String
            8 => Ok("void".to_string()),     // void
            _ => Ok("i8*".to_string()),      // 默认为指针类型
        }
    }

    /// 获取方法签名
    fn get_method_signature(&self, method: &MethodDefinition, pool: &ConstantPool) -> Result<String, JitError> {
        let ret_type = self.get_llvm_type_string(method.return_type_index, pool)?;
        let mut sig = format!("{} (", ret_type);

        for (i, param_type_idx) in method.param_type_indices.iter().enumerate() {
            if i > 0 {
                sig.push_str(", ");
            }
            let param_type = self.get_llvm_type_string(*param_type_idx, pool)?;
            sig.push_str(&param_type);
        }

        sig.push(')');
        Ok(sig)
    }

    /// 获取常量值
    fn get_constant_value(&self, index: ConstantIndex, pool: &ConstantPool) -> Result<String, JitError> {
        if let Some(val) = pool.get_integer(index) {
            Ok(val.to_string())
        } else if let Some(val) = pool.get_long(index) {
            Ok(val.to_string())
        } else if let Some(val) = pool.get_float(index) {
            Ok(format!("0x{:x}", val.to_bits()))
        } else if let Some(val) = pool.get_double(index) {
            Ok(format!("0x{:x}", val.to_bits()))
        } else if let Some(val) = pool.get_string(index) {
            // 转义字符串
            let escaped = val.replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\0A")
                .replace("\r", "\\0D")
                .replace("\t", "\\09");
            Ok(format!("c\"{}\\00\"", escaped))
        } else {
            Ok("0".to_string())
        }
    }

    /// 编译IR为对象文件
    fn compile_ir_to_object(&self, ir_file: &std::path::Path, obj_file: &std::path::Path) -> Result<(), JitError> {
        let clang = find_clang()?;

        let mut cmd = std::process::Command::new(&clang);
        cmd.arg("-c")
            .arg("-x")
            .arg("ir")
            .arg(ir_file)
            .arg(&self.options.optimization)
            .arg("-o")
            .arg(obj_file);

        let output = cmd.output()
            .map_err(|e| JitError::CompilationError(format!("Failed to run clang: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(JitError::CompilationError(stderr.to_string()));
        }

        Ok(())
    }

    /// 链接可执行文件
    fn link_executable(&self, obj_file: &std::path::Path, output_path: &str) -> Result<(), JitError> {
        let clang = find_clang()?;

        let mut cmd = std::process::Command::new(&clang);
        cmd.arg(obj_file)
            .arg(&self.options.optimization)
            .arg("-o")
            .arg(output_path);

        // 添加库搜索路径
        for path in &self.options.lib_paths {
            cmd.arg("-L").arg(path);
        }

        // 添加链接的库
        for lib in &self.options.link_libs {
            cmd.arg(format!("-l{}", lib));
        }

        // 平台特定的库
        if self.options.target.contains("windows") || self.options.target.contains("mingw") {
            // Windows不需要额外的库
        } else {
            // Linux/macOS
            cmd.arg("-lm"); // 数学库
        }

        let output = cmd.output()
            .map_err(|e| JitError::LinkingError(format!("Failed to run linker: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(JitError::LinkingError(stderr.to_string()));
        }

        Ok(())
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new(JitOptions::default())
    }
}

/// 查找clang编译器
fn find_clang() -> Result<std::path::PathBuf, JitError> {
    // 1. 尝试系统PATH中的clang
    if let Ok(output) = std::process::Command::new("clang").arg("--version").output() {
        if output.status.success() {
            return Ok(std::path::PathBuf::from("clang"));
        }
    }

    // 2. 尝试编译器所在目录下的llvm-minimal
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let bundled_clang = exe_dir.join("llvm-minimal/bin/clang.exe");
            if bundled_clang.exists() {
                return Ok(bundled_clang);
            }
        }
    }

    Err(JitError::CompilationError(
        "找不到clang编译器。请确保clang已安装并在PATH中。".to_string()
    ))
}

/// 便捷函数：编译字节码文件为可执行文件
pub fn compile_bytecode_file(input_path: &str, output_path: &str) -> Result<(), JitError> {
    // 1. 读取字节码文件
    let module = serializer::deserialize_from_file(input_path)
        .map_err(|e| JitError::InvalidBytecode(e.to_string()))?;

    // 2. 编译
    let compiler = JitCompiler::new(JitOptions::default());
    compiler.compile_to_executable(&module, output_path)
}
