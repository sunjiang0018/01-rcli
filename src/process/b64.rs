use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use crate::get_reader;

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let mut reader = get_reader(input).unwrap();
        let format = Base64Format::Standard;
        assert!(process_encode(&mut reader, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let mut reader = get_reader(input).unwrap();
        let format = Base64Format::Standard;
        assert!(process_decode(&mut reader, format).is_ok());
    }
}
