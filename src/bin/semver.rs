use clap::Parser;

use semver::{cmd, parse};

fn main() -> anyhow::Result<()> {
    let opts = cmd::Semver::parse();
    parse(&opts.version)?.print(opts);
    Ok(())
}
