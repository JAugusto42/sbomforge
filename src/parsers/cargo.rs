use std::fs;
use toml;
use crate::sbom::{Dependency, SBOM};

pub fn parse_cargo_toml(file_path: &str) -> Result<SBOM, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(format!("{}/Cargo.toml", file_path))?;
    let parsed: toml::Value = toml::from_str(&content)?;

    let mut sbom = SBOM::new(
        parsed["package"]["name"].as_str().unwrap_or("unknown"),
        parsed["package"]["version"].as_str().unwrap_or("unknown")
    );

    // Add dependencies
    if let Some(deps) = parsed.get("dependencies") {
        for (name, version) in deps.as_table().unwrap().iter() {
            sbom.add_dependency(Dependency {
                name: name.clone(),
                version: version.as_str().unwrap_or("").to_string(),
                license: None,  // License could be fetched from external sources if needed
                dependencies: vec![],
            });
        }
    }

    Ok(sbom)
}
