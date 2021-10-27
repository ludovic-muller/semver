extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "semver")]
pub struct Semver {
    /// Should print the 'v' prefix
    #[clap(short, long)]
    pub remove_v_prefix: bool,

    /// Version to be parsed
    pub version: String,
}
