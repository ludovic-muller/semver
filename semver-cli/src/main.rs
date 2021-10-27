use clap::Parser;
use semver::{parse, DisplayOptions};

pub mod cmd;

fn main() -> anyhow::Result<()> {
    let opts = cmd::Semver::parse();
    parse(&opts.version)?.print(DisplayOptions::from(opts));
    Ok(())
}
