use crate::errors::Result;
use crate::errors::*;
use regex::Regex;
use serde_json::from_str;
use serde_json::{from_value, Value};
use snafu::ResultExt;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref FILE_EXTENSIONS: Vec<&'static str> = vec!["js", "ts", "tsx", ".jsx"];
}

type DepType = HashMap<String, String>;
type JsonMap = Result<DepType>;

fn read_file_from_path(path: &PathBuf) -> Result<String> {
    Ok(read_to_string(path).context(ReadConfiguration { path })?)
}

pub fn read_dev_packages(pkg_path: &PathBuf) -> JsonMap {
    let package = read_file_from_path(&pkg_path)?;
    let mut content: HashMap<String, Value> =
        from_str(package.as_str()).context(Deserialization)?;
    let dependencies: DepType =
        from_value(content.get_mut("dependencies").unwrap().take()).context(Deserialization)?;
    Ok(dependencies)
}

fn should_consider(path: &PathBuf) -> bool {
    if path.extension().is_none() {
        return false;
    };
    if path.to_str().unwrap().contains("node_modules") {
        return false;
    };
    let file_extensions: Vec<&str> = FILE_EXTENSIONS.to_vec();
    let extension = path.extension().unwrap().to_str().unwrap();
    if file_extensions.contains(&extension) {
        return true;
    };
    return false;
}

fn get_all_paths(dir_path: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<()> {
    let dir = read_dir(dir_path).context(ReadConfiguration { path: dir_path })?;
    for curr_path in dir {
        let dir_entry = curr_path.context(ReadConfiguration { path: dir_path })?;
        if dir_entry.path().is_dir() {
            get_all_paths(&dir_entry.path(), paths)?;
        } else {
            if should_consider(&dir_entry.path()) {
                paths.push(dir_entry.path());
            }
        }
    }
    Ok(())
}

fn check_if_package_used_regex(content: &String, regexes: &Vec<Regex>) -> bool {
    for regex in regexes {
        if regex.is_match(content) {
            return true;
        }
    }
    return false;
}

// iterate through given dir path all folders and files
pub fn iterate_dir_files(
    dir_path: &PathBuf,
    deps_count: &mut HashMap<String, i32>,
    regex_map: HashMap<String, Vec<Regex>>,
) -> Result<()> {
    let mut paths: Vec<PathBuf> = vec![];
    get_all_paths(dir_path, &mut paths)?;
    let mut packages: Vec<String> = vec![];
    for (package, _) in deps_count.iter() {
        packages.push(package.to_string());
    }
    // read content from each path and check condition
    for path in paths {
        let content = read_file_from_path(&path)?;
        for package in &packages {
            if check_if_package_used_regex(&content, regex_map.get(package).unwrap()) {
                deps_count
                    .entry(package.to_string())
                    .and_modify(|e| *e += 1);
            }
        }
    }
    Ok(())
}
