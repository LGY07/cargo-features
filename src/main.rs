mod metadata;

use metadata::Metadata;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let raw = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--all-features")
        .output()?;

    let raw = String::from_utf8(raw.stdout)?;

    let metadata:Metadata = raw.parse()?;

    metadata.show_features();

    Ok(())
}
