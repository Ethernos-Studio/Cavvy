use std::fs;

fn main() {
    let source = fs::read_to_string("examples/web_server.cay").unwrap();
    
    let mut pp = cavvy::preprocessor::Preprocessor::new(".");
    let result = pp.process_with_source_map(&source, "examples/web_server.cay").unwrap();
    
    let lines: Vec<&str> = result.code.lines().collect();
    
    // 输出第455-465行的内容和映射
    for i in 454..465 {
        if i < lines.len() {
            let line_content = lines[i];
            if let Some(pos) = result.source_map.mappings.get(i) {
                println!("行 {}: '{}' -> {}:{}", i+1, line_content, pos.file, pos.line);
            } else {
                println!("行 {}: '{}' -> (无映射)", i+1, line_content);
            }
        }
    }
}
