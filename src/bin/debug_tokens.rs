// 调试token生成
use cavvy::lexer::lex_with_diagnostics;

fn main() {
    let source = r#"
__ir {
    %cmp = fcmp oeq float %a, %b
}
"#;
    
    let (tokens, _diagnostics) = lex_with_diagnostics(source);
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, token);
    }
}
