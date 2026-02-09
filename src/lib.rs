pub mod error;
pub mod types;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod codegen;

use error::cayResult;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, source: &str, output_path: &str) -> cayResult<()> {
        // 1. 词法分析
        let tokens = lexer::lex(source)?;
        
        // 调试：打印所有token
        #[cfg(debug_assertions)]
        {
            println!("Tokens:");
            for (i, t) in tokens.iter().enumerate() {
                println!("  {}: {:?} at {}", i, t.token, t.loc);
            }
            println!();
        }
        
        // 2. 语法分析
        let ast = parser::parse(tokens)?;
        
        // 3. 语义分析
        let mut analyzer = semantic::SemanticAnalyzer::new();
        analyzer.analyze(&ast)?;

        // 4. 代码生成 - 生成LLVM IR（字符串常量已在生成器内处理）
        let mut ir_gen = codegen::IRGenerator::new();
        // 传递类型注册表以支持正确的方法名生成
        ir_gen.set_type_registry(analyzer.get_type_registry().clone());
        let ir = ir_gen.generate(&ast)?;
        
        // 输出到文件
        std::fs::write(output_path, ir)
            .map_err(|e| error::cayError::Io(e.to_string()))?;
        
        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_lexer() {
        let source = r#"public class hello {
    public static void main() {
        print("Hello, World");
    }
}"#;
        let tokens = lexer::lex(source).unwrap();
        println!("Tokens:");
        for (i, t) in tokens.iter().enumerate() {
            println!("  {}: {:?} at {}", i, t.token, t.loc);
        }
    }

    #[test]
    fn test_hello_parser() {
        let source = r#"public class hello {
    public static void main() {
        print("Hello, World");
    }
}"#;
        let tokens = lexer::lex(source).unwrap();
        let ast = parser::parse(tokens).unwrap();
        println!("AST: {:?}", ast);
    }
}
