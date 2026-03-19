//! 字体管理模块
//!
//! 提供系统字体自动识别和字体设置功能：
//! - 自动检测系统中可用的字体
//! - 支持界面字体和编辑器字体分别设置
//! - 默认字体优先级规则

use std::collections::HashSet;

/// 字体配置
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// 界面字体（用于UI元素）
    pub ui_font: String,
    /// 界面字体备选
    pub ui_font_fallback: Vec<String>,
    /// 编辑器字体（用于代码编辑）
    pub editor_font: String,
    /// 编辑器字体备选
    pub editor_font_fallback: Vec<String>,
    /// 界面字体大小
    pub ui_font_size: f32,
    /// 编辑器字体大小
    pub editor_font_size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        // 编辑器字体回退链：优先使用等宽字体，但中文字符会回退到中文字体
        // egui 会按顺序尝试每个字体，直到找到包含该字符的字体
        let editor_fallback = vec![
            // 英文字体（等宽）
            "JetBrains Mono".to_string(),
            "Consolas".to_string(),
            "Monaco".to_string(),
            "Courier New".to_string(),
            // 中文字体（用于中文回退）
            "Microsoft YaHei".to_string(),
            "Microsoft YaHei UI".to_string(),
            "SimHei".to_string(),
            "SimSun".to_string(),
            "Noto Sans CJK SC".to_string(),
            "Source Han Sans SC".to_string(),
            "PingFang SC".to_string(),
            "Hiragino Sans GB".to_string(),
            // 最终回退
            "monospace".to_string(),
        ];

        Self {
            ui_font: "Microsoft YaHei".to_string(),
            ui_font_fallback: vec![
                "Segoe UI".to_string(),
                "Arial".to_string(),
                "Helvetica".to_string(),
                "sans-serif".to_string(),
            ],
            editor_font: "JetBrains Mono".to_string(),
            editor_font_fallback: editor_fallback,
            ui_font_size: 14.0,
            editor_font_size: 14.0,
        }
    }
}

impl FontConfig {
    /// 创建新的字体配置，自动检测系统可用字体
    pub fn new_with_system_detection() -> Self {
        let mut config = Self::default();
        config.detect_system_fonts();
        config
    }

    /// 检测系统可用字体并调整配置
    fn detect_system_fonts(&mut self) {
        let available_fonts = Self::get_available_fonts();

        // 检测界面字体（中文优先）
        let ui_candidates = vec![
            "Microsoft YaHei UI",
            "Microsoft YaHei",
            "PingFang SC",
            "Hiragino Sans GB",
            "WenQuanYi Micro Hei",
            "Noto Sans CJK SC",
            "Source Han Sans SC",
            "SimHei",
            "SimSun",
            "Segoe UI",
            "Arial",
            "Helvetica",
        ];

        for font in &ui_candidates {
            if available_fonts.contains(*font) {
                self.ui_font = font.to_string();
                break;
            }
        }

        // 检测编辑器字体（等宽字体，优先JetBrains Mono）
        let editor_candidates = vec![
            "JetBrains Mono",
            "Fira Code",
            "Cascadia Code",
            "Consolas",
            "Monaco",
            "Menlo",
            "DejaVu Sans Mono",
            "Source Code Pro",
            "Ubuntu Mono",
            "Courier New",
            "Courier",
        ];

        for font in &editor_candidates {
            if available_fonts.contains(*font) {
                self.editor_font = font.to_string();
                break;
            }
        }
    }

    /// 获取系统可用字体列表
    pub fn get_available_fonts() -> HashSet<String> {
        let mut fonts = HashSet::new();

        // Windows 字体检测 - 通过检查字体文件是否存在
        #[cfg(target_os = "windows")]
        {
            let font_files = [
                ("Microsoft YaHei UI", r"C:\Windows\Fonts\msyh.ttc"),
                ("Microsoft YaHei", r"C:\Windows\Fonts\msyh.ttc"),
                ("SimHei", r"C:\Windows\Fonts\simhei.ttf"),
                ("SimSun", r"C:\Windows\Fonts\simsun.ttc"),
                ("NSimSun", r"C:\Windows\Fonts\nsimsun.ttc"),
                ("FangSong", r"C:\Windows\Fonts\simfang.ttf"),
                ("KaiTi", r"C:\Windows\Fonts\simkai.ttf"),
                ("Consolas", r"C:\Windows\Fonts\consola.ttf"),
                ("Courier New", r"C:\Windows\Fonts\cour.ttf"),
                ("Arial", r"C:\Windows\Fonts\arial.ttf"),
                ("Segoe UI", r"C:\Windows\Fonts\segoeui.ttf"),
            ];

            for (name, path) in &font_files {
                if std::path::Path::new(path).exists() {
                    fonts.insert(name.to_string());
                }
            }

            // 检查用户字体目录中的 JetBrains Mono
            if let Ok(username) = std::env::var("USERNAME") {
                let user_font_path = format!(
                    r"C:\Users\{}\AppData\Local\Microsoft\Windows\Fonts\JetBrainsMono-Regular.ttf",
                    username
                );
                if std::path::Path::new(&user_font_path).exists() {
                    fonts.insert("JetBrains Mono".to_string());
                }
            }
        }

        // macOS 字体检测
        #[cfg(target_os = "macos")]
        {
            let common_macos_fonts = vec![
                "PingFang SC",
                "Hiragino Sans GB",
                "Heiti SC",
                "STHeiti",
                "Apple SD Gothic Neo",
                "Helvetica Neue",
                "Arial",
                "Menlo",
                "Monaco",
                "SF Mono",
                "JetBrains Mono",
                "Fira Code",
            ];
            for font in common_macos_fonts {
                fonts.insert(font.to_string());
            }
        }

        // Linux 字体检测
        #[cfg(target_os = "linux")]
        {
            // 尝试从系统获取字体列表
            if let Ok(output) = std::process::Command::new("fc-list")
                .args(&[":family"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let font = line.trim();
                    if !font.is_empty() {
                        fonts.insert(font.to_string());
                    }
                }
            }

            // 添加常见Linux字体
            let common_linux_fonts = vec![
                "Noto Sans CJK SC",
                "Source Han Sans SC",
                "WenQuanYi Micro Hei",
                "WenQuanYi Zen Hei",
                "DejaVu Sans",
                "Liberation Sans",
                "Ubuntu",
                "DejaVu Sans Mono",
                "Liberation Mono",
                "Ubuntu Mono",
                "JetBrains Mono",
                "Fira Code",
            ];
            for font in common_linux_fonts {
                fonts.insert(font.to_string());
            }
        }

        fonts
    }

