use clap::Parser;

pub mod cmd;
pub mod semver;

fn main() -> anyhow::Result<()> {
    let opts = cmd::Semver::parse();
    semver::parse(&opts.version)?.print(opts.prefix, !opts.remove_v_prefix);
    Ok(())
}
