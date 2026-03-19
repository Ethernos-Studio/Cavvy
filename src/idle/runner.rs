//! 编译运行模块
//!
//! 集成cay-run功能，提供一键编译运行

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

pub type CompileResult = Result<String, String>;

/// 代码运行器
pub struct CodeRunner {
    /// 是否正在运行
    is_running: bool,
    /// 输出缓冲区
    output_buffer: String,
    /// 进程句柄
    child_process: Option<std::process::Child>,
}

impl CodeRunner {
    pub fn new() -> Self {
        Self {
            is_running: false,
            output_buffer: String::new(),
            child_process: None,
        }
    }
    
    /// 异步编译并运行
    pub fn compile_and_run_async(&mut self, file_path: PathBuf, sender: Sender<CompileResult>) {
        if self.is_running {
            let _ = sender.send(Err("已有编译任务正在运行".to_string()));
            return;
        }
        
        self.is_running = true;
        self.output_buffer.clear();
        
        thread::spawn(move || {
            let result = Self::compile_and_run(&file_path);
            let _ = sender.send(result);
        });
    }
    
    /// 异步仅编译
    pub fn compile_only_async(&mut self, file_path: PathBuf, sender: Sender<CompileResult>) {
        if self.is_running {
            let _ = sender.send(Err("已有编译任务正在运行".to_string()));
            return;
        }
        
        self.is_running = true;
        
        thread::spawn(move || {
            let result = Self::compile_only(&file_path);
            let _ = sender.send(result);
        });
    }
    
    /// 编译并运行（同步）
    fn compile_and_run(file_path: &PathBuf) -> CompileResult {
        let output_dir = std::env::temp_dir();
        let exe_name = if cfg!(windows) {
            "cay_run_output.exe"
        } else {
            "cay_run_output"
        };
        let exe_path = output_dir.join(exe_name);
        
        // 第一步：编译为LLVM IR
        let ir_path = output_dir.join("cay_run_output.ll");
        
        // 调用cay-ir生成IR
        let mut cmd = Command::new("cay-ir");
        cmd.arg(file_path)
            .arg("-o")
            .arg(&ir_path);
        
        let output = cmd.output()
            .map_err(|e| format!("无法启动cay-ir: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("编译错误:\n{}", stderr));
        }
        
        // 第二步：IR编译为可执行文件
        let mut cmd = Command::new("ir2exe");
        cmd.arg(&ir_path)
            .arg("-o")
            .arg(&exe_path);
        
        let output = cmd.output()
            .map_err(|e| format!("无法启动ir2exe: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("链接错误:\n{}", stderr));
        }
        
        // 第三步：运行可执行文件
        let output = Command::new(&exe_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("无法运行程序: {}", e))?;
        
        let mut result = String::new();
        result.push_str("=== 编译成功 ===\n");
        result.push_str(&String::from_utf8_lossy(&output.stdout));
        
        if !output.stderr.is_empty() {
            result.push_str("\n=== 标准错误 ===\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }
        
        result.push_str(&format!("\n=== 退出码: {} ===", output.status.code().unwrap_or(-1)));
        
        // 清理临时文件
        let _ = std::fs::remove_file(&ir_path);
        let _ = std::fs::remove_file(&exe_path);
        
        Ok(result)
    }
    
    /// 仅编译（同步）
    fn compile_only(file_path: &PathBuf) -> CompileResult {
        let output_dir = std::env::temp_dir();
        let exe_name = if cfg!(windows) {
            "cay_run_output.exe"
        } else {
            "cay_run_output"
        };
        let exe_path = output_dir.join(exe_name);
        
        // 第一步：编译为LLVM IR
        let ir_path = output_dir.join("cay_run_output.ll");
        
        let mut cmd = Command::new("cay-ir");
        cmd.arg(file_path)
            .arg("-o")
            .arg(&ir_path);
        
        let output = cmd.output()
            .map_err(|e| format!("无法启动cay-ir: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("编译错误:\n{}", stderr));
        }
        
        // 第二步：IR编译为可执行文件
        let mut cmd = Command::new("ir2exe");
        cmd.arg(&ir_path)
            .arg("-o")
            .arg(&exe_path);
        
        let output = cmd.output()
            .map_err(|e| format!("无法启动ir2exe: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("链接错误:\n{}", stderr));
        }
        
        // 清理临时文件
        let _ = std::fs::remove_file(&ir_path);
        let _ = std::fs::remove_file(&exe_path);
        
        Ok("编译成功！".to_string())
    }
    
    /// 使用cay-run直接运行
    pub fn run_with_cay_run(file_path: &PathBuf) -> CompileResult {
        let output = Command::new("cay-run")
            .arg(file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("无法启动cay-run: {}", e))?;
        
        let mut result = String::new();
        result.push_str(&String::from_utf8_lossy(&output.stdout));
        
        if !output.stderr.is_empty() {
            result.push_str("\n=== 错误输出 ===\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }
        
        if !output.status.success() {
            return Err(result);
        }
        
        Ok(result)
    }
    
    /// 停止当前运行的进程
    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.child_process {
            let _ = child.kill();
            self.child_process = None;
        }
        self.is_running = false;
    }
    
    /// 是否正在运行
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    /// 标记完成
    pub fn mark_done(&mut self) {
        self.is_running = false;
    }
    
    /// 获取输出缓冲区
    pub fn get_output(&self) -> &str {
        &self.output_buffer
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
    }
    
    /// 追加输出
    pub fn append_output(&mut self, text: &str) {
        self.output_buffer.push_str(text);
    }
}

/// 编译选项
#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub optimize_level: String,
    pub output_path: Option<PathBuf>,
    pub link_libs: Vec<String>,
    pub lib_paths: Vec<String>,
    pub keep_temp: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            optimize_level: "-O2".to_string(),
            output_path: None,
            link_libs: Vec::new(),
            lib_paths: Vec::new(),
            keep_temp: false,
        }
    }
}
