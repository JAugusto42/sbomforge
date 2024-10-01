use std::fs;
use crate::sbom::SBOM;
use serde_json;

pub fn generate_sbom_json(sbom: &SBOM) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(&sbom)?;
    fs::write("sbom.json", json_data)?;
    Ok(())
}
