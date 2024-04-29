use anyhow;
use clap::Parser;
use rcli::{process_csv, process_genpass, Base64Subcommand, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbols,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                println!("{:?}", opts);
            }
            Base64Subcommand::Decode(opts) => {
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
