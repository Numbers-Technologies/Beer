use std::env;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use std::time::Duration;
use std::io::Read;

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

async fn find_package(package_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let formula_url = format!(
        "https://raw.githubusercontent.com/Numbers-Technologies/beer-formulaes/main/{}.formula.toml",
        package_name
    );
    let beer = "🍺";
    println!("{} {} Searching for formula: {}", beer.yellow(), "[FIND]".bold().green(), formula_url.cyan());
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}").unwrap());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Contacting GitHub...");
    let response = reqwest::get(&formula_url).await?;
    pb.finish_and_clear();

    if response.status().is_success() {
        let formula_content = response.text().await?;
        println!("{} {} Found formula for package '{}':", beer.green(), "[OK]".bold().green(), package_name.bold().yellow());
        println!("{}", formula_content.bright_white());
    } else {
        println!("{} {} Formula for package '{}' not found.", beer.red(), "[NOT FOUND]".bold().red(), package_name.bold().yellow());
    }
    Ok(())
}

async fn install_package(package_name: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let formula_url = format!(
        "https://raw.githubusercontent.com/Numbers-Technologies/beer-formulaes/main/{}.formula.toml",
        package_name
    );
    let beer = "🍺";
    println!("{} {} Searching for formula: {}", beer.yellow(), "[INSTALL]".bold().green(), formula_url.cyan());
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}").unwrap());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Contacting GitHub...");
    let response = reqwest::get(&formula_url).await?;
    pb.finish_and_clear();
    
    if response.status().is_success() {
        let formula_content = response.text().await?;
        println!("{} {} Found formula for package '{}':", beer.green(), "[OK]".bold().green(), package_name.bold().yellow());
        match toml::from_str::<Package>(&formula_content) {
            Ok(package) => {
                println!("{} {} Parsed package:", beer, "[PARSE]".bold().blue());
                println!("{:#?}", package);
                let clone_dir = format!("/opt/beerpm/Packages/{}", package_name);
                if Path::new(&clone_dir).exists() {
                    println!("{} {} Removing existing directory: {}", beer, "[CLEAN]".bold().yellow(), clone_dir.magenta());
                    let _ = fs::remove_dir_all(&clone_dir);
                }
                println!("{} {} Cloning {} into {}...", beer, "[GIT]".bold().cyan(), package.git_repository.cyan(), clone_dir.magenta());
                let pb = ProgressBar::new_spinner();
                pb.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}").unwrap());
                pb.enable_steady_tick(Duration::from_millis(100));
                pb.set_message("Cloning repository...");
                let status = Command::new("git")
                    .arg("clone")
                    .arg(&package.git_repository)
                    .arg(&clone_dir)
                    .status()
                    .await?;
                pb.finish_and_clear();
                if !status.success() {
                    println!("{} {} Failed to clone repository", beer.red(), "[FAIL]".bold().red());
                    return Ok(());
                }
                let total_cmds = package.formula.install_cmds.len() as u64;
                let pb = ProgressBar::new(total_cmds);
                pb.set_style(ProgressStyle::default_bar()
                    .template("{bar:40.cyan/blue} {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("🍺=>-"));
                for cmd in &package.formula.install_cmds {
                    pb.set_message(format!("Running: {}", cmd));
                    println!("{} {} Running install command: {}", beer, "[CMD]".bold().yellow(), cmd.bold().white());
                    let mut parts = cmd.split_whitespace();
                    if let Some(program) = parts.next() {
                        let args: Vec<&str> = parts.collect();
                        let mut command = Command::new(program);
                        command.args(&args).current_dir(&clone_dir);
                        if !verbose {
                            command.stdout(std::process::Stdio::null());
                            command.stderr(std::process::Stdio::null());
                        }
                        let status = command.status().await?;
                        if !status.success() {
                            println!("{} {} Command failed: {}", beer.red(), "[FAIL]".bold().red(), cmd.red());
                        }
                    }
                    pb.inc(1);
                }
                pb.finish_with_message("All install commands finished!");
                println!("{} {} Done!", beer.green(), "[DONE]".bold().green());
            }
            Err(e) => {
                println!("{} {} Could not parse package TOML: {}", beer.red(), "[FAIL]".bold().red(), e);
                println!("Raw content:");
                println!("{}", formula_content);
            }
        }
    } else {
        println!("{} {} Package '{}' not found in beer-formulaes repository", beer.red(), "[NOT FOUND]".bold().red(), package_name.bold().yellow());
        println!("Available packages can be found at: https://github.com/Numbers-Technologies/beer-formulaes");
    }
    
    Ok(())
}

