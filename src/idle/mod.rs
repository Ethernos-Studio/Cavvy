//! cay-IDLE - Cavvy 集成开发环境
//!
//! 基于egui的轻量级IDE，提供：
//! - 代码编辑器（语法高亮、行号）
//! - 实时语法检查（基于cay-check）
//! - 一键编译运行
//! - 项目文件管理

pub mod editor;
pub mod project;
pub mod runner;
pub mod syntax_checker;
pub mod ui;
pub mod font_manager;
pub mod file_browser;

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

/// IDLE 应用程序
pub struct IdleApp {
    /// 当前打开的项目
    pub project: project::Project,
    /// 代码编辑器
    pub editor: editor::CodeEditor,
    /// 语法检查器
    pub syntax_checker: syntax_checker::SyntaxChecker,
    /// 运行器
    pub runner: runner::CodeRunner,
    /// UI状态
    pub ui_state: ui::UiState,
    /// 字体配置
    pub font_config: font_manager::FontConfig,
    /// 字体设置对话框状态
    pub font_settings_state: font_manager::FontSettingsState,
    /// 文件浏览器状态
    pub file_browser_state: file_browser::FileBrowserState,
    /// 编译输出通道
    compile_receiver: Option<Receiver<runner::CompileResult>>,
    /// 检查输出通道
    check_receiver: Option<Receiver<syntax_checker::CheckResult>>,
    /// 版本号
    pub version: String,
}

impl IdleApp {
    /// 创建新的IDLE实例
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 初始化字体配置（自动检测系统字体）
        let font_config = font_manager::FontConfig::new_with_system_detection();
        let font_settings_state = font_manager::FontSettingsState::new(&font_config);

        // 配置egui字体
        Self::configure_fonts(cc, &font_config);

