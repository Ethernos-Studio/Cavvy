//! UI组件模块

use eframe::egui;
use crate::idle::IdleApp;

/// UI状态
pub struct UiState {
    /// 状态栏消息
    pub status_message: String,
    /// 控制台是否可见
    pub console_visible: bool,
    /// 文件浏览器是否可见
    pub file_browser_visible: bool,
    /// 控制台输出
    pub console_output: String,
    /// 显示关于对话框
    pub show_about_dialog: bool,
    /// 选中的菜单项
    pub selected_menu: Option<String>,
    /// 侧边栏宽度
    pub sidebar_width: f32,
    /// 底部面板高度
    pub bottom_panel_height: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            status_message: "就绪".to_string(),
            console_visible: false,
            file_browser_visible: true,
            console_output: String::new(),
            show_about_dialog: false,
            selected_menu: None,
            sidebar_width: 250.0,
            bottom_panel_height: 150.0,
        }
    }
}

/// 顶部菜单栏
pub fn menu_bar(app: &mut IdleApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("新建文件 (Ctrl+N)").clicked() {
                    app.editor.set_text("");
                    app.project.current_file = None;
                    app.ui_state.status_message = "新文件".to_string();
                    ui.close_menu();
                }
                if ui.button("打开文件 (Ctrl+O)").clicked() {
                    app.open_file_dialog();
                    ui.close_menu();
                }
                if ui.button("打开文件夹").clicked() {
                    app.open_folder_dialog();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("保存 (Ctrl+S)").clicked() {
                    app.save_file();
                    ui.close_menu();
                }
                if ui.button("另存为...").clicked() {
                    app.save_file_dialog();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("退出").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    ui.close_menu();
                }
            });
            
            ui.menu_button("编辑", |ui| {
                if ui.button("撤销 (Ctrl+Z)").clicked() {
                    ui.close_menu();
                }
                if ui.button("重做 (Ctrl+Y)").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("剪切 (Ctrl+X)").clicked() {
                    ui.close_menu();
                }
                if ui.button("复制 (Ctrl+C)").clicked() {
                    ui.close_menu();
                }
                if ui.button("粘贴 (Ctrl+V)").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("查找 (Ctrl+F)").clicked() {
                    ui.close_menu();
                }
            });
            
            ui.menu_button("构建", |ui| {
                if ui.button("检查语法 (F5)").clicked() {
                    app.check_syntax();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("编译 (F7)").clicked() {
                    app.compile_only();
                    ui.close_menu();
                }
                if ui.button("编译并运行 (F9)").clicked() {
                    app.compile_and_run();
                    ui.close_menu();
                }
            });
            
            ui.menu_button("视图", |ui| {
                if ui.checkbox(&mut app.ui_state.file_browser_visible, "文件浏览器").clicked() {
                    ui.close_menu();
                }
                if ui.checkbox(&mut app.ui_state.console_visible, "控制台").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("字体设置...").clicked() {
                    app.font_settings_state.reset(&app.font_config);
                    app.font_settings_state.show_dialog = true;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("帮助", |ui| {
                if ui.button("关于 cay-IDLE").clicked() {
                    app.ui_state.show_about_dialog = true;
                    ui.close_menu();
                }
            });
        });
    });
}

/// 主布局
pub fn main_layout(app: &mut IdleApp, ui: &mut egui::Ui) {
    // 使用垂直分割来确保控制台不会遮挡编辑器
    // 首先渲染底部控制台（如果可见）
    if app.ui_state.console_visible {
        egui::TopBottomPanel::bottom("console")
            .resizable(true)
            .default_height(app.ui_state.bottom_panel_height)
            .height_range(50.0..=400.0)
            .show_inside(ui, |ui| {
                render_console(app, ui);
            });
    }
    
    // 左侧文件浏览器
    if app.ui_state.file_browser_visible {
        egui::SidePanel::left("file_browser")
            .resizable(true)
            .default_width(app.ui_state.sidebar_width)
            .width_range(150.0..=500.0)
            .show_inside(ui, |ui| {
                render_file_browser(app, ui);
            });
    }
    
    // 主编辑区（占据剩余空间）
    egui::CentralPanel::default().show_inside(ui, |ui| {
        // 编辑器标签页
        render_editor_tabs(app, ui);
        
        // 代码编辑器
        app.editor.ui(ui);
    });
}

/// 渲染文件浏览器
fn render_file_browser(app: &mut IdleApp, ui: &mut egui::Ui) {
    ui.heading("资源管理器");
    ui.separator();
    
    // 使用新的文件浏览器UI
    crate::idle::file_browser::render_file_browser_ui(
        ui,
        &mut app.file_browser_state,
    );
    
    // 处理待展开/折叠
    if let Some(path) = app.file_browser_state.take_pending_toggle() {
        app.file_browser_state.toggle_folder(&path);
    }
    
    // 处理待点击的文件
    if let Some(path) = app.file_browser_state.take_pending_file_click() {
        app.open_file(path);
    }
}

