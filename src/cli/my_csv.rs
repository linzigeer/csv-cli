use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn verify_file(input: &str) -> Result<String, String> {
    if Path::new(&input).exists() {
        Ok(input.into())
    } else {
        Err(format!("file:{:?} doesn't exist!", input))
    }
}
