use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Pattern by which the executables will be filtered
    #[clap(short, long)]
    pub pattern: String,
    /// Maximum count of executables in output
    #[clap(short, long)]
    pub exe_count: usize,
}