/// 渲染编辑器标签页
fn render_editor_tabs(app: &mut IdleApp, ui: &mut egui::Ui) {
    // 先复制到Vec避免borrow问题
    let open_files: Vec<_> = app.project.get_open_files().iter().cloned().collect();
    let is_empty = open_files.is_empty();
    
    ui.horizontal(|ui| {
        // 显示打开的文件标签
        for path in open_files {
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            
            let is_current = app.project.get_current_file() == Some(&path);
            let modified = app.editor.is_modified();
            
            let label = if modified && is_current {
                format!("{} ●", file_name)
            } else {
                file_name.to_string()
            };
            
            let button = ui.selectable_label(is_current, label);
            
            if button.clicked() {
                app.project.set_current_file(path.clone());
                // 重新加载文件内容
                if let Ok(content) = std::fs::read_to_string(&path) {
                    app.editor.set_text(&content);
                }
            }
            
            // 右键关闭
            button.context_menu(|ui| {
                if ui.button("关闭").clicked() {
                    app.project.close_file(&path);
                    ui.close_menu();
                }
            });
        }
        
        if is_empty {
            ui.label("无打开的文件");
        }
    });
    
    ui.separator();
}

/// 渲染控制台
fn render_console(app: &mut IdleApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading("控制台");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🗑️ 清空").clicked() {
                app.ui_state.console_output.clear();
            }
            if ui.button("⏹️ 停止").clicked() {
                app.runner.stop();
            }
        });
    });
    
    ui.separator();
    
    // 控制台输出区域
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut app.ui_state.console_output.as_str())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
                    .interactive(false)
            );
        });
}

/// 状态栏
pub fn status_bar(app: &mut IdleApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(&app.ui_state.status_message);
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 光标位置
                ui.label(format!("行 {}, 列 {}", 
                    app.editor.cursor_pos.0 + 1, 
                    app.editor.cursor_pos.1 + 1));
                
                // 修改状态
                if app.editor.is_modified() {
                    ui.label("● 已修改");
                }
            });
        });
    });
}

/// 关于对话框
pub fn about_dialog(app: &mut IdleApp, ctx: &egui::Context) {
    egui::Window::new("关于 cay-IDLE")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("cay-IDLE");
            ui.label(format!("版本: {}", app.version));
            ui.label("Cavvy 集成开发环境");
            ui.separator();
            ui.label("基于 egui 构建");
            ui.label("支持语法检查和一键编译运行");
            ui.separator();
            ui.label("快捷键:");
            ui.label("  Ctrl+N - 新建文件");
            ui.label("  Ctrl+O - 打开文件");
            ui.label("  Ctrl+S - 保存文件");
            ui.label("  F5 - 检查语法");
            ui.label("  F7 - 编译");
            ui.label("  F9 - 编译并运行");
            
            if ui.button("关闭").clicked() {
                app.ui_state.show_about_dialog = false;
            }
        });
}

/// 工具栏按钮
pub fn toolbar_button(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> bool {
    ui.button(icon).on_hover_text(tooltip).clicked()
}

/// 字体设置对话框
pub fn font_settings_dialog(app: &mut IdleApp, ctx: &egui::Context) {
    if !app.font_settings_state.show_dialog {
        return;
    }

    egui::Window::new("字体设置")
        .collapsible(false)
        .resizable(false)
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.heading("界面字体");
            ui.horizontal(|ui| {
                ui.label("字体:");
                egui::ComboBox::from_id_salt("ui_font_combo")
                    .selected_text(&app.font_settings_state.temp_ui_font)
                    .show_ui(ui, |ui| {
                        for font in crate::idle::font_manager::get_recommended_ui_fonts() {
                            ui.selectable_value(
                                &mut app.font_settings_state.temp_ui_font,
                                font.to_string(),
                                font,
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("大小:");
                ui.add(
                    egui::Slider::new(&mut app.font_settings_state.temp_ui_font_size, 8.0..=32.0)
                        .step_by(1.0)
                        .text("px"),
                );
            });

            ui.separator();
            ui.heading("编辑器字体");
            ui.horizontal(|ui| {
                ui.label("字体:");
                egui::ComboBox::from_id_salt("editor_font_combo")
                    .selected_text(&app.font_settings_state.temp_editor_font)
                    .show_ui(ui, |ui| {
                        for font in crate::idle::font_manager::get_recommended_editor_fonts() {
                            ui.selectable_value(
                                &mut app.font_settings_state.temp_editor_font,
                                font.to_string(),
                                font,
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("大小:");
                ui.add(
                    egui::Slider::new(
                        &mut app.font_settings_state.temp_editor_font_size,
                        8.0..=32.0,
                    )
                    .step_by(1.0)
                    .text("px"),
                );
            });

            ui.separator();

            // 预览区域
            ui.heading("预览");
            ui.group(|ui| {
                ui.label(
                    egui::RichText::new("界面字体预览 - 中文 ABC 123")
                        .font(egui::FontId::proportional(
                            app.font_settings_state.temp_ui_font_size,
                        )),
                );
                ui.label(
                    egui::RichText::new("编辑器字体预览 - 中文 ABC 123")
                        .font(egui::FontId::monospace(
                            app.font_settings_state.temp_editor_font_size,
                        )),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("确定").clicked() {
                    app.font_settings_state.apply(&mut app.font_config);
                    app.reload_fonts(ctx);
                    app.font_settings_state.show_dialog = false;
                    app.ui_state.status_message = "字体设置已更新".to_string();
                }
                if ui.button("取消").clicked() {
                    app.font_settings_state.show_dialog = false;
                }
                if ui.button("重置为默认").clicked() {
                    app.font_config = crate::idle::font_manager::FontConfig::default();
                    app.font_settings_state.reset(&app.font_config);
                    app.reload_fonts(ctx);
                }
            });
        });
}
