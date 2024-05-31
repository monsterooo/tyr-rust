use std::fs;

use anyhow;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify, Base64Subcommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};

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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbols,
            )?;
            println!("{}", password)
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64Subcommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;

                println!("{}", decode);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig)
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", verified)
            }
            TextSubCommand::Generate(opts) => {
                let key = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }

    Ok(())
}
