use std::fs;
use crate::sbom::{Dependency, SBOM};

pub fn parse_pip_requirements(file_path: &str) -> Result<SBOM, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(format!("{}/requirements.txt", file_path))?;
    let mut sbom = SBOM::new("PythonProject", "unknown");

    for line in content.lines() {
        if let Some((name, version)) = line.split_once("==") {
            sbom.add_dependency(Dependency {
                name: name.to_string(),
                version: version.to_string(),
                license: None,  // Could integrate with PyPI to get license info
                dependencies: vec![],
            });
        }
    }

    Ok(sbom)
}
