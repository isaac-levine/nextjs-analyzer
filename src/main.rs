use swc_common::SourceMap;
use swc_common::FileName;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn main() {
    // The code we want to analyze
    let source_code = r#"
        function hello() {
            console.log("Hello, World!");
        }
    "#;

    // Create a source map (for error reporting)
    let source_map = SourceMap::default();

    // Create a source file
    let source_file = source_map.new_source_file(
        FileName::Anon,
        source_code.into()
    );

    // Create a parser
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*source_file),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    // Parse the module
    match parser.parse_module() {
        Ok(module) => {
            println!("Successfully parsed the code!");
            println!("{:#?}", module); // magically pretty-print the AST
        } 
        Err(e) => {
            println!("Failed to parse: {:?}", e);
        }
    }
}
