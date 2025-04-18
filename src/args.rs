use std::path::PathBuf;

use clap::Parser;

use parsing::{parse_input_path, parse_output_path};

/// Simple program to transform zoho style csv into proton style csv for export-import
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the zoho csv file
    #[arg(short = 'i', long = "input_file", value_name = "INPUT_FILE", value_parser = parse_input_path)]
    pub input_file: PathBuf,

    /// Write the output file
    #[arg(short = 'w', long = "write", value_name = "WRITE")]
    pub write: bool,

    /// Path of the output file (required if --write is set)
    #[arg(short = 'o', long = "output_file", value_name = "OUTPUT_FILE", value_parser = parse_output_path, requires = "write")]
    pub output_file: Option<PathBuf>,
}

pub mod parsing;
