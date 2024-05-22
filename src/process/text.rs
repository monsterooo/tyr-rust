use crate::{get_reader, TextSignFormat};

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    let sign = match format {
        TextSignFormat::Blake3 => todo!(),
        TextSignFormat::Ed25519 => todo!(),
    };

    Ok(())
}
