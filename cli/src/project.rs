use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SrishtiProject {
    pub project: ProjectConfig,
    pub runtime: Option<RuntimeConfig>,
    pub build: Option<BuildConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub entry: Option<String>,
    pub authors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub memory_backend: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub output: Option<String>,
    pub target: Option<String>,
}

pub fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;

    loop {
        let manifest_path = current.join("srishti.toml");
        if manifest_path.exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

pub fn load_project() -> Option<SrishtiProject> {
    let root = find_project_root()?;
    let manifest_path = root.join("srishti.toml");
    let content = fs::read_to_string(manifest_path).ok()?;
    toml::from_str(&content).ok()
}