fn print_info() {
    let beer = "🍺";
    println!("{} {} BeerPM Info:", beer.cyan(), "[INFO]".bold().cyan());

    // Read info.toml
    let info_path = "/opt/beerpm/info.toml";
    if Path::new(info_path).exists() {
        let mut info = String::new();
        if let Ok(mut f) = fs::File::open(info_path) {
            let _ = f.read_to_string(&mut info);
            println!("{} info.toml:
{}", beer, info.bright_white());
        }
    } else {
        println!("{} info.toml not found.", beer.red());
    }

    // List installed formulae
    let formulaes_path = "/opt/beerpm/Formulaes";
    if Path::new(formulaes_path).exists() {
        match fs::read_dir(formulaes_path) {
            Ok(entries) => {
                let formulaes: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                println!("{} Installed formulae ({}):", beer, formulaes.len());
                for f in &formulaes {
                    if let Some(name) = f.file_name().to_str() {
                        println!("  - {}", name.green());
                    }
                }
            }
            Err(e) => println!("{} Could not read Formulaes dir: {}", beer.red(), e),
        }
    } else {
        println!("{} No formulaes installed.", beer.yellow());
    }

    // Disk usage for Packages
    let packages_path = "/opt/beerpm/Packages";
    if Path::new(packages_path).exists() {
        if let Ok(output) = std::process::Command::new("du").arg("-sh").arg(packages_path).output() {
            if output.status.success() {
                let usage = String::from_utf8_lossy(&output.stdout);
                println!("{} Packages disk usage: {}", beer, usage.trim().magenta());
            }
        }
    }
}

fn print_help() {
    let beer = "🍺";
    println!("{} BeerPM - Yet another package manager for your system\n", beer.yellow());
    println!("USAGE:");
    println!("  beer install <package> [--verbose]   Install a package from the formulaes repo");
    println!("  beer find <package>                  Search for a package formula");
    println!("  beer --create-package <dir>           Create a new beer_package.toml in <dir>");
    println!("  beer info                            Show BeerPM installation info");
    println!("  beer help | --help | -h               Show this help message");
    println!("\nFLAGS:");
    println!("  --verbose                             Show output of install commands");
    println!("\nEXAMPLES:");
    println!("  beer install cmake");
    println!("  beer install llvm --verbose");
    println!("  beer find python3");
    println!("  beer info");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&"--verbose".to_string());
    let filtered_args: Vec<String> = args.iter().filter(|a| *a != "--verbose").cloned().collect();
    
    if filtered_args.len() < 2 || ["help", "--help", "-h"].contains(&filtered_args[1].as_str()) {
        print_help();
    } else if filtered_args.len() >= 3 && filtered_args[1] == "--create-package" {
        let directory = &filtered_args[2];
        create_package_file(directory);
    } else if filtered_args.len() >= 3 && filtered_args[1] == "install" {
        let package_name = &filtered_args[2];
        if let Err(e) = install_package(package_name, verbose).await {
            eprintln!("Error installing package: {}", e);
        }
    } else if filtered_args.len() >= 3 && filtered_args[1] == "find" {
        let package_name = &filtered_args[2];
        if let Err(e) = find_package(package_name).await {
            eprintln!("Package not found. {}", e);
        }
    } else if filtered_args.len() >= 2 && filtered_args[1] == "info" {
        print_info();
    } else {
        print_help();
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
