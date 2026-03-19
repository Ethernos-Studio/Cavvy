//! 文件浏览器模块 - VSCode风格的资源管理器

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// 文件树节点
#[derive(Clone, Debug)]
pub struct FileNode {
    /// 文件/文件夹名称
    pub name: String,
    /// 完整路径
    pub path: PathBuf,
    /// 是否是文件夹
    pub is_dir: bool,
    /// 子节点（仅文件夹有）
    pub children: Vec<FileNode>,
    /// 是否展开（仅文件夹有效）
    pub expanded: bool,
}

impl FileNode {
    /// 创建文件节点
    pub fn new_file(path: &Path) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        Self {
            name,
            path: path.to_path_buf(),
            is_dir: false,
            children: Vec::new(),
            expanded: false,
        }
    }
    
    /// 创建文件夹节点
    pub fn new_dir(path: &Path) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        Self {
            name,
            path: path.to_path_buf(),
            is_dir: true,
            children: Vec::new(),
            expanded: false,
        }
    }
    
    /// 加载子节点
    pub fn load_children(&mut self) {
        if !self.is_dir {
            return;
        }
        
        self.children.clear();
        
        if let Ok(entries) = std::fs::read_dir(&self.path) {
            let mut dirs: Vec<FileNode> = Vec::new();
            let mut files: Vec<FileNode> = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                
                // 跳过隐藏文件/文件夹
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if name_str.starts_with('.') {
                            continue;
                        }
                    }
                }
                
                if is_dir {
                    dirs.push(FileNode::new_dir(&path));
                } else {
                    files.push(FileNode::new_file(&path));
                }
            }
            
            // 文件夹在前，按字母排序
            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            
            self.children.extend(dirs);
            self.children.extend(files);
        }
    }
    
    /// 切换展开状态
    pub fn toggle_expanded(&mut self) {
        if self.is_dir {
            self.expanded = !self.expanded;
            if self.expanded && self.children.is_empty() {
                self.load_children();
            }
        }
    }
    
    /// 获取图标
    pub fn icon(&self) -> &'static str {
        if self.is_dir {
            if self.expanded {
                "📂"
            } else {
                "📁"
            }
        } else {
            get_file_icon(&self.name)
        }
    }
}

/// 文件浏览器状态
pub struct FileBrowserState {
    /// 根路径
    pub root_path: Option<PathBuf>,
    /// 根节点
    pub root_node: Option<FileNode>,
    /// 选中的文件/文件夹
    pub selected_path: Option<PathBuf>,
    /// 展开的文件夹集合
    pub expanded_paths: HashSet<PathBuf>,
    /// 是否显示文件
    pub show_files: bool,
    /// 搜索过滤
    pub search_filter: String,
    /// 待处理的点击路径（用于回调）
    pub pending_file_click: Option<PathBuf>,
    /// 待处理的文件夹点击路径
    pub pending_folder_click: Option<PathBuf>,
    /// 待展开/折叠的路径
    pub pending_toggle: Option<PathBuf>,
}

impl Default for FileBrowserState {
    fn default() -> Self {
        Self {
            root_path: None,
            root_node: None,
            selected_path: None,
            expanded_paths: HashSet::new(),
            show_files: true,
            search_filter: String::new(),
            pending_file_click: None,
            pending_folder_click: None,
            pending_toggle: None,
        }
    }
}

impl FileBrowserState {
    /// 从路径打开文件夹
    pub fn open_folder(&mut self, path: &Path) {
        self.root_path = Some(path.to_path_buf());
        let mut root = FileNode::new_dir(path);
        root.expanded = true;
        root.load_children();
        self.root_node = Some(root);
        self.expanded_paths.insert(path.to_path_buf());
    }
    
    /// 刷新当前文件夹
    pub fn refresh(&mut self) {
        if let Some(ref root) = self.root_node {
            let path = root.path.clone();
            self.open_folder(&path);
        }
    }
    
    /// 设置选中路径
    pub fn select_path(&mut self, path: &Path) {
        self.selected_path = Some(path.to_path_buf());
    }
    
    /// 切换文件夹展开状态
    pub fn toggle_folder(&mut self, path: &Path) {
        if self.expanded_paths.contains(path) {
            self.expanded_paths.remove(path);
        } else {
            self.expanded_paths.insert(path.to_path_buf());
        }
    }
    
