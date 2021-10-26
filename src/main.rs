pub mod semver;

fn main() -> anyhow::Result<()> {
    println!("Classic semver parse:");
    semver::parse("v1.2.3")?.print(true);

    println!("\nSpecial semver parse:");
    semver::parse("v1.2.3-alpha")?.print(true);

    Ok(())
}
