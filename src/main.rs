use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Formula
{
    pub install_cmds: Vec<String>,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub git_repository: String,
    pub dependencies: Vec<Package>,
    pub formula: Formula,
}

pub trait IFormula
{
    fn new(install_cmds: Vec<String>) -> Self;
    fn run_cmds(&self) -> std::io::Result<()>;
}

pub trait IPackage
{
    fn new(name: &str, git_repository: &str, dependencies: Vec<Package>, formula: Formula) -> Self;
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
    fn new(name: &str, git_repository: &str, dependencies: Vec<Package>, formula: Formula) -> Self {
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


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 3 && args[1] == "--create-package" {
        let directory = &args[2];
        create_package_file(directory);
    } else {
        let bsd_book_formula = Formula::new(vec!["make".to_string()]);
        let package = Package::new("BSDBook", "github.com/TwelveFacedJanus/BSDBook.git", vec![], bsd_book_formula);
        println!("PACKAGE: {:#?}", package);
        package.install_dependencies();
        package.run_formula();
    }
}

fn create_package_file(directory: &str) {
    // Extract directory name from path
    let package_name = if directory == "." {
        // For current directory, get the actual directory name
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