        Self {
            project: project::Project::new(),
            editor: editor::CodeEditor::new(),
            syntax_checker: syntax_checker::SyntaxChecker::new(),
            runner: runner::CodeRunner::new(),
            ui_state: ui::UiState::default(),
            font_config,
            font_settings_state,
            file_browser_state: file_browser::FileBrowserState::default(),
            compile_receiver: None,
            check_receiver: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 配置egui字体
    fn configure_fonts(cc: &eframe::CreationContext<'_>, config: &font_manager::FontConfig) {
        let mut fonts = egui::FontDefinitions::default();

        // 获取egui默认字体族作为基础
        let default_proportional = fonts.families.get(&egui::FontFamily::Proportional).cloned()
            .unwrap_or_else(|| vec!["Ubuntu-Light".to_string(), "NotoSans-Regular".to_string()]);
        let default_monospace = fonts.families.get(&egui::FontFamily::Monospace).cloned()
            .unwrap_or_else(|| vec!["Hack".to_string(), "UbuntuMono".to_string()]);

        // 构建界面字体族（优先使用系统字体，回退到egui内置字体）
        let mut proportional_family: Vec<String> = Vec::new();

        // 添加检测到的系统字体
        let available_fonts = font_manager::FontConfig::get_available_fonts();

        // 界面字体候选（中文优先）
        let ui_candidates = vec![
            "Microsoft YaHei UI",
            "Microsoft YaHei",
            "Segoe UI",
            "Arial",
        ];

        for font in &ui_candidates {
            if available_fonts.contains(*font) {
                proportional_family.push(font.to_string());
            }
        }

        // 添加egui内置字体作为回退
        proportional_family.extend(default_proportional);

        // 构建编辑器字体族（中英混合）
        let mut monospace_family: Vec<String> = Vec::new();

        // 编辑器字体候选（等宽英文字体）
        let editor_candidates = vec![
            "JetBrains Mono",
            "Consolas",
            "Courier New",
        ];

        for font in &editor_candidates {
            if available_fonts.contains(*font) {
                monospace_family.push(font.to_string());
            }
        }

        // 添加中文字体作为回退（用于显示中文）
        let chinese_candidates = vec![
            "Microsoft YaHei",
            "Microsoft YaHei UI",
            "SimHei",
            "SimSun",
            "Noto Sans CJK SC",
            "Source Han Sans SC",
        ];

        for font in &chinese_candidates {
            if available_fonts.contains(*font) {
                monospace_family.push(font.to_string());
            }
        }

        // 添加egui内置字体作为最终回退
        monospace_family.extend(default_monospace);

        // 配置 Proportional 字体族
        fonts.families.insert(
            egui::FontFamily::Proportional,
            proportional_family,
        );

        // 配置 Monospace 字体族
        fonts.families.insert(
            egui::FontFamily::Monospace,
            monospace_family,
        );

        // 尝试加载系统字体文件
        #[cfg(target_os = "windows")]
        {
            Self::load_windows_fonts(&mut fonts);
        }

        cc.egui_ctx.set_fonts(fonts);
    }

    /// 加载Windows系统字体
    #[cfg(target_os = "windows")]
    fn load_windows_fonts(fonts: &mut egui::FontDefinitions) {
        use std::path::Path;

        let font_paths = [
            ("Microsoft YaHei", r"C:\Windows\Fonts\msyh.ttc"),
            ("Microsoft YaHei UI", r"C:\Windows\Fonts\msyh.ttc"),
            ("SimHei", r"C:\Windows\Fonts\simhei.ttf"),
            ("SimSun", r"C:\Windows\Fonts\simsun.ttc"),
            ("Consolas", r"C:\Windows\Fonts\consola.ttf"),
            ("Courier New", r"C:\Windows\Fonts\cour.ttf"),
            ("Arial", r"C:\Windows\Fonts\arial.ttf"),
            ("Segoe UI", r"C:\Windows\Fonts\segoeui.ttf"),
        ];

        for (name, path) in &font_paths {
            if Path::new(path).exists() {
                if let Ok(data) = std::fs::read(path) {
                    fonts
                        .font_data
                        .insert(name.to_string(), egui::FontData::from_owned(data));
                }
            }
        }

        // 尝试加载JetBrains Mono（如果已安装）
        let jetbrains_paths = [
            r"C:\Windows\Fonts\JetBrainsMono-Regular.ttf",
            r"C:\Users\%USERNAME%\AppData\Local\Microsoft\Windows\Fonts\JetBrainsMono-Regular.ttf",
        ];

        for path in &jetbrains_paths {
            let expanded_path = path.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default());
            if Path::new(&expanded_path).exists() {
                if let Ok(data) = std::fs::read(&expanded_path) {
                    fonts
                        .font_data
                        .insert("JetBrains Mono".to_string(), egui::FontData::from_owned(data));
                    break;
                }
            }
        }
    }

    /// 重新加载字体配置
    pub fn reload_fonts(&mut self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // 获取egui默认字体族作为基础
        let default_proportional = fonts.families.get(&egui::FontFamily::Proportional).cloned()
            .unwrap_or_else(|| vec!["Ubuntu-Light".to_string(), "NotoSans-Regular".to_string()]);
        let default_monospace = fonts.families.get(&egui::FontFamily::Monospace).cloned()
            .unwrap_or_else(|| vec!["Hack".to_string(), "UbuntuMono".to_string()]);

        // 构建界面字体族
        let mut proportional_family: Vec<String> = Vec::new();
        let available_fonts = font_manager::FontConfig::get_available_fonts();

        let ui_candidates = vec![
            "Microsoft YaHei UI",
            "Microsoft YaHei",
            "Segoe UI",
            "Arial",
        ];

        for font in &ui_candidates {
            if available_fonts.contains(*font) {
                proportional_family.push(font.to_string());
            }
        }
        proportional_family.extend(default_proportional);

        // 构建编辑器字体族（中英混合）
        let mut monospace_family: Vec<String> = Vec::new();
        let editor_candidates = vec![
            "JetBrains Mono",
            "Consolas",
            "Courier New",
        ];

        for font in &editor_candidates {
            if available_fonts.contains(*font) {
                monospace_family.push(font.to_string());
            }
        }

        // 添加中文字体作为回退
        let chinese_candidates = vec![
            "Microsoft YaHei",
            "Microsoft YaHei UI",
            "SimHei",
            "SimSun",
            "Noto Sans CJK SC",
            "Source Han Sans SC",
        ];

        for font in &chinese_candidates {
            if available_fonts.contains(*font) {
                monospace_family.push(font.to_string());
            }
        }

        monospace_family.extend(default_monospace);

        // 配置字体族
        fonts.families.insert(
            egui::FontFamily::Proportional,
            proportional_family,
        );
        fonts.families.insert(
            egui::FontFamily::Monospace,
            monospace_family,
        );

        #[cfg(target_os = "windows")]
        {
            Self::load_windows_fonts(&mut fonts);
        }

        ctx.set_fonts(fonts);
    }
    
