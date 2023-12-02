use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub edition: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Option<Package>,
    pub depenedencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub members: Option<Vec<String>>,
}
