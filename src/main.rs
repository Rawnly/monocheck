use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

use clap::Parser;
use glob::glob;
use monocheck::models::file::*;
use monocheck::models::package_json::PackageJson;
use monocheck::models::workspace::Workspace;
use monocheck::{log, Args};
use prettytable::{row, Table};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Dependency {
    pub versions: HashSet<String>,
    pub workspaces: HashSet<String>,
}

impl Dependency {
    pub fn len(&self) -> usize {
        self.workspaces.len()
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            versions: HashSet::new(),
            workspaces: HashSet::new(),
        }
    }
}

impl Hash for Dependency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut versions: Vec<_> = self.versions.iter().collect();
        versions.sort();

        let mut workspaces: Vec<_> = self.workspaces.iter().collect();
        workspaces.sort();

        // hash the sorted versions and workspaces using `DefaultHasher`
        let mut v_hasher = DefaultHasher::new();
        versions.hash(&mut v_hasher);
        let v_hash = v_hasher.finish();

        let mut w_hasher = DefaultHasher::new();
        workspaces.hash(&mut w_hasher);
        let w_hash = w_hasher.finish();

        v_hash.hash(state);
        w_hash.hash(state);
    }
}

impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        let mut h1 = DefaultHasher::new();
        self.hash(&mut h1);

        let mut h2 = DefaultHasher::new();
        other.hash(&mut h2);

        h1.finish() == h2.finish()
    }
}

impl Eq for Dependency {}

type DependencyMap = HashMap<String, Dependency>;

fn add_to_dependency_map(
    map: &mut DependencyMap,
    package_name: &String,
    package_version: &String,
    workspace: &String,
    args: &Args,
) {
    let ignored_pkgs = args.ignore.clone().unwrap_or_default();

    // skip ignored packages
    if ignored_pkgs.contains(package_name) || args.ignore_workspace.contains(workspace) {
        return;
    }

    // search for workspace name matches
    if let Some(matches) = args.match_workspace.clone() {
        if !matches.is_match(&workspace) {
            return;
        }
    }

    // search for matches
    if let Some(matches) = args.matches.clone() {
        if !matches.is_match(package_name) {
            return;
        }
    }

    let mut dependency = map
        .entry(package_name.to_owned())
        .or_insert_with(Dependency::default);

    dependency.workspaces.insert(workspace.to_owned());
    dependency.versions.insert(package_version.to_owned());
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let workspace_file = Path::new("./pnpm-workspace.yaml");

    if !workspace_file.exists() {
        log::error("Workspace file not found", workspace_file);
        return Ok(());
    }

    let Workspace { packages } = Workspace::load(workspace_file)?;
    let mut dependency_map: DependencyMap = DependencyMap::new();

    for g in packages {
        for entry in glob(&g)? {
            let path = entry?;
            let package_json_path = path.join("package.json");

            let pkg = PackageJson::load(&package_json_path)?;

            let name = &pkg.name;

            for (pkg_name, version) in pkg.dependencies.0 {
                add_to_dependency_map(&mut dependency_map, &pkg_name, &version, &name, &args);
            }

            for (pkg_name, version) in pkg.dev_dependencies.0 {
                add_to_dependency_map(&mut dependency_map, &pkg_name, &version, &name, &args);
            }
        }
    }

    #[derive(Debug, Serialize)]
    struct JSONData {
        pub name: String,
        pub count: usize,
        pub workspaces: Vec<String>,
        pub versions: Vec<String>,
    }

    if args.json {
        // update result with array of packages taht have keys: name, workspaces and count
        let mut result: Vec<JSONData> = Vec::new();

        for (name, packages) in dependency_map {
            let count = packages.len();

            // ingore --min when deep is true
            if count < args.min {
                continue;
            }

            let workspaces = packages
                .clone()
                .workspaces
                .into_iter()
                .collect::<Vec<String>>();

            let versions = packages
                .clone()
                .versions
                .into_iter()
                .collect::<Vec<String>>();

            result.push(JSONData {
                name,
                count,
                workspaces,
                versions,
            });
        }

        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    let mut table = Table::new();

    if args.deep {
        table.add_row(row!["Dependency", "Count", "Versions", "Workspaces"]);
    } else {
        table.add_row(row!["Dependency", "Count", "Packages"]);
    }

    for (name, packages) in dependency_map {
        let count = packages.len();

        if count < args.min {
            continue;
        }

        let workspaces = packages
            .clone()
            .workspaces
            .into_iter()
            .collect::<Vec<String>>();

        let versions = packages
            .clone()
            .versions
            .into_iter()
            .collect::<Vec<String>>();

        if args.deep {
            table.add_row(row![name, count, versions.len(), workspaces.join(", ")]);
        } else {
            table.add_row(row![name, count, workspaces.join(", ")]);
        }
    }

    table.printstd();

    let total = table.row_iter().len() - 1;

    if total == 0 {
        println!("No duplicate dependencies found (min: {})", args.min);
        return Ok(());
    }

    // -1 because we have to remove header row
    println!("Total : {}", table.row_iter().len() - 1);
    Ok(())
}
