use std::{path::PathBuf, process::exit};

use anyhow::Result;
use serde::{Deserialize, Deserializer};

use crate::SecretData;

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

pub fn csv_str_to_vec<'de, D>(de: D) -> Result<Vec<String>, D::Error>
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
        secret_type,
        username,
        password,
    })
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

pub fn unwrap_path(path: &Option<PathBuf>) -> &PathBuf {
    match path {
        Some(p) => p,
        None => {
            println!("Output file must be specified if --write is set.\n no further processinng will be performed");
            exit(1);
        }
    }
}
