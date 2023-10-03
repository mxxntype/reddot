use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Pattern by which the executables will be filtered
    #[arg(short, long)]
    pub pattern: String,
    /// Maximum count of executables in output
    #[arg(short, long)]
    pub exe_count: usize,
}
