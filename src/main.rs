use std::env;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct Formula
{
    pub install_cmds: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub git_repository: String,
    pub dependencies: Vec<String>,
    pub formula: Formula,
}

pub trait IFormula
{
    fn new(install_cmds: Vec<String>) -> Self;
    fn run_cmds(&self) -> std::io::Result<()>;
}

pub trait IPackage
{
    fn new(name: &str, git_repository: &str, dependencies: Vec<String>, formula: Formula) -> Self;
    fn empty_package() -> Self;
    fn install_dependencies(&self) -> std::io::Result<()>;
    fn run_formula(&self) -> std::io::Result<()>;
}

impl IFormula for Formula {
    fn new(install_cmds: Vec<String>) -> Self {
        Self { install_cmds }
    }
    fn run_cmds(&self) -> std::io::Result<()>
    {
        for i in &self.install_cmds {
            println!("{:#?}", i);
        }
        Ok(())
    }
}


impl IPackage for Package
{
    fn new(name: &str, git_repository: &str, dependencies: Vec<String>, formula: Formula) -> Self {
        Package { 
            name: name.to_string(),
            git_repository: git_repository.to_string(), 
            dependencies, 
            formula 
        }
    }
    fn empty_package() -> Self {
        Self { 
            name: "default".to_string(), 
            git_repository: "empty".to_string(), 
            dependencies: vec![], 
            formula: Formula::new(vec!["empty".to_string()])
        }
    }
    fn install_dependencies(&self) -> std::io::Result<()>
    {
        for i in &self.dependencies {
            println!("{:#?}", i);
        }
        Ok(())
    }
    fn run_formula(&self) -> std::io::Result<()> {
        println!("{:#?}", self.formula);
        Ok(())
    }
}

async fn install_package(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let formula_url = format!(
        "https://raw.githubusercontent.com/Numbers-Technologies/beer-formulaes/main/{}.formula.toml",
        package_name
    );
    
    println!("Searching for formula: {}", formula_url);
    
    let response = reqwest::get(&formula_url).await?;
    
    if response.status().is_success() {
        let formula_content = response.text().await?;
        println!("Found formula for package '{}':", package_name);
        println!("{}", formula_content);
        
        match toml::from_str::<Package>(&formula_content) {
            Ok(package) => {
                println!("\nParsed package:");
                println!("{:#?}", package);

                let clone_dir = format!("/tmp/{}", package_name);
                if Path::new(&clone_dir).exists() {
                    println!("Removing existing directory: {}", clone_dir);
                    let _ = fs::remove_dir_all(&clone_dir);
                }
                println!("Cloning {} into {}...", package.git_repository, clone_dir);
                let status = Command::new("git")
                    .arg("clone")
                    .arg(&package.git_repository)
                    .arg(&clone_dir)
                    .status()
                    .await?;
                if !status.success() {
                    println!("Failed to clone repository");
                    return Ok(());
                }
                for cmd in &package.formula.install_cmds {
                    println!("Running install command: {}", cmd);
                    let mut parts = cmd.split_whitespace();
                    if let Some(program) = parts.next() {
                        let args: Vec<&str> = parts.collect();
                        let status = Command::new(program)
                            .args(&args)
                            .current_dir(&clone_dir)
                            .status()
                            .await?;
                        if !status.success() {
                            println!("Command failed: {}", cmd);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Warning: Could not parse package TOML: {}", e);
                println!("Raw content:");
                println!("{}", formula_content);
            }
        }
    } else {
        println!("Package '{}' not found in beer-formulaes repository", package_name);
        println!("Available packages can be found at: https://github.com/Numbers-Technologies/beer-formulaes");
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 3 && args[1] == "--create-package" {
        let directory = &args[2];
        create_package_file(directory);
    } else if args.len() >= 3 && args[1] == "install" {
        let package_name = &args[2];
        if let Err(e) = install_package(package_name).await {
            eprintln!("Error installing package: {}", e);
        }
    } else {
        let bsd_book_formula = Formula::new(vec!["make".to_string()]);
        let package = Package::new("BSDBook", "github.com/TwelveFacedJanus/BSDBook.git", vec![], bsd_book_formula);
        println!("PACKAGE: {:#?}", package);
        package.install_dependencies();
        package.run_formula();
    }
}

fn create_package_file(directory: &str) {
    let package_name = if directory == "." {
        env::current_dir()
            .ok()
            .and_then(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
            .unwrap_or_else(|| "unknown-package".to_string())
    } else {
        Path::new(directory)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-package")
            .to_string()
    };
    
    let package_content = format!(r#"# Beer Package Configuration
name = "{}"
git_repository = "https://github.com/username/repo.git"
dependencies = []

[formula]
install_cmds = []
"#, package_name);

    let package_path = Path::new(directory).join("beer_package.toml");
    
    match fs::write(&package_path, package_content) {
        Ok(_) => println!("Created beer_package.toml in {} with package name '{}'", directory, package_name),
        Err(e) => eprintln!("Error creating beer_package.toml: {}", e),
    }
}
