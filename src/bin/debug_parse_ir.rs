// 调试IR解析
use cavvy::lexer::{lex_with_diagnostics, Token};

fn main() {
    let source = r#"
__ir {
    %cmp = fcmp oeq float %a, %b
}
"#;
    
    let (tokens, _diagnostics) = lex_with_diagnostics(source);
    
    // 模拟parse_inline_ir_from_tokens
    let mut raw_lines: Vec<String> = Vec::new();
    let mut current_line: Vec<String> = Vec::new();
    let mut brace_depth = 1;
    let mut current_line_num = 3; // 从第3行开始（__ir { 之后）
    
    // 跳过前两个token: InlineIr 和 LBrace
    let mut i = 2;
    
    while i < tokens.len() && brace_depth > 0 {
        let token = &tokens[i];
        let token_line = token.loc.line;
        
        println!("Processing token {}: {:?} at line {}", i, token.token, token_line);
        
        // 如果行号变化，保存当前行并开始新行
        if token_line != current_line_num && !current_line.is_empty() {
            let line = current_line.join(" ");
            if !line.trim().is_empty() {
                println!("  -> Saving line: '{}'", line);
                raw_lines.push(line);
            }
            current_line.clear();
            current_line_num = token_line;
        }
        
        match &token.token {
            Token::LBrace => {
                brace_depth += 1;
                if brace_depth > 1 {
                    current_line.push("{".to_string());
                }
                i += 1;
            }
            Token::RBrace => {
                brace_depth -= 1;
                if brace_depth > 0 {
                    current_line.push("}".to_string());
                }
                i += 1;
            }
            Token::Percent => {
                i += 1;
                if i < tokens.len() {
                    let reg_name = match &tokens[i].token {
                        Token::Identifier(s) => {
                            let name = s.clone();
                            i += 1;
                            name
                        }
                        Token::IntegerLiteral(opt) => {
                            let val = match opt {
                                Some((v, _)) => v.to_string(),
                                None => "0".to_string(),
                            };
                            i += 1;
                            val
                        }
                        _ => {
                            current_line.push("%".to_string());
                            continue;
                        }
                    };
                    current_line.push(format!("%{}", reg_name));
                }
            }
            Token::Identifier(s) => {
                current_line.push(s.clone());
                i += 1;
            }
            Token::Float => {
                println!("  -> Found Float token, pushing 'float'");
                current_line.push("float".to_string());
                i += 1;
            }
            Token::IntegerLiteral(opt) => {
                let val = match opt {
                    Some((v, _)) => v.to_string(),
                    None => "0".to_string(),
                };
                current_line.push(val);
                i += 1;
            }
            Token::Comma => {
                current_line.push(",".to_string());
                i += 1;
            }
            Token::Star => {
                current_line.push("*".to_string());
                i += 1;
            }
            Token::Assign => {
                current_line.push("=".to_string());
                i += 1;
            }
            Token::Float => {
                println!("  -> Found Float token, pushing 'float'");
                current_line.push("float".to_string());
                i += 1;
            }
            Token::Double => {
                current_line.push("double".to_string());
                i += 1;
            }
            _ => {
                println!("  -> Unhandled token, skipping");
                i += 1;
            }
        }
    }
    
    // 处理最后一行
    if !current_line.is_empty() {
        let line = current_line.join(" ");
        if !line.trim().is_empty() {
            println!("  -> Saving final line: '{}'", line);
            raw_lines.push(line);
        }
    }
    
    println!("\nFinal raw_lines:");
    for (i, line) in raw_lines.iter().enumerate() {
        println!("  Line {}: '{}'", i + 1, line);
    }
}
