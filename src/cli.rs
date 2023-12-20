use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// Pattern by which the executables will be filtered
    pub pattern: String,
    /// Maximum count of executables in output
    #[clap(short, long, default_value_t = 10)]
    pub max_files: usize,
    /// Whether or not to append empty strings if less than `max_files` files were found
    #[clap(short, long, default_value_t = false)]
    pub padding: bool,
}
