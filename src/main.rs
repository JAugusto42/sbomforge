mod sbom;
mod parsers;
mod output;

use clap::Parser;
use parsers::{cargo::parse_cargo_toml, npm::parse_npm_package_json, pip::parse_pip_requirements};
use sbom::SBOM;
use output::json::generate_sbom_json;

#[derive(Parser, Debug)]
#[command(name = "SBOMForge", version = "1.0", about = "Generates SBOM with License and Dependency Tree")]
struct Args {
    /// Path to the project folder
    #[arg(short, long)]
    path: String,

    /// Project type (e.g., "cargo", "npm", "pip")
    #[arg(short = 't', long)]
    project_type: String,
}

fn main() {
    let args = Args::parse();

    println!("Scanning project at: {}", args.path);
    println!("Project type: {}", args.project_type);

    let sbom = match args.project_type.as_str() {
        "cargo" => parse_cargo_toml(&args.path),
        "npm" => parse_npm_package_json(&args.path),
        "pip" => parse_pip_requirements(&args.path),
        _ => Err("Unsupported project type".into()),
    };

    match sbom {
        Ok(sbom) => {
            generate_sbom_json(&sbom).unwrap();
            println!("SBOM generated successfully!");
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
