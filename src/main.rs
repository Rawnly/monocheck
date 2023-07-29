use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

use clap::Parser;
use colored_json::ToColoredJson;
use glob::glob;
use monocheck::models::file::*;
use monocheck::models::package_json::PackageJson;
use monocheck::models::semantic_version::*;
use monocheck::models::workspace::Workspace;

use monocheck::{log, Action, Args};

use prettytable::{row, Table};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct Dependency {
    pub versions: HashSet<String>,
    pub workspaces: HashSet<String>,
}

impl Dependency {
    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
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
    let version = match package_version.as_ref() {
        "workspace:^" => "workspace".to_string(),
        "workspace:*" => "workspace".to_string(),
        v => v.replace('^', ""),
    };

    if version == "workspace" && !args.include_root {
        return;
    }

    let ignored_pkgs = args.ignore.clone().unwrap_or_default();

    // skip ignored packages
    if ignored_pkgs.contains(package_name) || args.ignore_workspace.contains(workspace) {
        return;
    }

    // search for workspace name matches
    if let Some(matches) = args.match_workspace.clone() {
        if !matches.is_match(workspace) {
            return;
        }
    }

    // search for matches
    if let Some(matches) = args.matches.clone() {
        if !matches.is_match(package_name) {
            return;
        }
    }

    let dependency = map
        .entry(package_name.to_owned())
        .or_insert_with(Dependency::default);

    dependency.workspaces.insert(workspace.to_owned());

    dependency.versions.insert(version);
}

#[derive(Debug, Serialize)]
struct JSONData {
    pub name: String,
    pub count: usize,
    pub workspaces: Vec<String>,
    pub versions: Vec<String>,
}

