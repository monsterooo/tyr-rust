use std::io::Read;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let encode = URL_SAFE.encode(input);
    println!("{}", encode);

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let decode = URL_SAFE.decode(input)?;
    let decode = String::from_utf8(decode)?;
    println!("{}", decode);

    Ok(())
}
