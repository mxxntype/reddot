use std::os::unix::prelude::PermissionsExt;
use std::{env, fs};

mod cli;
use clap::Parser;
use cli::Args;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let path = env::var("PATH")?;
    let args = Args::parse();

    let mut executable_files: Vec<fs::DirEntry> = vec![];
    path.split(':').for_each(|dir| {
        if let Ok(files) = fs::read_dir(dir) {
            for exe in find_executables(files) {
                executable_files.push(exe);
            }
        }
    });

    executable_files.retain(|file: &fs::DirEntry| {
        file.file_name()
            .to_string_lossy()
            .to_string()
            .to_lowercase()
            .contains(&args.pattern.to_lowercase())
    });

    executable_files.truncate(args.exe_count);

    print_filenames_json(&executable_files);

    Ok(())
}

/// Finds files that can be executed by any user
fn find_executables(directory: fs::ReadDir) -> Vec<fs::DirEntry> {
    let mut accumulator = vec![];
    directory.flatten().for_each(|file| {
        if file.is_executable() {
            accumulator.push(file);
        }
    });
    accumulator
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

trait CheckExecutePermission {
    /// Returns `true` if the file is executable, `false` otherwise
    fn is_executable(&self) -> bool;
}

impl CheckExecutePermission for fs::DirEntry {
    fn is_executable(&self) -> bool {
        self.metadata()
            .map_or(false, |md| md.permissions().mode() & 0o111 != 0)
    }
}