fn search_deps(
    manifest: &PackageJson,
    map: &mut HashMap<String, HashSet<String>>,
    args: &Args,
    value: &regex::Regex,
) {
    if args.prod || !args.dev && !args.peer {
        for (name, version) in manifest.dependencies.0.clone() {
            // search dependency
            if !value.is_match(&name) {
                continue;
            }

            let version = match version.as_ref() {
                "workspace:^" => "workspace".to_string(),
                "workspace:*" => "workspace".to_string(),
                v => v.replace('^', ""),
            };

            let pkg_name = if args.deep {
                format!("{}@{}", name, version)
            } else {
                name
            };

            map.entry(pkg_name)
                .or_insert_with(HashSet::new)
                .insert(manifest.name.clone());
        }
    }

    if args.dev {
        for (name, version) in manifest.dev_dependencies.0.clone() {
            // search dependency
            if !value.is_match(&name) {
                continue;
            }

            let version = match version.as_ref() {
                "workspace:^" => "workspace".to_string(),
                "workspace:*" => "workspace".to_string(),
                v => v.replace('^', ""),
            };

            let pkg_name = if args.deep {
                format!("{}@{}", name, version)
            } else {
                name
            };

            map.entry(pkg_name)
                .or_insert_with(HashSet::new)
                .insert(manifest.name.clone());
        }
    }

    if args.peer {
        for (name, version) in manifest.peer_dependencies.0.clone() {
            // search dependency
            if !value.is_match(&name) {
                continue;
            }

            let version = match version.as_ref() {
                "workspace:^" => "workspace".to_string(),
                "workspace:*" => "workspace".to_string(),
                v => v.replace('^', ""),
            };

            let pkg_name = if args.deep {
                format!("{}@{}", name, version)
            } else {
                name
            };

            map.entry(pkg_name)
                .or_insert_with(HashSet::new)
                .insert(manifest.name.clone());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let workspace_file = Path::new("pnpm-workspace.yaml");

    if !workspace_file.exists() {
        log::error("Workspace file not found", workspace_file);
        return Ok(());
    }

    match args.action.clone() {
        Some(action) => match action {
            Action::Search { value } => {
                // prints to the stdout where the package is installed.
                // HashMap<package_name, HashSet<workspace_name>>
                let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

                let Workspace { packages } = Workspace::load(workspace_file)?;
                let root_manifest = PackageJson::load(Path::new("package.json"))?;

                if args.include_root {
                    search_deps(&root_manifest, &mut dependencies, &args, &value);
                }

                for g in packages {
                    for entry in glob(&g)? {
                        let path = entry?;
                        let package_json_path = path.join("package.json");

                        let manifest = PackageJson::load(&package_json_path)?;

                        search_deps(&manifest, &mut dependencies, &args, &value);
                    }
                }

                for (pkg_name, workspaces) in dependencies {
                    #[derive(Serialize, Debug)]
                    struct Data {
                        name: String,
                        workspaces: Vec<String>,
                    }

                    let data = Data {
                        name: pkg_name,
                        workspaces: workspaces.into_iter().collect(),
                    };

                    println!("{}", serde_json::to_string(&data)?.to_colored_json_auto()?);
                }
            }
        },
        None => {
            let Workspace { packages } = Workspace::load(workspace_file)?;
            let mut dependency_map: DependencyMap = DependencyMap::new();

            for g in packages {
                for entry in glob(&g)? {
                    let path = entry?;
                    let package_json_path = path.join("package.json");

                    let pkg = PackageJson::load(&package_json_path)?;

                    let name = &pkg.name;

                    if args.prod || !args.dev {
                        for (pkg_name, version) in pkg.dependencies.0 {
                            add_to_dependency_map(
                                &mut dependency_map,
                                &pkg_name,
                                &version,
                                name,
                                &args,
                            );
                        }
                    }

                    if args.dev {
                        for (pkg_name, version) in pkg.dev_dependencies.0 {
                            add_to_dependency_map(
                                &mut dependency_map,
                                &pkg_name,
                                &version,
                                name,
                                &args,
                            );
                        }
                    }
                }
            }

            if args.check_workspace {
                let pkg = PackageJson::load(Path::new("./package.json"))?;

                let name = &pkg.name;

                if args.prod || !args.dev {
                    for (pkg_name, version) in pkg.dependencies.0 {
                        add_to_dependency_map(
                            &mut dependency_map,
                            &pkg_name,
                            &version,
                            name,
                            &args,
                        );
                    }
                }

                if args.dev {
                    for (pkg_name, version) in pkg.dev_dependencies.0 {
                        add_to_dependency_map(
                            &mut dependency_map,
                            &pkg_name,
                            &version,
                            name,
                            &args,
                        );
                    }
                }
            }

            // raw output
            if args.json || args.yaml {
                // update result with array of packages taht have keys: name, workspaces and count
                let mut result: Vec<JSONData> = Vec::new();

                for (name, packages) in dependency_map {
                    let count = packages.len();

                    // ingore --min when deep is true
                    if count < args.min {
                        continue;
                    }

                    let mut workspaces = packages
                        .clone()
                        .workspaces
                        .into_iter()
                        .collect::<Vec<String>>();

                    workspaces.sort();

                    let mut versions = packages
                        .clone()
                        .versions
                        .into_iter()
                        .map(SemanticVersion::from)
                        .collect::<Vec<SemanticVersion>>();

                    versions.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    result.push(JSONData {
                        name,
                        count,
                        workspaces,
                        versions: versions.iter().map(|v| v.to_string()).collect(),
                    });
                }

                let string = if args.yaml {
                    serde_yaml::to_string(&result)?
                } else if args.no_color {
                    serde_json::to_string_pretty(&result)?
                } else {
                    serde_json::to_string_pretty(&result)?.to_colored_json_auto()?
                };

                println!("{}", string);
                return Ok(());
            }

            // pretty print as table
            let mut table = Table::new();

            if args.deep {
                table.add_row(row!["Dependency", "Count", "Versions", "Workspaces"]);
            } else {
                table.add_row(row!["Dependency", "Count", "Packages"]);
            }

            for (name, packages) in dependency_map.iter() {
                let count = packages.len();

                if count < args.min {
                    continue;
                }

                let mut workspaces = packages
                    .clone()
                    .workspaces
                    .into_iter()
                    .collect::<Vec<String>>();

                workspaces.sort();

                let versions_count = packages
                    .clone()
                    .versions
                    .into_iter()
                    .collect::<Vec<String>>()
                    .len();

                if args.deep {
                    table.add_row(row![name, count, versions_count, workspaces.join(", ")]);
                } else {
                    table.add_row(row![name, count, workspaces.join(", ")]);
                }
            }

            table.printstd();

            // -1 because we have to remove header row
            let total = table.row_iter().len() - 1;

            if total == 0 {
                println!("No duplicate dependencies found (min: {})", args.min);
                return Ok(());
            }

            println!("Total : {}", total);
        }
    }

    Ok(())
}
