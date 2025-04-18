use std::{path::PathBuf, process::exit};

pub fn unwrap_path(path: &Option<PathBuf>) -> &PathBuf {
    match path {
        Some(p) => p,
        None => {
            println!("Output file must be specified if --write is set.\n no further processinng will be performed");
            exit(1);
        }
    }
}
