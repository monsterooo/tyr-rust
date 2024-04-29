mod base64;
mod csv;
mod genpass;
use clap::Parser;

use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::{base64::Base64Subcommand, csv::OutputFormat};

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
}
