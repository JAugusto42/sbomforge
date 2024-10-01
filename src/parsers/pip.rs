use std::fs;
use std::error::Error;
use std::process::Command;
use crate::sbom::{Dependency, SBOM};
use regex::Regex;

/// public function who act as entry point for this module
pub fn parse_setup_py(file_path: &str) -> Result<SBOM, Box<dyn Error>> {
    let content = fs::read_to_string(&format!("{}/setup.py", file_path))?;
    // TODO maybe change this later to a better name ...
    let mut sbom = SBOM::new("PythonProject", "unknown");

    let requirements = content
        .lines()
        .skip_while(|line| !line.trim().starts_with("install_requires"))
        .skip(1)
        .take_while(|line| !line.trim().ends_with(']'))
        .map(|line| line.trim().trim_matches(|c| c == '[' || c == ']'))
        .collect::<Vec<&str>>()
        .join("");

    let re_version = Regex::new(r"(?P<version>\d+\.\d+\.\d+)").unwrap();
    let re_name = Regex::new(r"(?P<name>[\w-]+)\s*[><=]*\s*(?P<versao>\d+\.\d+\.\d+)?").unwrap();

    for requirement in requirements.split(',')
        .map(|s| s.trim().trim_matches(|c| c == '\'' || c == '"'))
    {
        let version = re_version
            .captures_iter(requirement)
            .filter_map(|caps| caps.name("version"))
            .map(|m| m.as_str().to_string())
            .next()
            .unwrap_or_default();

        let name = re_name
            .captures_iter(requirement)
            .filter_map(|caps| caps.name("name"))
            .map(|m| m.as_str().to_string())
            .next().unwrap_or_default();

        let mut dependency = Dependency {
            name: name.clone(),
            version,
            license: None,
            dependencies: vec![],
        };

        // Fetch sub-dependencies via pip
        let sub_deps = resolve_sub_dependencies(&name)?;
        dependency.dependencies.extend(sub_deps);

        if !dependency.name.is_empty() {
            sbom.add_dependency(dependency);
        }
    }

    if sbom.dependencies.is_empty() {
        eprintln!("No dependencies found in setup.py. Content:\n{}", content);
    }

    Ok(sbom)
}

/// Function to resolve sub-dependencies using pip
fn resolve_sub_dependencies(package_name: &str) -> Result<Vec<Dependency>, Box<dyn Error>> {
    let output = Command::new("pip")
        .args(&["show", package_name])
        .output()?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut dependencies = Vec::new();

    let re_dependency = Regex::new(r"^Requires:\s*(?P<deps>.+)$").unwrap();

    for line in output_str.lines() {
        if let Some(caps) = re_dependency.captures(line) {
            if let Some(deps) = caps.name("deps") {
                for dep in deps.as_str().split(',') {
                    let dep_name = dep.trim();
                    let version = resolve_dependency_version(dep_name)?;

                    if !dep_name.is_empty() && !version.is_empty() {
                        dependencies.push(Dependency {
                            name: dep_name.to_string(),
                            version,
                            license: None,
                            dependencies: vec![],
                        });
                    }
                }
            }
        }
    }

    Ok(dependencies)
}

/// Function to resolve the version of a dependency using pip
fn resolve_dependency_version(dep_name: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("pip")
        .args(&["show", dep_name])
        .output()?;

    if !output.status.success() {
        return Ok(String::new());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let re_version = Regex::new(r"^Version:\s*(?P<version>.+)$").unwrap();

    for line in output_str.lines() {
        if let Some(caps) = re_version.captures(line) {
            if let Some(version) = caps.name("version") {
                return Ok(version.as_str().to_string());
            }
        }
    }

    Ok(String::new())
}
