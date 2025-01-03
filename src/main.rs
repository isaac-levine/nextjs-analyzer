//use std::fs::File;
//use std::io::prelude::*;
//
//fn main() {
//    let mut file = File::open("test.txt").expect("Can't open file");
//
//    let mut contents = String::new();
//    file.read_to_string(&mut contents)
//        .expect("Oops! Can not read the file.");
//
//    println!("File contents:\n\n{}", contents);
//}

// First, update Cargo.toml:
//
// [dependencies]
// swc_common = "0.31.0"
// swc_ecma_parser = "0.137.0"
// swc_ecma_ast = "0.107.0"
// swc_ecma_visit = "0.93.0"
// colored = "2.0.0"  // For nice console output

use swc_common::{FileName, SourceMap, Spanned};
use swc_ecma_ast::*;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Visit, VisitWith};
use colored::*;
use std::fs;
use std::path::Path;

// Track what we find in a component
#[derive(Debug, Default)]
struct ComponentAnalyzer {
    hooks: Vec<(String, usize)>,  // (hook name, line number)
    has_use_client: bool,
    file_path: String,
}

impl Visit for ComponentAnalyzer {
    // Look for 'use client' directive
    fn visit_module_item(&mut self, item: &ModuleItem) {
        if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = item {
            if let Expr::Lit(Lit::Str(str_lit)) = &**expr {
                if str_lit.value.to_string() == "use client" {
                    self.has_use_client = true;
                }
            }
        }
    }

    // Find React hooks
    fn visit_call_expr(&mut self, call: &CallExpr) {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = &**expr {
                let name = ident.sym.to_string();
                if name.starts_with("use") {  // React hooks start with 'use'
                    let line = 1 + ident.span.lo.0 / 80; // Rough line number estimation
                    self.hooks.push((name, line as usize));
                }
            }
        }
    }
}

fn analyze_file(path: &Path) -> Result<ComponentAnalyzer, String> {
    // Read the file
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Set up the parser
    let cm = SourceMap::default();
    let fm = cm.new_source_file(FileName::Real(path.to_path_buf()), source.into());
    
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module()
        .map_err(|e| format!("Failed to parse: {}", e))?;
    
    // Analyze the component
    let mut analyzer = ComponentAnalyzer {
        file_path: path.display().to_string(),
        ..Default::default()
    };
    module.visit_with(&mut analyzer);
    
    Ok(analyzer)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    
    println!("\n🔍 {} {}\n", "Analyzing Next.js components in".cyan(), dir.cyan());

    let paths = fs::read_dir(dir).expect("Failed to read directory");
    let mut has_issues = false;

    for path in paths {
        if let Ok(entry) = path {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "tsx" || ext == "jsx" {
                    match analyze_file(&path) {
                        Ok(analysis) => {
                            // Only report if we found client hooks in a server component
                            if !analysis.has_use_client && !analysis.hooks.is_empty() {
                                has_issues = true;
                                println!("{}", "⚠️  Server Component Issue Found!".red());
                                println!("   File: {}", analysis.file_path);
                                println!("   Client-side hooks in server component:");
                                for (hook, line) in &analysis.hooks {
                                    println!("   - {} on line {}", hook.yellow(), line);
                                }
                                println!();
                            }
                        }
                        Err(e) => eprintln!("Error analyzing {}: {}", path.display(), e),
                    }
                }
            }
        }
    }

    if !has_issues {
        println!("{}", "✅ No server component issues found!".green());
    }
}
