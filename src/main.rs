use std::fs;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use std::path::Path;
use swc_common::{SourceMap, FileName};
use swc_ecma_visit::{Visit, VisitWith};
use swc_ecma_ast::*;
use colored::Colorize;
use walkdir::WalkDir;

// Represents an opportunity to optimize code by moving it from client to server component
#[derive(Debug)]
struct OptimizationOpportunity {
    file: String,
    line: usize,
    description: String,
    code: String,
}

// Tracks opportunities found while traversing the AST of a component
struct OpportunityFinder {
    opportunities: Vec<OptimizationOpportunity>,
    has_client_directive: bool,
    current_file: String,
}


// Implementation of AST visitor to find optimization opportunities
impl Visit for OpportunityFinder {
    fn visit_module_item(&mut self, node: &ModuleItem) {
        println!("Visiting module item");
        if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = node {
            if let Expr::Lit(Lit::Str(str_lit)) = &**expr {
                println!("Found string literal: {}", str_lit.value.to_string());
                if str_lit.value.to_string() == "use client" {
                    println!("Found 'use client' directive");
                    self.has_client_directive = true;
                }
            }
        }
    }

    fn visit_await_expr(&mut self, await_expr: &AwaitExpr) {
        println!("Found await expression");
        if self.has_client_directive {
            println!("In client component, adding opportunity");
            let line = await_expr.span.lo.0 as usize / 80;
            self.opportunities.push(OptimizationOpportunity {
                file: self.current_file.clone(),
                line,
                description: "Data fetching could be moved to server component".to_string(),
                code: "await fetch(...)".to_string(),
            });
        }
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        if self.has_client_directive {
            if let Callee::Expr(expr) = &call.callee {
                if let Expr::Ident(ident) = &**expr {
                    let name = ident.sym.to_string();
                    println!("Found function call: {}", name);
                }
            }
        }
    }

    fn visit_var_decl(&mut self, var: &VarDecl) {
        println!("Visiting var decl");
        if self.has_client_directive {
            for decl in &var.decls {
                if let Some(init) = &decl.init {
                    if let Expr::Arrow(_) | Expr::Fn(_) = &**init {
                        // We've found a function declaration, visit its body
                        init.visit_with(self);
                    }
                }
            }
        }
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        println!("Visiting fn decl");
        if self.has_client_directive {
            fn_decl.function.visit_with(self);
        }
    }
}

// Analyzes a single file for optimization opportunities
fn analyze_file(path: &Path) -> Vec<OptimizationOpportunity> {
    println!("Reading file: {:?}", path);
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            return vec![];
        }
    };

    let cm = SourceMap::default();
    let fm = cm.new_source_file(FileName::Real(path.to_path_buf()), source.into());
    
    // Configure parser
    let lexer = Lexer::new(
        Syntax::Typescript(swc_ecma_parser::TsConfig {
            tsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    
    let mut parser = Parser::new_from(lexer);
    
    match parser.parse_module() {
        Ok(module) => {
            let mut finder = OpportunityFinder {
                opportunities: vec![],
                has_client_directive: false,
                current_file: path.display().to_string(),
            };
            module.visit_with(&mut finder);
            finder.opportunities
        }
        Err(e) => {
            println!("Parser error: {:?}", e);
            vec![]
        }
    }
}

// Recursively analyzes all JS/TS files in directory
fn analyze_directory(dir: &str) -> Vec<OptimizationOpportunity> {
    println!("Scanning directory: {}", dir);
    let mut all_opportunities = Vec::new();
    
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok()) {
            println!("Found file: {:?}", entry.path());
            if let Some(ext) = entry.path().extension() {
                println!("Extension: {:?}", ext);
                if ext == "tsx" || ext == "jsx" || ext == "js" || ext == "ts" {
                    println!("Analyzing file: {:?}", entry.path());
                    all_opportunities.extend(analyze_file(entry.path()));
                }
            }
    }
    
    all_opportunities
}

fn main() {
    // Get directory from args or use current directory 
    let args: Vec<String> = std::env::args().collect();
    let default_dir = String::from(".");
    let dir = args.get(1).unwrap_or(&default_dir).as_str();
    
    println!("\n🔍 Analyzing components in: {}\n", dir.cyan());
    
    let opportunities = analyze_directory(dir);
    
    if opportunities.is_empty() {
        println!("{}", "✅ No optimization opportunities found!".green());
    } else {
        println!("{}", "💡 Found potential optimizations:".yellow());
        for opp in opportunities {
            // Pretty-print opportunity results
            println!(
                "{}:{} - {}\n  Code: {}",
                opp.file.cyan(),
                opp.line.to_string().yellow(),
                opp.description.white(),
                opp.code.bright_black()
            );
        }
    }
}