    /// 展开到指定路径
    pub fn expand_to_path(&mut self, path: &Path) {
        // 收集所有父路径
        let mut current = Some(path);
        while let Some(p) = current {
            if let Some(parent) = p.parent() {
                self.expanded_paths.insert(parent.to_path_buf());
                current = Some(parent);
            } else {
                break;
            }
        }
    }
    
    /// 获取当前文件夹路径
    pub fn get_current_folder(&self) -> Option<&Path> {
        self.root_path.as_deref()
    }
    
    /// 处理待点击的文件
    pub fn take_pending_file_click(&mut self) -> Option<PathBuf> {
        self.pending_file_click.take()
    }
    
    /// 处理待点击的文件夹
    pub fn take_pending_folder_click(&mut self) -> Option<PathBuf> {
        self.pending_folder_click.take()
    }
    
    /// 处理待展开/折叠
    pub fn take_pending_toggle(&mut self) -> Option<PathBuf> {
        self.pending_toggle.take()
    }
    
    /// 应用展开状态到树
    pub fn apply_expanded_state(&mut self) {
        if let Some(ref mut root) = self.root_node {
            Self::apply_expanded_recursive(root, &self.expanded_paths);
        }
    }
    
    /// 递归应用展开状态
    fn apply_expanded_recursive(node: &mut FileNode, expanded: &HashSet<PathBuf>) {
        if node.is_dir {
            let was_expanded = node.expanded;
            node.expanded = expanded.contains(&node.path);
            
            // 如果刚展开且没有子节点，加载子节点
            if node.expanded && !was_expanded && node.children.is_empty() {
                node.load_children();
            }
            
            // 递归更新子节点
            for child in &mut node.children {
                Self::apply_expanded_recursive(child, expanded);
            }
        }
    }
}

/// 根据文件名获取图标
fn get_file_icon(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    
    if lower.ends_with(".cay") {
        "📝"
    } else if lower.ends_with(".cayh") {
        "📋"
    } else if lower.ends_with(".toml") {
        "⚙️"
    } else if lower.ends_with(".md") || lower.ends_with(".txt") {
        "📄"
    } else if lower.ends_with(".rs") {
        "🦀"
    } else if lower.ends_with(".py") {
        "🐍"
    } else if lower.ends_with(".js") || lower.ends_with(".ts") {
        "📜"
    } else if lower.ends_with(".json") {
        "📊"
    } else if lower.ends_with(".html") || lower.ends_with(".htm") {
        "🌐"
    } else if lower.ends_with(".css") {
        "🎨"
    } else if lower.ends_with(".cpp") || lower.ends_with(".c") || lower.ends_with(".h") || lower.ends_with(".hpp") {
        "🔧"
    } else if lower.ends_with(".exe") || lower.ends_with(".dll") || lower.ends_with(".so") {
        "⚙️"
    } else {
        "📄"
    }
}

/// 渲染文件浏览器UI
pub fn render_file_browser_ui(ui: &mut egui::Ui, state: &mut FileBrowserState) {
    // 工具栏
    ui.horizontal(|ui| {
        if ui.button("🔄").on_hover_text("刷新").clicked() {
            state.refresh();
        }
        
        if ui.button("⬆️").on_hover_text("上级目录").clicked() {
            if let Some(ref root) = state.root_path.clone() {
                if let Some(parent) = root.parent() {
                    state.open_folder(parent);
                }
            }
        }
        
        ui.separator();
        
        // 搜索框
        ui.add(
            egui::TextEdit::singleline(&mut state.search_filter)
                .hint_text("🔍 搜索文件...")
                .desired_width(ui.available_width() - 10.0)
        );
    });
    
    ui.separator();
    
    // 当前路径显示
    if let Some(ref root) = state.root_path {
        ui.horizontal(|ui| {
            ui.label("📁");
            ui.label(root.display().to_string())
                .on_hover_text("当前文件夹");
        });
        ui.separator();
    }
    
    // 文件树
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // 先应用展开状态
            state.apply_expanded_state();
            
            // 渲染文件树 - 使用临时的待处理操作收集器
            let mut pending_ops = PendingOps::default();
            
            // 克隆需要的渲染状态
            let selected_path = state.selected_path.clone();
            let search_filter = state.search_filter.clone();
            
            if let Some(ref mut root) = state.root_node {
                render_node_tree(ui, root, &selected_path, &search_filter, 0, &mut pending_ops);
            } else {
                ui.label("未打开文件夹");
                ui.label("点击菜单 文件 → 打开文件夹");
            }
            
            // 应用待处理的操作
            if let Some(path) = pending_ops.toggle {
                state.toggle_folder(&path);
            }
            if let Some(path) = pending_ops.file_click {
                state.pending_file_click = Some(path);
            }
            if let Some(path) = pending_ops.folder_click {
                state.pending_folder_click = Some(path);
            }
            if let Some(path) = pending_ops.select {
                state.selected_path = Some(path);
            }
        });
}

