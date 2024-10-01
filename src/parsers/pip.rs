use std::process::{Command, Stdio};
use crate::sbom::{Dependency, SBOM};

pub fn parse_setup_py(file_path: &str) -> Result<SBOM, Box<dyn std::error::Error>> {
    let output = Command::new("python3")
        .arg(format!("{}/setup.py", file_path))
        .arg("install_requires")
        .output()?;

    // Check if the command executed successfully
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to execute setup.py: {}", stderr).into());
    }

    let dependencies = String::from_utf8_lossy(&output.stdout);
    let mut sbom = SBOM::new("PythonProject", "unknown");

    for line in dependencies.lines() {
        let line = line.trim();
        if !line.is_empty() {
            sbom.add_dependency(Dependency {
                name: line.to_string(),
                version: "".to_string(), // Leave version blank if not specified
                license: None,           // Leave license as None for now
                dependencies: vec![],    // Could expand this later if needed
            });
        }
    }

    Ok(sbom)
}
