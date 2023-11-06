use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// Pattern by which the executables will be filtered
    #[clap(short, long)]
    pub pattern: String,
    /// Maximum count of executables in output
    #[clap(short, long)]
    pub max_files: usize,
}
