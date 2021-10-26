use anyhow::Context;
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

pub fn parse(version: &str) -> anyhow::Result<Semver> {
    let re = Regex::new(
        r"^(?x)v?
(?P<major>(0|[1-9]\d*))  # major
\.
(?P<minor>(0|[1-9]\d*))  # minor
\.
(?P<patch>(0|[1-9]\d*))  # patch
$",
    )?;

    let caps = re.captures(version).context("invalid semver")?;

    let major = caps["major"].to_string();
    let minor = caps["minor"].to_string();
    let patch = caps["patch"].to_string();

    Ok(Semver {
        major,
        minor,
        patch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let v = parse("1.2.3")?;

        assert_eq!(v.major, "1");
        assert_eq!(v.minor, "2");
        assert_eq!(v.patch, "3");

        Ok(())
    }

    #[test]
    fn test_prefix_parse() -> anyhow::Result<()> {
        let v = parse("v1.2.3")?;

        assert_eq!(v.major, "1");
        assert_eq!(v.minor, "2");
        assert_eq!(v.patch, "3");

        Ok(())
    }

    #[test]
    fn test_valid_parse() {
        assert!(parse("0.0.4").is_ok());
        assert!(parse("1.2.3").is_ok());
        assert!(parse("10.20.30").is_ok());
        // assert!(parse("1.1.2-prerelease+meta").is_ok());
        // assert!(parse("1.1.2+meta").is_ok());
        // assert!(parse("1.1.2+meta-valid").is_ok());
        // assert!(parse("1.0.0-alpha").is_ok());
        // assert!(parse("1.0.0-beta").is_ok());
        // assert!(parse("1.0.0-alpha.beta").is_ok());
        // assert!(parse("1.0.0-alpha.beta.1").is_ok());
        // assert!(parse("1.0.0-alpha.1").is_ok());
        // assert!(parse("1.0.0-alpha0.valid").is_ok());
        // assert!(parse("1.0.0-alpha.0valid").is_ok());
        // assert!(parse("1.0.0-alpha-a.b-c-somethinglong+build.1-aef.1-its-okay").is_ok());
        // assert!(parse("1.0.0-rc.1+build.1").is_ok());
        // assert!(parse("2.0.0-rc.1+build.123").is_ok());
        // assert!(parse("1.2.3-beta").is_ok());
        // assert!(parse("10.2.3-DEV-SNAPSHOT").is_ok());
        // assert!(parse("1.2.3-SNAPSHOT-123").is_ok());
        assert!(parse("1.0.0").is_ok());
        assert!(parse("2.0.0").is_ok());
        assert!(parse("1.1.7").is_ok());
        // assert!(parse("2.0.0+build.1848").is_ok());
        // assert!(parse("2.0.1-alpha.1227").is_ok());
        // assert!(parse("1.0.0-alpha+beta").is_ok());
        // assert!(parse("1.2.3----RC-SNAPSHOT.12.9.1--.12+788").is_ok());
        // assert!(parse("1.2.3----R-S.12.9.1--.12+meta").is_ok());
        // assert!(parse("1.2.3----RC-SNAPSHOT.12.9.1--.12").is_ok());
        // assert!(parse("1.0.0+0.build.1-rc.10000aaa-kk-0.1").is_ok());
        // assert!(parse("99999999999999999999999.999999999999999999.99999999999999999").is_ok());
        // assert!(parse("1.0.0-0A.is.legal").is_ok());
    }

    #[test]
    fn test_invalid_parse() {
        assert!(parse("1").is_err());
        assert!(parse("1.2").is_err());
        assert!(parse("1.2.3-0123").is_err());
        assert!(parse("1.2.3-0123.0123").is_err());
        assert!(parse("1.1.2+.123").is_err());
        assert!(parse("+invalid").is_err());
        assert!(parse("-invalid").is_err());
        assert!(parse("-invalid+invalid").is_err());
        assert!(parse("-invalid.01").is_err());
        assert!(parse("alpha").is_err());
        assert!(parse("alpha.beta").is_err());
        assert!(parse("alpha.beta.1").is_err());
        assert!(parse("alpha.1").is_err());
        assert!(parse("alpha+beta").is_err());
        assert!(parse("alpha_beta").is_err());
        assert!(parse("alpha.").is_err());
        assert!(parse("alpha..").is_err());
        assert!(parse("beta").is_err());
        assert!(parse("1.0.0-alpha_beta").is_err());
        assert!(parse("-alpha.").is_err());
        assert!(parse("1.0.0-alpha..").is_err());
        assert!(parse("1.0.0-alpha..1").is_err());
        assert!(parse("1.0.0-alpha...1").is_err());
        assert!(parse("1.0.0-alpha....1").is_err());
        assert!(parse("1.0.0-alpha.....1").is_err());
        assert!(parse("1.0.0-alpha......1").is_err());
        assert!(parse("1.0.0-alpha.......1").is_err());
        assert!(parse("01.1.1").is_err());
        assert!(parse("1.01.1").is_err());
        assert!(parse("1.1.01").is_err());
        assert!(parse("1.2").is_err());
        assert!(parse("1.2.3.DEV").is_err());
        assert!(parse("1.2-SNAPSHOT").is_err());
        assert!(parse("1.2.31.2.3----RC-SNAPSHOT.12.09.1--..12+788").is_err());
        assert!(parse("1.2-RC-SNAPSHOT").is_err());
        assert!(parse("-1.0.3-gamma+b7718").is_err());
        assert!(parse("+justmeta").is_err());
        assert!(parse("9.8.7+meta+meta").is_err());
        assert!(parse("9.8.7-whatever+meta+meta").is_err());
        assert!(parse("99999999999999999999999.999999999999999999.99999999999999999----RC-SNAPSHOT.12.09.1--------------------------------..12").is_err());
    }
}
