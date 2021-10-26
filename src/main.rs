use regex::Regex;

fn main() {
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

    let version = "v1.2.3";
    let caps = re.captures(version).unwrap();

    let major = &caps["major"];
    let minor = &caps["minor"];
    let patch = &caps["patch"];

    println!("v{}", major);
    println!("v{}.{}", major, minor);
    println!("v{}.{}.{}", major, minor, patch);
}
