use std::os::unix::prelude::PermissionsExt;
use std::{env, fs};

mod cli;
use clap::Parser;
use cli::Config;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let path = env::var("PATH")?;
    let config = Config::parse();

    let mut executable_files: Vec<fs::DirEntry> = path
        .split(':')
        .filter_map(|dir| fs::read_dir(dir).ok())
        .flat_map(find_executables)
        .collect();

    executable_files.retain(|file: &fs::DirEntry| {
        let filename = file.file_name();
        let is_hidden = filename.to_str().is_some_and(|s| s.starts_with('.'));
        let matches_pattern = filename
            .to_string_lossy()
            .to_lowercase()
            .contains(&config.pattern.to_lowercase());
        matches_pattern && !is_hidden
    });

    executable_files.truncate(config.max_files);

    print_filenames_json(&executable_files);

    Ok(())
}

/// Finds files that can be executed by any user
fn find_executables(directory: fs::ReadDir) -> Vec<fs::DirEntry> {
    directory
        .flatten()
        .filter(ExecutePermission::is_executable)
        .collect()
}

/// Prints the files' basenames as a JSON array
fn print_filenames_json(files: &[fs::DirEntry]) {
    let filenames: Vec<String> = files
        .iter()
        .map(|f| f.file_name().to_string_lossy().to_string())
        .collect();
    let filenames_json = serde_json::to_string(&filenames).unwrap();
    println!("{filenames_json}");
}

trait ExecutePermission {
    /// Returns `true` if the file is executable, `false` otherwise
    fn is_executable(&self) -> bool;
}

impl ExecutePermission for fs::DirEntry {
    fn is_executable(&self) -> bool {
        self.metadata()
            .map_or(false, |md| md.permissions().mode() & 0o111 != 0)
    }
}
