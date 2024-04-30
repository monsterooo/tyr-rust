use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub fn process_encode(input: &str) -> anyhow::Result<()> {
    let encode = URL_SAFE.encode(input);
    println!("{}", encode);

    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decode = URL_SAFE.decode(input)?;
    let decode = String::from_utf8(decode)?;
    println!("{}", decode);

    Ok(())
}