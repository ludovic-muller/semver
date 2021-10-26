pub mod semver;

fn main() {
    let v = semver::parse("v1.2.3");
    v.print(true);
}
