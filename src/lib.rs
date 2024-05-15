mod cli;
mod process;

pub use cli::{Base64Format, Base64Subcommand, Opts, SubCommand};
pub use process::{process_csv, process_genpass};
