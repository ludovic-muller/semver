extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "semver")]
pub struct Semver {
    /// Should remove the 'v' prefix
    #[clap(short, long)]
    pub remove_v_prefix: bool,

    /// Use a custom prefix
    #[clap(short, long, default_value = "")]
    pub prefix: String,

    /// Version to be parsed
    pub version: String,
}
