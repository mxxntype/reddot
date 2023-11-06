use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// Pattern by which the executables will be filtered
    pub pattern: String,
    /// Maximum count of executables in output
    #[clap(short, long, default_value_t = usize::MAX)]
    pub max_files: usize,
}
