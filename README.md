# SBOMForge

SBOMForge is a fast, extensible, and generic tool written in Rust for generating Software Bill of Materials (SBOM) with license and dependency tree information. It supports multiple project types, such as Cargo, NPM, and Pip, making it adaptable to various ecosystems. SBOMForge helps developers and organizations ensure better security and compliance by producing detailed SBOM reports in JSON format.

## Features

- **Multi-language support**: Analyze dependencies from multiple ecosystems (e.g., Cargo, NPM, Pip).
- **License detection**: Extract license information from project dependencies.
- **Dependency tree**: Generate a full dependency tree for your projects.
- **SBOM output**: Export Software Bill of Materials in JSON format.
- **High performance**: Built with Rust for speed and safety.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Supported Project Types](#supported-project-types)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Prerequisites

- **Rust**: You need to have Rust installed on your system. Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust.

After installing Rust, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/SBOMForge.git
cd SBOMForge
cargo build --release
```

## Usage
```bash
SBOMForge -p <path-to-project> -t <project-type>
```

### Command-line Options
- ```-p, --path <PATH>: The path to the project folder. (Required)```
- ```-t, --project_type <TYPE>: The type of project (e.g., cargo, npm, pip). (Required)```

## Help

```bash
SBOMForge --help
```
This will display help and usage information.

## Supported Project Types
SBOMForge supports the following types of projects:

- Cargo (Rust projects with a Cargo.toml file)
- NPM (JavaScript/Node.js projects with a package.json file)
- Pip (Python projects with requirements.txt or setup.py files)

## Examples
### Example 1: Generate an SBOM for a Cargo project

```bash
SBOMForge -p ./my_cargo_project -t cargo
```

### Example 2: Generate an SBOM for an NPM project
```bash
SBOMForge -p ./my_npm_project -t npm
```

### Example 3: Generate an SBOM for a Pip project
```bash
SBOMForge -p ./my_python_project -t pip
```
This will scan the projects and output the SBOM.

## Output Format
The generated SBOM file will include:
- Project Name
- List of dependencies: Each dependency will have its own entry with the following details:
  - Name
  - Version
  - License information
  - Dependency tree

## License
This project is licensed under the MIT License - see the LICENSE file for details.