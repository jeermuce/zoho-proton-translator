mod args;
use clap::Parser;
mod csv_lc;
use args::parsing::parse_secret_data;
mod utils;
use args::Args;
use csv::WriterBuilder;
use csv_lc::file_managment::{create_file_with_dirs, make_input_reader, make_output_writer};
use csv_lc::serialize::{ProtonStyleCsv, ZohoStyleCsv};
use utils::custom_unwrapping::unwrap_path;
use utils::type_conversion::{bool_from_int, csv_str_to_vec};

use std::io::Cursor;
use std::process::exit;

use anyhow::{Error, Result};

fn dry_run(args: &Args) -> Result<()> {
    let mut reader = make_input_reader(&args.input_file);
    let mut proton_vec = vec![];

    for result in reader.deserialize::<ZohoStyleCsv>() {
        match result {
            Ok(z) => {
                let rr: ProtonStyleCsv = z.into();
                proton_vec.push(rr);
            }
            Err(e) => eprintln!("could not deserialize: {e}"),
        }
    }

    let mut writer = WriterBuilder::new().from_writer(Cursor::new(Vec::new()));

    for result in proton_vec {
        writer.serialize(result)?;
    }
    writer.flush()?;

    let buffer = match String::from_utf8(
        match writer.into_inner() {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "could not write into cursor: {e}\n no further processinng will be performed"
                );
                exit(1)
            }
        }
        .into_inner(),
    ) {
        Ok(e) => e,
        Err(e) => {
            println!("could not stringify: {e}\n no further processinng will be performed");
            exit(1)
        }
    };

    print!("{buffer}");

    Ok(())
}

fn write_run(args: &Args) -> Result<()> {
    let output_path = unwrap_path(&args.output_file);
    let mut reader = make_input_reader(&args.input_file);
    create_file_with_dirs(output_path)?;
    let mut writer = make_output_writer(output_path);

    for result in reader.deserialize::<ZohoStyleCsv>() {
        match result {
            Ok(z) => {
                let rr: ProtonStyleCsv = z.into();
                writer.serialize(rr)?;
            }
            Err(e) => eprintln!("could not deserialize: {e}"),
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Args = Args::parse();

    if args.input_file.exists() {
        println!("{args:?}");
    }

    match args.write {
        false => dry_run(&args)?,
        true => write_run(&args)?,
    }

    Ok(())
}
