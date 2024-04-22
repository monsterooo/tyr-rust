use clap::Parser;
use std::path::Path;
#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "convert csv to format.")]
    Csv(CsvOpts)
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts)
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}