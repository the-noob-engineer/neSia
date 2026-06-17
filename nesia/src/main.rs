use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    versbose: bool,

    #[arg(default_value = "main.ns", num_args = 1)]
    file: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    if args.versbose {
        println!("Verbose mode enabled");
    }

    let file_full_path = Path::new(&current_dir).join(&args.file);
    let source_code = std::fs::read_to_string(&file_full_path).expect("Failed to read source code");
    let tokens = nesia_lexer::tokenize(&source_code);
    let ast = nesia_parser::parse_tokens(tokens);

    println!("{:?}", ast);
}
