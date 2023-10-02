use std::os::unix::prelude::PermissionsExt;
use std::{env, fs};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let path = env::var("PATH")?;

    let mut executables: Vec<fs::DirEntry> = vec![];
    path.split(':').for_each(|dir| {
        if let Ok(files) = fs::read_dir(dir) {
            for exe in find_executables(files) {
                executables.push(exe);
            }
        }
    });

    match env::args().nth(1) {
        Some(pattern) => {
            for basename in executables.iter().map(Basename::basename) {
                let basename_lowercase = basename.to_lowercase();
                let matches: Vec<&str> = basename_lowercase
                    .matches(pattern.to_lowercase().as_str())
                    .collect();
                if !matches.is_empty() {
                    println!("{basename}");
                }
            }
        }
        None => print_file_names(executables),
    }

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

/// Prints the files' basenames on newlines
fn print_file_names(files: Vec<fs::DirEntry>) {
    files
        .iter()
        .for_each(|file| println!("{}", file.basename()));
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

trait Basename {
    /// Returns the file's basename (everything **after** the last `/`)
    fn basename(&self) -> String;
}

impl Basename for fs::DirEntry {
    fn basename(&self) -> String {
        self.path()
            .display()
            .to_string()
            .split('/')
            .last()
            .unwrap_or_default()
            .to_string()
    }
}
