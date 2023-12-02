use serde::Deserialize;

use super::file::*;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub packages: Vec<String>,
}

impl File<Workspace> for Workspace {}