    /// 打开文件
    pub fn open_file(&mut self, path: PathBuf) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            self.editor.set_text(&content);
            let path_display = path.display().to_string();
            self.project.current_file = Some(path.clone());
            self.project.add_recent_file(path.clone());
            self.project.open_file(path.clone());
            self.ui_state.status_message = format!("已打开: {}", path_display);
            
            // 如果文件浏览器没有打开文件夹，尝试打开文件所在目录
            if self.file_browser_state.root_path.is_none() {
                if let Some(parent) = path.parent() {
                    self.file_browser_state.open_folder(parent);
                }
            }
            
            // 选中当前文件
            self.file_browser_state.select_path(&path);
            self.file_browser_state.expand_to_path(&path);
        } else {
            self.ui_state.status_message = format!("无法打开文件: {}", path.display());
        }
    }
    
    /// 使用系统文件对话框打开文件
    pub fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Cavvy 源文件", &["cay"])
            .add_filter("Cavvy 头文件", &["cayh"])
            .add_filter("所有文件", &["*"])
            .pick_file()
        {
            self.open_file(path);
        }
    }
    
    /// 使用系统文件对话框打开文件夹
    pub fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.open_folder(&path);
        }
    }
    
    /// 打开文件夹
    pub fn open_folder(&mut self, path: &PathBuf) {
        self.file_browser_state.open_folder(path);
        self.project.name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        self.project.path = Some(path.clone());
        self.ui_state.status_message = format!("已打开文件夹: {}", path.display());
    }
    
    /// 使用系统文件对话框保存文件
    pub fn save_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Cavvy 源文件", &["cay"])
            .add_filter("Cavvy 头文件", &["cayh"])
            .set_file_name("untitled.cay")
            .save_file()
        {
            self.project.current_file = Some(path.clone());
            self.save_file();
            self.project.add_recent_file(path.clone());
            self.project.open_file(path);
        }
    }
    
    /// 保存当前文件
    pub fn save_file(&mut self) {
        if let Some(ref path) = self.project.current_file {
            let content = self.editor.get_text();
            if let Err(e) = std::fs::write(path, content) {
                self.ui_state.status_message = format!("保存失败: {}", e);
            } else {
                self.ui_state.status_message = format!("已保存: {}", path.display());
                self.editor.mark_saved();
            }
        } else {
            self.save_file_dialog();
        }
    }
    
    /// 运行语法检查
    pub fn check_syntax(&mut self) {
        let code = self.editor.get_text();
        let (sender, receiver) = channel();
        self.check_receiver = Some(receiver);
        self.syntax_checker.check_async(code, sender);
        self.ui_state.status_message = "正在检查语法...".to_string();
    }
    
    /// 编译并运行
    pub fn compile_and_run(&mut self) {
        // 先保存文件
        self.save_file();
        
        if let Some(ref path) = self.project.current_file {
            let (sender, receiver) = channel();
            self.compile_receiver = Some(receiver);
            self.runner.compile_and_run_async(path.clone(), sender);
            self.ui_state.status_message = "正在编译...".to_string();
            self.ui_state.console_visible = true;
        } else {
            self.ui_state.status_message = "请先保存文件".to_string();
        }
    }
    
    /// 仅编译
    pub fn compile_only(&mut self) {
        self.save_file();
        
        if let Some(ref path) = self.project.current_file {
            let (sender, receiver) = channel();
            self.compile_receiver = Some(receiver);
            self.runner.compile_only_async(path.clone(), sender);
            self.ui_state.status_message = "正在编译...".to_string();
            self.ui_state.console_visible = true;
        } else {
            self.ui_state.status_message = "请先保存文件".to_string();
        }
    }
    
    /// 处理异步结果
    fn handle_async_results(&mut self) {
        // 处理语法检查结果
        if let Some(ref receiver) = self.check_receiver {
            match receiver.try_recv() {
                Ok(result) => {
                    match result {
                        Ok(errors) => {
                            self.editor.set_diagnostics(errors.clone());
                            if errors.is_empty() {
                                self.ui_state.status_message = "语法检查通过".to_string();
                            } else {
                                let error_count = errors.iter().filter(|e| e.is_error).count();
                                let warning_count = errors.len() - error_count;
                                self.ui_state.status_message = 
                                    format!("发现 {} 个错误, {} 个警告", error_count, warning_count);
                            }
                        }
                        Err(e) => {
                            self.ui_state.status_message = format!("检查失败: {}", e);
                        }
                    }
                    self.check_receiver = None;
                    self.syntax_checker.mark_done(); // 重置检查状态
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // 通道断开，清理接收器
                    self.ui_state.status_message = "检查中断".to_string();
                    self.check_receiver = None;
                    self.syntax_checker.mark_done(); // 重置检查状态
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // 还没有结果，继续等待
                }
            }
        }
        
        // 处理编译结果
        if let Some(ref receiver) = self.compile_receiver {
            match receiver.try_recv() {
                Ok(result) => {
                    match result {
                        Ok(output) => {
                            self.ui_state.console_output.push_str(&output);
                            self.ui_state.console_output.push('\n');
                            self.ui_state.status_message = "编译完成".to_string();
                        }
                        Err(e) => {
                            self.ui_state.console_output.push_str(&format!("错误: {}\n", e));
                            self.ui_state.status_message = "编译失败".to_string();
                        }
                    }
                    self.compile_receiver = None;
                    self.runner.mark_done(); // 重置运行状态
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // 通道断开，清理接收器
                    self.ui_state.status_message = "编译中断".to_string();
                    self.compile_receiver = None;
                    self.runner.mark_done(); // 重置运行状态
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // 还没有结果，继续等待
                }
            }
        }
    }
    
    /// 处理控制台输入
    pub fn handle_console_input(&mut self, input: &str) {
        // 简单的命令处理
        match input.trim() {
            "help" | "?" => {
                self.ui_state.console_output.push_str("可用命令:\n");
                self.ui_state.console_output.push_str("  help, ? - 显示帮助\n");
                self.ui_state.console_output.push_str("  clear   - 清空控制台\n");
                self.ui_state.console_output.push_str("  version - 显示版本\n");
                self.ui_state.console_output.push_str("  status  - 显示当前状态\n");
            }
            "clear" | "cls" => {
                self.ui_state.console_output.clear();
            }
            "version" | "ver" => {
                self.ui_state.console_output.push_str(&format!("cay-IDLE 版本 {}\n", self.version));
            }
            "status" => {
                self.ui_state.console_output.push_str(&format!("状态: {}\n", self.ui_state.status_message));
                if let Some(ref path) = self.project.current_file {
                    self.ui_state.console_output.push_str(&format!("当前文件: {}\n", path.display()));
                }
            }
            _ => {
                self.ui_state.console_output.push_str(&format!("未知命令: {}\n", input));
                self.ui_state.console_output.push_str("输入 'help' 查看可用命令\n");
            }
        }
    }
}

impl eframe::App for IdleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理异步结果
        self.handle_async_results();
        
        // 顶部菜单栏
        ui::menu_bar(self, ctx);
        
        // 主界面布局
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::main_layout(self, ui);
        });
        
        // 底部状态栏
        ui::status_bar(self, ctx);
        
        // 模态对话框
        if self.ui_state.show_about_dialog {
            ui::about_dialog(self, ctx);
        }

        // 字体设置对话框
        ui::font_settings_dialog(self, ctx);

        // 持续刷新以处理异步结果
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}
