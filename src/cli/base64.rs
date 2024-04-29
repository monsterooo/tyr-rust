use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long)]
    pub input: String,
}
