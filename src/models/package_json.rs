use std::collections::HashMap;

use crate::models::file::File;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Dependencies(pub HashMap<String, String>);

impl Default for Dependencies {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Dependencies {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageJson {
    pub name: String,
    pub workspaces: Option<Vec<String>>,

    #[serde(default)]
    pub dependencies: Dependencies,

    #[serde(rename = "devDependencies", default)]
    pub dev_dependencies: Dependencies,

    #[serde(rename = "peerDependencies", default)]
    pub peer_dependencies: Dependencies,
}

impl File<PackageJson> for PackageJson {}
impl PackageJson {
    pub fn has_workspaces(self) -> bool {
        self.workspaces.unwrap().is_empty()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PnpmWorkspace {
    pub packages: Vec<String>,
}
impl File<PnpmWorkspace> for PnpmWorkspace {}

pub mod utils {
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use walkdir::WalkDir;

    pub fn search_file(search_path: &str, filename: &str) -> Option<PathBuf> {
        match fs::read_dir(search_path) {
            Err(_) => None,
            Ok(content) => {
                let mut fpath: Option<PathBuf> = None;

                for f in content {
                    let path = match f {
                        Err(_) => None,
                        Ok(entry) => Some(entry.path()),
                    };

                    if let Some(p) = path {
                        if p.is_file() && p.file_name().unwrap().to_str().unwrap() == filename {
                            fpath = Some(p);
                            break;
                        }
                    }
                }

                fpath
            }
        }
    }

    pub fn list_dirs(path: &Path) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(move |p| match p {
                Ok(entry) => Some(entry),
                _ => None,
            })
            .filter(|p| p.path().is_dir())
            .map(|f| PathBuf::from(f.path()))
            .collect()
    }

    pub fn list_files(path: &Path) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(move |p| match p {
                Ok(entry) => Some(entry),
                _ => None,
            })
            .filter(|p| p.path().is_file())
            .map(|f| PathBuf::from(f.path()))
            .collect()
    }
}
