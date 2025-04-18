use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
    process::exit,
};

use anyhow::Result;
use csv::{Reader, Writer};

use crate::utils::interactions::confirm_overwrite;

pub fn make_input_reader(path: &PathBuf) -> Reader<File> {
    match csv::ReaderBuilder::new().flexible(true).from_path(path) {
        Ok(file_reader) => file_reader,
        Err(e) => {
            println!("Error creating file reader: {e}\n no further processinng will be performed");
            exit(1)
        }
    }
}

pub fn make_output_writer(path: &PathBuf) -> Writer<File> {
    match csv::Writer::from_path(path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error creating file writer: {e}\n no further processinng will be performed");
            exit(1);
        }
    }
}

pub fn create_file_with_dirs(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        confirm_overwrite(path)?;
    }

    let mut _file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    println!("File created/truncated successfully.");

    Ok(())
}
