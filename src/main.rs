use anyhow::Result;
use clap::Parser;

use csv_cli::{Opts, SubCommand};

use crate::cli::process_csv;

mod cli;

fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);

    println!("{:?}", std::env::current_dir());

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(
            &opts.input,
            &opts.output.unwrap_or("./output.json".to_string()),
        )?,
    }

    Ok(())
}
