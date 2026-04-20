use std::fs;
use std::path::PathBuf;

fn main() {
    let source = fs::read_to_string("examples/web_server.cay").unwrap();
    
    let mut pp = cavvy::preprocessor::Preprocessor::new(".");
    let result = pp.process_with_source_map(&source, "examples/web_server.cay").unwrap();
    
    let lines: Vec<&str> = result.code.lines().collect();
    println!("原始文件行数: {}", source.lines().count());
    println!("预处理后行数: {}", lines.len());
    println!("源映射条目数: {}", result.source_map.mappings.len());
    println!();
    
    // 检查第492行对应什么
    if lines.len() >= 492 {
        println!("第492行内容: {}", lines[491]);
        if let Some(pos) = result.source_map.mappings.get(491) {
            println!("第492行映射到: {}:{}", pos.file, pos.line);
        }
    }
    
    // 检查最后几行
    println!("\n最后5行:");
    for i in (lines.len()-5..lines.len()).rev() {
        if let Some(pos) = result.source_map.mappings.get(i) {
            println!("  行{} -> {}:{}", i+1, pos.file, pos.line);
        }
    }
}
