use clap::Parser;
use clap_verbosity_flag::{ErrorLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity<ErrorLevel>,
}
