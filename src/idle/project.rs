//! 项目管理模块

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

/// 项目配置
#[derive(Clone, Debug)]
pub struct Project {
    /// 项目名称
    pub name: String,
    /// 项目路径
    pub path: Option<PathBuf>,
    /// 当前打开的文件
    pub current_file: Option<PathBuf>,
    /// 最近打开的文件列表
    pub recent_files: VecDeque<PathBuf>,
    /// 打开的文件列表
    pub open_files: Vec<PathBuf>,
    /// 项目设置
    pub settings: ProjectSettings,
}

/// 项目设置
#[derive(Clone, Debug)]
pub struct ProjectSettings {
    /// 编译优化级别
    pub optimize_level: String,
    /// 输出目录
    pub output_dir: String,
    /// 编译选项
    pub compiler_flags: Vec<String>,
    /// 链接库
    pub link_libs: Vec<String>,
    /// 库搜索路径
    pub lib_paths: Vec<String>,
    /// 自动保存
    pub auto_save: bool,
    /// 自动检查
    pub auto_check: bool,
    /// 编辑器字体大小
    pub font_size: f32,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            optimize_level: "-O2".to_string(),
            output_dir: "build".to_string(),
            compiler_flags: Vec::new(),
            link_libs: Vec::new(),
            lib_paths: Vec::new(),
            auto_save: false,
            auto_check: true,
            font_size: 14.0,
        }
    }
}

impl Project {
    /// 创建新项目
    pub fn new() -> Self {
        Self {
            name: "Untitled".to_string(),
            path: None,
            current_file: None,
            recent_files: VecDeque::with_capacity(10),
            open_files: Vec::new(),
            settings: ProjectSettings::default(),
        }
    }
    
    /// 从目录打开项目
    pub fn open_from_directory(path: &Path) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        Self {
            name,
            path: Some(path.to_path_buf()),
            current_file: None,
            recent_files: VecDeque::with_capacity(10),
            open_files: Vec::new(),
            settings: ProjectSettings::default(),
        }
    }
    
    /// 添加最近文件
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // 移除已存在的相同路径
        self.recent_files.retain(|p| p != &path);
        
        // 添加到队首
        self.recent_files.push_front(path);
        
        // 限制数量
        while self.recent_files.len() > 10 {
            self.recent_files.pop_back();
        }
    }
    
    /// 获取最近文件列表
    pub fn get_recent_files(&self) -> &std::collections::VecDeque<PathBuf> {
        &self.recent_files
    }
    
    /// 打开文件
    pub fn open_file(&mut self, path: PathBuf) {
        if !self.open_files.contains(&path) {
            self.open_files.push(path.clone());
        }
        self.current_file = Some(path.clone());
        self.add_recent_file(path);
    }
    
    /// 关闭文件
    pub fn close_file(&mut self, path: &Path) {
        self.open_files.retain(|p| p != path);
        
        let path_buf = path.to_path_buf();
        if self.current_file.as_ref() == Some(&path_buf) {
            self.current_file = self.open_files.last().cloned();
        }
    }
    
    /// 获取打开的文件列表
    pub fn get_open_files(&self) -> &[PathBuf] {
        &self.open_files
    }
    
    /// 获取当前文件
    pub fn get_current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    }
    
    /// 设置当前文件
    pub fn set_current_file(&mut self, path: PathBuf) {
        if self.open_files.contains(&path) {
            self.current_file = Some(path);
        }
    }
    
    /// 是否有打开的文件
    pub fn has_open_file(&self) -> bool {
        self.current_file.is_some()
    }
    
    /// 保存项目配置
    pub fn save_settings(&self) -> Result<(), String> {
        if let Some(ref project_path) = self.path {
            let config_path = project_path.join("cay-idle.toml");
            let config = self.generate_config_toml();
            std::fs::write(&config_path, config)
                .map_err(|e| format!("无法保存项目配置: {}", e))?;
        }
        Ok(())
    }
    
    /// 加载项目配置
    pub fn load_settings(&mut self) -> Result<(), String> {
        if let Some(ref project_path) = self.path {
            let config_path = project_path.join("cay-idle.toml");
            if config_path.exists() {
                let content = std::fs::read_to_string(&config_path)
                    .map_err(|e| format!("无法读取项目配置: {}", e))?;
                self.parse_config_toml(&content)?;
            }
        }
        Ok(())
    }
    
    /// 生成配置TOML
    fn generate_config_toml(&self) -> String {
        format!(r#"[project]
name = "{}"

[settings]
optimize_level = "{}"
output_dir = "{}"
auto_save = {}
auto_check = {}
font_size = {}

[recent_files]
paths = [{}]
"#,
            self.name,
            self.settings.optimize_level,
            self.settings.output_dir,
            self.settings.auto_save,
            self.settings.auto_check,
            self.settings.font_size,
            self.recent_files.iter()
                .map(|p| format!("\"{}\"", p.display()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
    
    /// 解析配置TOML（简化版）
    fn parse_config_toml(&mut self, content: &str) -> Result<(), String> {
        // 简单的键值对解析
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("optimize_level") {
                if let Some(value) = line.split('=').nth(1) {
                    self.settings.optimize_level = value.trim()
                        .trim_matches('"')
                        .to_string();
                }
            } else if line.starts_with("output_dir") {
                if let Some(value) = line.split('=').nth(1) {
                    self.settings.output_dir = value.trim()
                        .trim_matches('"')
                        .to_string();
                }
            } else if line.starts_with("auto_save") {
                if let Some(value) = line.split('=').nth(1) {
                    self.settings.auto_save = value.trim() == "true";
                }
            } else if line.starts_with("auto_check") {
                if let Some(value) = line.split('=').nth(1) {
                    self.settings.auto_check = value.trim() == "true";
                }
            } else if line.starts_with("font_size") {
                if let Some(value) = line.split('=').nth(1) {
                    if let Ok(size) = value.trim().parse::<f32>() {
                        self.settings.font_size = size;
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// 文件类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileType {
    Source,
    Header,
    Project,
    Other,
}

impl FileType {
    /// 从路径判断文件类型
    pub fn from_path(path: &Path) -> Self {
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("cay") => FileType::Source,
                Some("cayh") => FileType::Header,
                Some("toml") => FileType::Project,
                _ => FileType::Other,
            }
        } else {
            FileType::Other
        }
    }
    
    /// 获取文件图标
    pub fn icon(&self) -> &'static str {
        match self {
            FileType::Source => "📄",
            FileType::Header => "📋",
            FileType::Project => "⚙️",
            FileType::Other => "📎",
        }
    }
}
