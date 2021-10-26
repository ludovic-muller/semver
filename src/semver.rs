use regex::Regex;

pub fn semver(version: &str, print_v_prefix: bool) {
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

    let major = &caps["major"];
    let minor = &caps["minor"];
    let patch = &caps["patch"];

    let mut prefix = "";

    if print_v_prefix {
        prefix = "v";
    }

    println!("{}{}", prefix, major);
    println!("{}{}.{}", prefix, major, minor);
    println!("{}{}.{}.{}", prefix, major, minor, patch);
}
