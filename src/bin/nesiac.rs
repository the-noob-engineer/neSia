use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    file_path: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    info!("Welcome to the neSia interpreter");

    let file = std::fs::read_to_string(args.file_path);

    match file {
        Ok(value) => {
            if let Err(err) = libnesia::nesia_interpret(value) {
                eprintln!("Lexer error: {err}");
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("There is an error: {err}");
            std::process::exit(1);
        }
    }
}
