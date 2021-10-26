pub mod semver;

fn main() -> anyhow::Result<()> {
    let v = semver::parse("v1.2.3")?;
    v.print(true);

    Ok(())
}
