extern crate clap;
use clap::Parser;
use semver::DisplayOptions;

#[derive(Parser, Debug)]
#[clap(name = "semver")]
pub struct Semver {
    /// Should remove the 'v' prefix
    #[clap(short, long)]
    pub remove_v_prefix: bool,

    /// Use a custom prefix
    #[clap(short, long, default_value = "")]
    pub prefix: String,

    /// Display output in a single line
    #[clap(short, long)]
    pub single_line: bool,

    /// Version to be parsed
    pub version: String,
}

impl From<Semver> for DisplayOptions {
    fn from(item: Semver) -> Self {
        DisplayOptions {
            prefix: item.prefix,
            remove_v_prefix: item.remove_v_prefix,
            single_line: item.single_line,
        }
    }
}
