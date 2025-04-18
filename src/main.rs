use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Deserializer, Serialize};

/// Simple program to transform zoho style csv into proton style csv for export-import
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the zoho csv file
    #[arg(short = 'i', long = "input_file", value_name = "INPUT_FILE", value_parser = parse_input_path)]
    input_file: PathBuf,

    /// Write the output file
    #[arg(short = 'w', long = "write", value_name = "WRITE")]
    write: bool,

    /// Path of the output file (required if --write is set)
    #[arg(short = 'o', long = "output_file", value_name = "OUTPUT_FILE", value_parser = parse_output_path, requires = "write")]
    output_file: Option<PathBuf>,
}

pub fn parse_input_path(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(anyhow!("Input file does not exist: {}", s));
    }
    Ok(path)
}

pub fn parse_output_path(s: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(s))
}

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

fn csv_str_to_vec<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;

    if s.trim().is_empty() {
        Ok(vec![])
    } else {
        Ok(s.split(',').map(str::to_string).collect())
    }
}

fn parse_secret_data<'de, D>(de: D) -> Result<SecretData, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;

    let mut secret_type = String::new();
    let mut username = String::new();
    let mut password = String::new();

    for line in s.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("SecretType:") {
            secret_type = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("User Name:") {
            username = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("Password:") {
            password = rest.trim().to_string();
        }
    }

    Ok(SecretData {
        secret_type,
        username,
        password,
    })
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: u8 = Deserialize::deserialize(deserializer)?;

    match v {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("expected 0 or 1")),
    }
}

fn serialize_vec_to_comma_separated<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = value.join(",");
    serializer.serialize_str(&s)
}

impl From<ZohoStyleCsv> for ProtonStyleCsv {
    fn from(z: ZohoStyleCsv) -> Self {
        let email = if z.secret_data.username.contains('@') {
            Some(z.secret_data.username.clone())
        } else {
            None
        };

        let note = if z.notes.trim().is_empty() {
            None
        } else {
            Some(z.notes)
        };

        let vault = if z.folder_name.trim().is_empty() {
            None
        } else {
            Some(z.folder_name)
        };

        ProtonStyleCsv {
            name: z.password_name,
            url: z.password_url,
            email,
            username: z.secret_data.username,
            password: z.secret_data.password,
            note,
            totp: z.totp,
            vault,
        }
    }
}

fn make_input_reader(path: &PathBuf) -> Reader<File> {
    match csv::ReaderBuilder::new().flexible(true).from_path(path) {
        Ok(file_reader) => file_reader,
        Err(e) => {
            println!("Error creating file reader: {e}\n no further processinng will be performed");
            exit(1)
        }
    }
}

fn unwrap_path(path: &Option<PathBuf>) -> &PathBuf {
    match path {
        Some(p) => p,
        None => {
            println!("Output file must be specified if --write is set.\n no further processinng will be performed");
            exit(1);
        }
    }
}

fn make_output_writer(path: &PathBuf) -> Writer<File> {
    match csv::Writer::from_path(path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error creating file writer: {e}\n no further processinng will be performed");
            exit(1);
        }
    }
}

fn create_file_with_dirs(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        println!(
            "Warning: the file {:?} already exists, it will be truncated. Do you wish to continue? Y/n ",path
        );
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        let response = response.trim().to_lowercase();

        if response == "n" {
            println!("Operation cancelled by user, no further processing will be performed");
            exit(1);
        }
    }

    let mut _file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    println!("File created/truncated successfully.");

    Ok(())
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
