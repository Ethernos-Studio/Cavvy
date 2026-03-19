//! 代码编辑器模块

use egui::text::LayoutJob;
use std::collections::HashMap;

/// 诊断信息
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub is_error: bool,
}

/// 代码编辑器
pub struct CodeEditor {
    /// 编辑器内容
    text: String,
    /// 是否已修改
    modified: bool,
    /// 光标位置 (line, column)
    pub cursor_pos: (usize, usize),
    /// 诊断信息
    diagnostics: Vec<Diagnostic>,
    /// 选中区域
    selection: Option<(usize, usize)>,
    /// 滚动位置
    scroll_offset: f32,
    /// 行号宽度
    line_number_width: f32,
    /// 主题
    theme: EditorTheme,
}

/// 编辑器主题
#[derive(Clone, Copy)]
pub struct EditorTheme {
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub comment: egui::Color32,
    pub keyword: egui::Color32,
    pub string: egui::Color32,
    pub number: egui::Color32,
    pub function: egui::Color32,
    pub type_color: egui::Color32,
    pub line_number: egui::Color32,
    pub line_number_bg: egui::Color32,
    pub error_bg: egui::Color32,
    pub warning_bg: egui::Color32,
    pub selection: egui::Color32,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self {
            background: egui::Color32::from_rgb(30, 30, 30),
            foreground: egui::Color32::from_rgb(220, 220, 220),
            comment: egui::Color32::from_rgb(100, 150, 100),
            keyword: egui::Color32::from_rgb(200, 100, 200),
            string: egui::Color32::from_rgb(200, 150, 100),
            number: egui::Color32::from_rgb(100, 200, 200),
            function: egui::Color32::from_rgb(100, 150, 250),
            type_color: egui::Color32::from_rgb(100, 200, 150),
            line_number: egui::Color32::from_rgb(120, 120, 120),
            line_number_bg: egui::Color32::from_rgb(40, 40, 40),
            error_bg: egui::Color32::from_rgb(80, 30, 30),
            warning_bg: egui::Color32::from_rgb(80, 70, 30),
            selection: egui::Color32::from_rgb(60, 80, 120),
        }
    }
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            modified: false,
            cursor_pos: (0, 0),
            diagnostics: Vec::new(),
            selection: None,
            scroll_offset: 0.0,
            line_number_width: 60.0,
            theme: EditorTheme::default(),
        }
    }
    
    /// 设置文本内容
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.modified = false;
        self.diagnostics.clear();
    }
    
    /// 获取文本内容
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
    /// 标记为已保存
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }
    
    /// 是否已修改
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// 设置诊断信息
    pub fn set_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics = diagnostics;
    }
    
    /// 获取诊断信息
    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
    
    /// 获取行数
    pub fn line_count(&self) -> usize {
        self.text.lines().count().max(1)
    }
    
    /// 渲染编辑器
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let available_height = ui.available_height();

        // 计算行号宽度
        let line_count = self.line_count();
        let line_number_digits = line_count.to_string().len().max(2);
        self.line_number_width = (line_number_digits as f32 * 12.0 + 20.0).max(50.0);

        // 计算内容高度，添加底部留白（约3行的高度）
        let line_height = 16.0;
        let content_height = line_count as f32 * line_height + line_height * 3.0;

        let line_number_width = self.line_number_width;
        let editor_width = available_width - line_number_width - 5.0;

        // 使用单个 ScrollArea 包裹整个编辑器（行号+编辑区）
        egui::ScrollArea::vertical()
            .id_salt("editor_with_line_numbers")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // 设置内容区域大小
                ui.set_width(available_width);
                ui.set_height(content_height.max(available_height));

                let painter = ui.painter();
                let total_rect = ui.available_rect_before_wrap();

                // 行号区域矩形
                let line_number_rect = egui::Rect::from_min_size(
                    total_rect.min,
                    egui::vec2(line_number_width, content_height),
                );

                // 编辑器区域矩形
                let editor_rect = egui::Rect::from_min_size(
                    egui::pos2(total_rect.min.x + line_number_width, total_rect.min.y),
                    egui::vec2(editor_width, content_height),
                );

                // 绘制行号背景
                painter.rect_filled(line_number_rect, 0.0, self.theme.line_number_bg);

                // 绘制每个行号
                for line_num in 1..=line_count {
                    let line_has_error = self.diagnostics.iter()
                        .any(|d| d.line == line_num && d.is_error);
                    let line_has_warning = self.diagnostics.iter()
                        .any(|d| d.line == line_num && !d.is_error);

                    let text_color = if line_has_error {
                        egui::Color32::RED
                    } else if line_has_warning {
                        egui::Color32::YELLOW
                    } else {
                        self.theme.line_number
                    };

                    // 计算行号位置
                    let y = line_number_rect.top() + (line_num - 1) as f32 * line_height;

                    // 布局行号文本
                    let galley = painter.layout(
                        line_num.to_string(),
                        egui::FontId::monospace(14.0),
                        text_color,
                        line_number_width - 10.0,
                    );

                    // 右对齐绘制
                    let x = line_number_rect.right() - galley.size().x - 8.0;
                    let text_y = y + (line_height - galley.size().y) / 2.0;

                    painter.galley(
                        egui::pos2(x, text_y),
                        galley,
                        egui::Color32::PLACEHOLDER,
                    );
                }

                // 在编辑器区域创建子 UI 用于 TextEdit
                let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(editor_rect));
                self.render_editor_area(&mut child_ui, editor_width, content_height);
            });
    }

    /// 渲染编辑器区域
    fn render_editor_area(&mut self, ui: &mut egui::Ui, width: f32, content_height: f32) {
        // 设置 UI 的高度和宽度
        ui.set_height(content_height);
        ui.set_width(width);
        
        // 使用egui的TextEdit进行编辑
        let mut text = self.text.clone();

        let text_edit = egui::TextEdit::multiline(&mut text)
            .font(egui::TextStyle::Monospace)
            .text_color(self.theme.foreground)
            .desired_width(width)
            .lock_focus(true)
            .code_editor();

        let response = ui.add(text_edit);

        // 检测文本变化
        if response.changed() {
            self.text = text;
            self.modified = true;
        }

        // 显示诊断提示
        if response.hovered() {
            if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if let Some(line) = self.get_line_at_pos(pointer_pos, response.rect) {
                    if let Some(diag) = self.diagnostics.iter().find(|d| d.line == line) {
                        egui::show_tooltip(
                            ui.ctx(),
                            ui.layer_id(),
                            ui.id().with("diagnostic_tooltip"),
                            |ui| {
                                let color = if diag.is_error {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::YELLOW
                                };
                                ui.colored_label(color, &diag.message);
                            }
                        );
                    }
                }
            }
        }
    }
    
    /// 根据鼠标位置获取行号
    fn get_line_at_pos(&self, pos: egui::Pos2, rect: egui::Rect) -> Option<usize> {
        if !rect.contains(pos) {
            return None;
        }
        
        let relative_y = pos.y - rect.min.y;
        let line_height = 18.0; // 近似行高
        let line = (relative_y / line_height) as usize + 1;
        
        if line <= self.line_count() {
            Some(line)
        } else {
            None
        }
    }
    
    /// 获取编辑器字体ID（使用配置）
    pub fn get_editor_font_id(&self, font_config: &super::font_manager::FontConfig) -> egui::FontId {
        font_config.get_editor_font_id()
    }

    /// 获取界面字体ID（使用配置）
    pub fn get_ui_font_id(&self, font_config: &super::font_manager::FontConfig) -> egui::FontId {
        font_config.get_ui_font_id()
    }

    /// 简单的语法高亮（将代码转换为带颜色的LayoutJob）
    pub fn highlight_code(&self, code: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        
        // Cavvy关键字
        let keywords: Vec<&str> = vec![
            "public", "private", "protected", "static", "final", "abstract",
            "class", "interface", "extends", "implements", "void", "return",
            "if", "else", "while", "for", "do", "switch", "case", "default",
            "break", "continue", "new", "this", "super", "instanceof", "var", "let", "auto",
            "int", "long", "float", "double", "char", "boolean", "String",
        ];
        
        // 简单的基于行的解析
        for (line_idx, line) in code.lines().enumerate() {
            // 检查是否有诊断信息
            let has_error = self.diagnostics.iter().any(|d| d.line == line_idx + 1 && d.is_error);
            let has_warning = self.diagnostics.iter().any(|d| d.line == line_idx + 1 && !d.is_error);
            
            // 背景色
            let bg_color = if has_error {
                Some(self.theme.error_bg)
            } else if has_warning {
                Some(self.theme.warning_bg)
            } else {
                None
            };
            
            // 解析当前行
            let mut i = 0;
            let chars: Vec<char> = line.chars().collect();
            
            while i < chars.len() {
                let c = chars[i];
                
                // 跳过空白
                if c.is_whitespace() {
                    let start = i;
                    while i < chars.len() && chars[i].is_whitespace() {
                        i += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    job.append(&text, 0.0, self.default_format(bg_color));
                    continue;
                }
                
                // 字符串
                if c == '"' || c == '\'' {
                    let (text, consumed) = self.parse_string(&chars, i);
                    job.append(&text, 0.0, self.string_format(bg_color));
                    i += consumed;
                    continue;
                }
                
                // 注释
                if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    let text: String = chars[i..].iter().collect();
                    job.append(&text, 0.0, self.comment_format(bg_color));
                    break;
                }
                
                // 数字
                if c.is_ascii_digit() {
                    let start = i;
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == 'x' || chars[i] == 'b' || chars[i] == 'o' || chars[i] == '_') {
                        i += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    job.append(&text, 0.0, self.number_format(bg_color));
                    continue;
                }
                
                // 标识符/关键字
                if c.is_alphabetic() || c == '_' {
                    let start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    
                    if keywords.contains(&text.as_str()) {
                        job.append(&text, 0.0, self.keyword_format(bg_color));
                    } else {
                        // 检查是否是类型（大写开头）
                        if text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                            job.append(&text, 0.0, self.type_format(bg_color));
                        } else {
                            job.append(&text, 0.0, self.default_format(bg_color));
                        }
                    }
                    continue;
                }
                
                // 其他字符
                job.append(&c.to_string(), 0.0, self.default_format(bg_color));
                i += 1;
            }
            
            // 行尾
            job.append("\n", 0.0, self.default_format(None));
        }
        
        job
    }
    
    fn default_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.foreground,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn keyword_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.keyword,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn string_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.string,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn number_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.number,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn comment_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.comment,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn type_format(&self, bg: Option<egui::Color32>) -> egui::TextFormat {
        egui::TextFormat {
            font_id: egui::FontId::monospace(14.0),
            color: self.theme.type_color,
            background: bg.unwrap_or(self.theme.background),
            ..Default::default()
        }
    }
    
    fn parse_string(&self, chars: &[char], start: usize) -> (String, usize) {
        let quote = chars[start];
        let mut i = start + 1;
        let mut result = String::new();
        result.push(quote);
        
        while i < chars.len() {
            let c = chars[i];
            result.push(c);
            
            if c == quote {
                return (result, i - start + 1);
            }
            
            if c == '\\' && i + 1 < chars.len() {
                i += 1;
                result.push(chars[i]);
            }
            
            i += 1;
        }
        
        (result, i - start)
    }
}
