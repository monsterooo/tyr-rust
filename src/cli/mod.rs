mod base64;
mod csv;
mod genpass;
mod text;
use std::path::{Path, PathBuf};

use clap::Parser;

use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv::OutputFormat,
    text::{TextSignFormat, TextSubCommand},
};

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to format.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("no-exist"), Err("File does not exist".into()));
    }
}
