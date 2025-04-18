mod args;
use clap::Parser;
mod csv_lc;
use args::parsing::{bool_from_int, csv_str_to_vec, parse_secret_data, unwrap_path};
use args::Args;
use csv::WriterBuilder;
use csv_lc::file_managment::{create_file_with_dirs, make_input_reader, make_output_writer};
use csv_lc::serialize::serialize_vec_to_comma_separated;
use std::io::Cursor;
use std::process::exit;

use anyhow::{Error, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SecretData {
    #[serde(rename = "SecretType")]
    pub secret_type: String,
    #[serde(rename = "User Name")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ZohoStyleCsv {
    #[serde(rename = "Password Name")]
    pub password_name: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Password URL", deserialize_with = "csv_str_to_vec")]
    pub password_url: Vec<String>,

    #[serde(rename = "SecretData", deserialize_with = "parse_secret_data")]
    pub secret_data: SecretData,

    #[serde(rename = "Notes")]
    pub notes: String,

    #[serde(rename = "CustomData")]
    pub custom_data: String,

    #[serde(rename = "Tags", deserialize_with = "csv_str_to_vec")]
    pub tags: Vec<String>,

    #[serde(rename = "Classification")]
    #[serde(skip_deserializing)]
    pub _classification: Option<String>,

    #[serde(rename = "Favorite", deserialize_with = "bool_from_int")]
    pub favorite: bool,

    #[serde(rename = "TOTP")]
    pub totp: Option<String>,

    #[serde(rename = "Folder Name")]
    pub folder_name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ProtonStyleCsv {
    pub name: String,
    #[serde(serialize_with = "serialize_vec_to_comma_separated")]
    pub url: Vec<String>,
    pub email: Option<String>,
    pub username: String,
    pub password: String,
    pub note: Option<String>,
    pub totp: Option<String>,
    pub vault: Option<String>,
}

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
