use swc_common::SourceMap;
use swc_common::FileName;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use std::fs::File;
use std::io::Read;

fn main() {
    
    // Read the file contents into source_code
    let mut file = File::open("code.js").expect("Can't open file!");
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)
        .expect("Oops! Can not read the file.");

    println!("File contents:\n\n{}", source_code);

    // Create a source map 
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
            println!("{:#?}", module); // Pretty-print the AST 
        } 
        Err(e) => {
            println!("Failed to parse: {:?}", e);
        }
    }
}
