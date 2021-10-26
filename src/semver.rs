use regex::Regex;

#[derive(Debug)]
pub struct Semver {
    major: String,
    minor: String,
    patch: String,
}

impl Semver {
    pub fn print(&self, print_v_prefix: bool) {
        let mut prefix = "";

        if print_v_prefix {
            prefix = "v";
        }

        println!("{}{}", prefix, &self.major);
        println!("{}{}.{}", prefix, &self.major, &self.minor);
        println!("{}{}.{}.{}", prefix, &self.major, &self.minor, &self.patch);
    }
}

pub fn parse(version: &str) -> Semver {
    let re = Regex::new(
        r"(?x)v?
(?P<major>0|[1-9]\d*)  # major
\.
(?P<minor>0|[1-9]\d*)  # minor
\.
(?P<patch>0|[1-9]\d*)  # patch
",
    )
    .unwrap();

    let caps = re.captures(version).unwrap();

    let major = caps["major"].to_string();
    let minor = caps["minor"].to_string();
    let patch = caps["patch"].to_string();

    Semver {
        major,
        minor,
        patch,
    }
}
