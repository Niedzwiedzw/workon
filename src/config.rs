use crate::error::{WorkonError, WorkonResult};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::PathBuf;

pub const CONFIG_DEFAULT_FILENAME: &str = "workon.yaml";
pub const APP_MANIFEST: [&str; 3] = ["com", "niedzwiedz", "workon"];

pub fn config_dir() -> WorkonResult<PathBuf> {
    let dir = ProjectDirs::from(APP_MANIFEST[0], APP_MANIFEST[1], APP_MANIFEST[2])
        .expect("configuration error :: failed to detect OS's default config directory")
        .config_dir()
        .to_path_buf();
    if !dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    Ok(dir)
}

pub fn config_path() -> WorkonResult<PathBuf> {
    let path = config_dir()?.join(CONFIG_DEFAULT_FILENAME);
    if !path.exists() {
        let content = serde_yaml::to_string(&WorkonConfig::default())?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(path)
}

macro_rules! assert_or {
    ($test:expr, $err:literal) => {
        if !$test {
            return Err(WorkonError::InvalidConfig($err));
        }
    };
}

pub trait Validate: Sized {
    fn validate(&self) -> WorkonResult<&Self>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerminalConfig {
    pub workdir: PathBuf,
    pub command: Vec<String>,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        let workdir = match directories::UserDirs::new() {
            Some(dirs) => dirs.home_dir().to_path_buf(),
            None => ["/"].iter().collect(),
        };
        Self {
            workdir,
            command: vec!["ls".to_string(), "-la".to_string()],
        }
    }
}

impl Validate for TerminalConfig {
    fn validate(&self) -> WorkonResult<&Self> {
        assert_or!(self.workdir.exists(), "project directory no longer exists");
        assert_or!(!self.command.is_empty(), "startup command needs to be specified");
        for segment in &self.command {
            assert_or!(!segment.is_empty(), "malformed command, a segment cannot be empty");
        }
        Ok(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub project_name: String,
    pub terminals: Vec<TerminalConfig>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            terminals: vec![Default::default()],
            project_name: "example project".to_string()
        }
    }
}

impl Validate for ProjectConfig {
    fn validate(&self) -> WorkonResult<&Self> {
        assert_or!(!self.project_name.is_empty(), "project name cannot be empty");
        let terminals = self.terminals.iter().map(|t| t.validate()).collect::<WorkonResult<Vec<_>>>()?;
        assert_or!(!terminals.is_empty(), "project must contain at least one application");
        Ok(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkonConfig {
    pub projects: Vec<ProjectConfig>,
}

impl Default for WorkonConfig {
    fn default() -> Self {
        Self {
            projects: vec![Default::default()],
        }
    }
}

impl WorkonConfig {
    pub fn current() -> WorkonResult<Self> {
        let config_path_ = config_path()?;
        eprintln!("using config: {}", config_path_.display());
        let content = std::fs::read_to_string(config_path_)?;
        let config = serde_yaml::from_str::<Self>(&content)?;
        config.validate()?;
        Ok(config)
    }
}

impl Validate for WorkonConfig {
    fn validate(&self) -> WorkonResult<&Self> {
        let projects = self
            .projects
            .iter()
            .map(|p| p.validate())
            .collect::<WorkonResult<Vec<_>>>()?;
        assert_or!(!projects.is_empty(), "no projects in config");
        Ok(self)
    }
}


#[cfg(test)]
mod test_default_config {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        assert!(WorkonConfig::default().validate().is_ok())
    }
}
