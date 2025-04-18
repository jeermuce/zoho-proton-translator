use std::{
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

use anyhow::Result;

pub fn confirm_overwrite(path: &PathBuf) -> Result<()> {
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
    Ok(())
}
