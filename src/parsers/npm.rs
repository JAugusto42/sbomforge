use std::fs;
use serde_json::Value;
use crate::sbom::{Dependency, SBOM};

pub fn parse_npm_package_json(file_path: &str) -> Result<SBOM, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(format!("{}/package.json", file_path))?;
    let parsed: Value = serde_json::from_str(&content)?;

    let mut sbom = SBOM::new(
        parsed["name"].as_str().unwrap_or("unknown"),
        parsed["version"].as_str().unwrap_or("unknown")
    );

    // Add dependencies
    if let Some(deps) = parsed["dependencies"].as_object() {
        for (name, version) in deps.iter() {
            sbom.add_dependency(Dependency {
                name: name.clone(),
                version: version.as_str().unwrap_or("").to_string(),
                license: None,  // License could be fetched from npm registry
                dependencies: vec![],
            });
        }
    }

    Ok(sbom)
}