/// 待处理的操作
#[derive(Default)]
struct PendingOps {
    toggle: Option<PathBuf>,
    file_click: Option<PathBuf>,
    folder_click: Option<PathBuf>,
    select: Option<PathBuf>,
}

/// 递归渲染节点树
fn render_node_tree(
    ui: &mut egui::Ui, 
    node: &mut FileNode, 
    selected_path: &Option<PathBuf>,
    search_filter: &str,
    depth: usize,
    pending_ops: &mut PendingOps,
) {
    // 搜索过滤
    if !search_filter.is_empty() {
        let filter = search_filter.to_lowercase();
        if !node.name.to_lowercase().contains(&filter) {
            // 如果当前节点不匹配，但子节点可能匹配，仍然显示文件夹
            if node.is_dir {
                // 检查是否有匹配的子节点
                let has_match = node.children.iter().any(|child| {
                    child.name.to_lowercase().contains(&filter) || child.is_dir
                });
                if !has_match {
                    return;
                }
            } else {
                return;
            }
        }
    }
    
    let indent = depth as f32 * 16.0;
    let is_selected = selected_path.as_ref() == Some(&node.path);
    let node_path = node.path.clone();
    let is_dir = node.is_dir;
    
    // 创建响应区域
    let response = ui.horizontal(|ui| {
        // 缩进
        ui.add_space(indent);
        
        // 展开/折叠按钮（仅文件夹）
        if is_dir {
            let arrow = if node.expanded { "▼" } else { "▶" };
            if ui.button(arrow).clicked() {
                pending_ops.toggle = Some(node_path.clone());
            }
        } else {
            ui.add_space(24.0); // 占位
        }
        
        // 图标
        ui.label(node.icon());
        
        // 名称
        let name_label = if is_selected {
            egui::RichText::new(&node.name).strong()
        } else {
            egui::RichText::new(&node.name)
        };
        
        ui.selectable_label(is_selected, name_label)
    }).inner;
    
    // 处理点击
    if response.clicked() {
        pending_ops.select = Some(node.path.clone());
        
        if is_dir {
            pending_ops.toggle = Some(node.path.clone());
            pending_ops.folder_click = Some(node.path.clone());
        } else {
            pending_ops.file_click = Some(node.path.clone());
        }
    }
    
    // 右键菜单 - 使用节点路径的克隆
    let node_path_for_menu = node.path.clone();
    let is_dir_for_menu = node.is_dir;
    response.context_menu(move |ui| {
        if is_dir_for_menu {
            if ui.button("📂 展开").clicked() {
                ui.close_menu();
            }
            if ui.button("📁 折叠").clicked() {
                ui.close_menu();
            }
            ui.separator();
            if ui.button("🔄 刷新").clicked() {
                ui.close_menu();
            }
        } else {
            if ui.button("📝 打开").clicked() {
                ui.close_menu();
            }
            ui.separator();
            if ui.button("📋 复制路径").clicked() {
                ui.ctx().output_mut(|o| {
                    o.copied_text = node_path_for_menu.display().to_string();
                });
                ui.close_menu();
            }
        }
    });
    
    // 递归渲染子节点
    if node.expanded {
        for child in &mut node.children {
            render_node_tree(ui, child, selected_path, search_filter, depth + 1, pending_ops);
        }
    }
}