    /// 获取界面字体族（用于egui）
    pub fn get_ui_font_family(&self) -> egui::FontFamily {
        egui::FontFamily::Proportional
    }

    /// 获取编辑器字体族（用于egui）
    pub fn get_editor_font_family(&self) -> egui::FontFamily {
        egui::FontFamily::Monospace
    }

    /// 获取界面字体ID
    pub fn get_ui_font_id(&self) -> egui::FontId {
        egui::FontId::new(self.ui_font_size, self.get_ui_font_family())
    }

    /// 获取编辑器字体ID
    pub fn get_editor_font_id(&self) -> egui::FontId {
        egui::FontId::new(self.editor_font_size, self.get_editor_font_family())
    }

    /// 设置界面字体
    pub fn set_ui_font(&mut self, font: &str) {
        self.ui_font = font.to_string();
    }

    /// 设置编辑器字体
    pub fn set_editor_font(&mut self, font: &str) {
        self.editor_font = font.to_string();
    }

    /// 设置界面字体大小
    pub fn set_ui_font_size(&mut self, size: f32) {
        self.ui_font_size = size.max(8.0).min(32.0);
    }

    /// 设置编辑器字体大小
    pub fn set_editor_font_size(&mut self, size: f32) {
        self.editor_font_size = size.max(8.0).min(32.0);
    }

    /// 获取字体CSS样式字符串（用于样式表）
    pub fn get_ui_font_css(&self) -> String {
        let fallbacks = self.ui_font_fallback.join(", ");
        format!("{} , {}", self.ui_font, fallbacks)
    }

    /// 获取编辑器字体CSS样式字符串
    pub fn get_editor_font_css(&self) -> String {
        let fallbacks = self.editor_font_fallback.join(", ");
        format!("{} , {}", self.editor_font, fallbacks)
    }
}

/// 字体设置对话框状态
#[derive(Debug, Default)]
pub struct FontSettingsState {
    pub show_dialog: bool,
    pub temp_ui_font: String,
    pub temp_editor_font: String,
    pub temp_ui_font_size: f32,
    pub temp_editor_font_size: f32,
}

impl FontSettingsState {
    pub fn new(config: &FontConfig) -> Self {
        Self {
            show_dialog: false,
            temp_ui_font: config.ui_font.clone(),
            temp_editor_font: config.editor_font.clone(),
            temp_ui_font_size: config.ui_font_size,
            temp_editor_font_size: config.editor_font_size,
        }
    }

    pub fn reset(&mut self, config: &FontConfig) {
        self.temp_ui_font = config.ui_font.clone();
        self.temp_editor_font = config.editor_font.clone();
        self.temp_ui_font_size = config.ui_font_size;
        self.temp_editor_font_size = config.editor_font_size;
    }

    pub fn apply(&self, config: &mut FontConfig) {
        config.set_ui_font(&self.temp_ui_font);
        config.set_editor_font(&self.temp_editor_font);
        config.set_ui_font_size(self.temp_ui_font_size);
        config.set_editor_font_size(self.temp_editor_font_size);
    }
}

/// 获取推荐的界面字体列表
pub fn get_recommended_ui_fonts() -> Vec<&'static str> {
    vec![
        "Microsoft YaHei UI",
        "Microsoft YaHei",
        "PingFang SC",
        "Hiragino Sans GB",
        "WenQuanYi Micro Hei",
        "Noto Sans CJK SC",
        "Source Han Sans SC",
        "SimHei",
        "SimSun",
        "Segoe UI",
        "Arial",
        "Helvetica",
    ]
}

/// 获取推荐的编辑器字体列表
pub fn get_recommended_editor_fonts() -> Vec<&'static str> {
    vec![
        "JetBrains Mono",
        "Fira Code",
        "Cascadia Code",
        "Consolas",
        "Monaco",
        "Menlo",
        "DejaVu Sans Mono",
        "Source Code Pro",
        "Ubuntu Mono",
        "Courier New",
        "Courier",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_config_default() {
        let config = FontConfig::default();
        assert!(!config.ui_font.is_empty());
        assert!(!config.editor_font.is_empty());
        assert!(config.ui_font_size > 0.0);
        assert!(config.editor_font_size > 0.0);
    }

    #[test]
    fn test_font_config_with_detection() {
        let config = FontConfig::new_with_system_detection();
        assert!(!config.ui_font.is_empty());
        assert!(!config.editor_font.is_empty());
    }

    #[test]
    fn test_font_size_limits() {
        let mut config = FontConfig::default();
        config.set_ui_font_size(5.0);
        assert_eq!(config.ui_font_size, 8.0); // 最小值限制

        config.set_ui_font_size(50.0);
        assert_eq!(config.ui_font_size, 32.0); // 最大值限制
    }
}
