use std::fs;

fn main() {
    let source = fs::read_to_string("examples/web_server.cay").unwrap();
    
    // 预处理
    let mut pp = cavvy::preprocessor::Preprocessor::new(".");
    let result = pp.process_with_source_map(&source, "examples/web_server.cay").unwrap();
    
    // 转换源映射
    let mut map = std::collections::HashMap::new();
    for (idx, pos) in result.source_map.mappings.iter().enumerate() {
        map.insert(idx + 1, (pos.file.clone(), pos.line));
    }
    
    // 词法分析
    let tokens = cavvy::lexer::lex_with_source_map(&result.code, map).unwrap();
    
    // 查找包含 "leftPtr" 或 "rightPtr" 的 token
    for (i, token) in tokens.iter().enumerate() {
        if let cavvy::lexer::Token::Identifier(name) = &token.token {
            if name == "leftPtr" || name == "rightPtr" {
                println!("Token {}: {:?}", i, name);
                println!("  loc: line={}, column={}", token.loc.line, token.loc.column);
                println!("  source_line: {:?}", token.source_line);
                println!("  source_file: {:?}", token.source_file);
                println!();
            }
        }
    }
}
