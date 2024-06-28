use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use toml::Value;
use glob::glob;

static SOURCE_EXTENSIONS: &[&str] = &["rs", "js", "json", "toml"];

// collect all source files from an initial crate path
// will follow local dependencies and add those too
pub fn get_source_files(crate_path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let crate_path = crate_path.as_ref();
    let mut all_files = HashSet::new();

    find_source_files(crate_path, &mut all_files)?;
    let cargo_toml_path = crate_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        all_files.extend(find_local_deps(&cargo_toml_path, None)?);
    }

    let paths = all_files.into_iter().collect();

    Ok(paths)

}

// A helper to just get all the subdirectories of a directory that contain a Cargo.toml file
// without knowing the specific contract name etc.
#[allow(dead_code)]
pub fn get_crate_subdirectories<P: AsRef<Path>>(start_dir: P) -> Vec<PathBuf> {
    let mut dirs_with_cargo_toml = Vec::new();

    // Read the entries in the start directory
    if let Ok(entries) = fs::read_dir(&start_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let cargo_toml_path = path.join("Cargo.toml");
                    if cargo_toml_path.exists() {
                        dirs_with_cargo_toml.push(path);
                    }
                }
            }
        }
    }

    dirs_with_cargo_toml
}

// recursively find local dependencies in a Cargo.toml file
// and add all .rs files in those dependencies to the list of files
// visited is used to avoid infinite loops in case of circular dependencies
fn find_local_deps(cargo_toml_path: &Path, visited: Option<&mut HashSet<PathBuf>>) -> Result<HashSet<PathBuf>> {

    // visited is an optional parameter, if it's None we create a new &mut HashSet
    // and we need the lifetime of that new HashSet to last for the whole function
    let mut local_visited;
    let visited = if let Some(v) = visited {
        v
    } else {
        local_visited = HashSet::new();
        &mut local_visited
    };

    let mut files = HashSet::new();

    if let Ok(cargo_toml) = fs::read_to_string(cargo_toml_path) {
        let cargo_toml: Value = toml::from_str(&cargo_toml).context("Failed to parse Cargo.toml")?;

        // Collect dependencies and dev-dependencies
        let deps = cargo_toml.get("dependencies").and_then(|d| d.as_table());
        let dev_deps = cargo_toml.get("dev-dependencies").and_then(|d| d.as_table());

        for dep_table in [deps, dev_deps].iter().flatten() {
            for dep_info in dep_table.values() {
                if let Some(dep_path) = dep_info.get("path").and_then(|p| p.as_str()) {
                    let dep_path_buf = PathBuf::from(dep_path);
                    // only process the dependency if we haven't visited it yet
                    // this is to avoid infinite loops in case of circular dependencies
                    if visited.insert(dep_path_buf.clone()) {
                        find_source_files(&dep_path_buf, &mut files)?;
                        let cargo_toml_path = dep_path_buf.join("Cargo.toml");
                        if cargo_toml_path.exists() {
                            // recursively find local dependencies
                            // and add them to the list of files
                            files.extend(find_local_deps(&cargo_toml_path, Some(visited))?);
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}

// find all source files in a directory and its subdirectories
fn find_source_files<P: AsRef<Path>>(path: P, files: &mut HashSet<PathBuf>) -> Result<()> {
    for ext in SOURCE_EXTENSIONS {
        let pattern = format!("{}/**/*.{}", path.as_ref().display(), ext);
        for path in glob(&pattern)? {
            files.insert(path?);
        }
    }

    Ok(())
}