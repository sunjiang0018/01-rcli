use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::CmdExecutor;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key")]
    Generate(GenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateKeyOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25510",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = crate::process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", sig);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = crate::process_text_verify(&self.input, &self.key, self.format, &self.sig)?;

        if verified {
            println!("Signature verified");
        } else {
            println!("Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecutor for GenerateKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_generate(self.format)?;

        for (k, v) in key {
            fs::write(self.output.join(k), v).await?;
        }

        Ok(())
    }
}
