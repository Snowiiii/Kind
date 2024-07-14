use std::{env, path::Path};

mod assembly;
mod generator;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file");
    }
    let input_file = Path::new(&args[1]);
    if !input_file.exists() {
        panic!("Input file does not exist");
    }
    println!("Using Input file {}", input_file.display());

    let output_file = Path::new("test.asm").to_path_buf();

    let tokens = lexer::read_file(input_file.to_path_buf());
    let nodes = parser::parse_tokens(tokens).expect("Failed to parse");
    let assembly_instructions = generator::parse_to_assembly(&nodes);
    assembly::write_into_file(assembly_instructions, output_file.to_path_buf());
}
