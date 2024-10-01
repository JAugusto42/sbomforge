use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub license: Option<String>,
    pub dependencies: Vec<Dependency>,  // Recursive structure for nested dependencies
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SBOM {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

impl SBOM {
    pub fn new(name: &str, version: &str) -> Self {
        SBOM {
            name: name.to_string(),
            version: version.to_string(),
            dependencies: vec![],
        }
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }
}
