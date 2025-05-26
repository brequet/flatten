use clap::Parser;

mod cli;
mod error;
mod ignore_handler;
mod output;
mod processor;

use cli::Cli;
use processor::FileProcessor;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let processor = FileProcessor::new(cli)?;
    processor.process()
}
