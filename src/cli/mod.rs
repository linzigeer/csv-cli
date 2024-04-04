mod my_csv;
mod process;

pub use my_csv::*;
pub use process::*;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv, or convert csv to other formats")]
    Csv(CsvOpts),
}
