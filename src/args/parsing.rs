use std::{path::PathBuf, process::exit};

use anyhow::Result;
use serde::{Deserialize, Deserializer};

use crate::csv_lc::serialize::SecretData;

pub fn parse_input_path(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if !path.exists() {
        eprintln!("Input file does not exist in provided path");
        exit(1)
    }
    Ok(path)
}

pub fn parse_output_path(s: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(s))
}

pub fn parse_secret_data<'de, D>(de: D) -> Result<SecretData, D::Error>
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
        _secret_type: secret_type,
        username,
        password,
    })
}
