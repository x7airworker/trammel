use lang_c::driver::{Config, parse};
use std::{env, fs};
use string_builder::Builder;
use visitor::{RustDeclaration, RustDeclarationType};

mod visitor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please provide an input amd output file!");
        return;
    }

    let config = Config::default();
    let parsing_res = parse(&config, args[1].clone());
    match parsing_res {
        Err(e) => {
            println!("Parsing error: {:?}", e);
        }
        Ok(x) => {
            println!("parsing correct");
            let mut visitor = visitor::Visitor::default();
            visitor.visit_translation_unit(&x.unit);
            let code = build_code(&visitor.0);
            fs::write(args[2].clone(), code).expect("Error while writing output file");
        }
    }
}

fn build_code(declarations: &Vec<RustDeclaration>) -> String {
    let mut builder = Builder::default();

    for decl in declarations {
        match &decl.type_declaration {
            RustDeclarationType::Struct(s) => {
                let field_str = s
                    .iter()
                    .map(|p| format!("\tpub {}: {}", p.name, p.param_type))
                    .collect::<Vec<String>>()
                    .join(",\n");

                builder.append(format!(
                    "
#[repr(C)]
pub struct {} {{
{}
}}
",
                    decl.name, field_str
                ));
            }
            RustDeclarationType::Function(f) => {
                let param_str = f
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.param_type))
                    .collect::<Vec<String>>()
                    .join(", ");

                builder.append(format!(
                    "
#[no_mangle]
pub extern \"C\" fn {}({}) {{

}}
",
                    decl.name, param_str
                ));
            }
        }
    }

    builder.string().expect("Error while building code")
}
