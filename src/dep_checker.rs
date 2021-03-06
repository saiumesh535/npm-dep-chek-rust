use crate::cli;
use crate::errors::Result;
use crate::fs_helper::{iterate_dir_files, read_dev_packages};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

type PackageCounterType = HashMap<String, i32>;

lazy_static! {
    static ref IGNORE_CONDITIONS_REGEX: Vec<Regex> = vec![
        Regex::new(r"(^@types/.*$)").unwrap(),
        Regex::new(r"(^@babel/.*$)").unwrap(),
        Regex::new(r"(^webpack.*$)").unwrap(),
        Regex::new(r"(^typescript$)").unwrap(),
        Regex::new(r"(^jest$)").unwrap(),
        Regex::new(r"(^ts-jest$)").unwrap(),
        Regex::new(r"(^tsc-watch$)").unwrap(),
        Regex::new(r"(^tslint$)").unwrap(),
    ];

    // regex vector
    static ref REGEX_VECTOR: Vec<&'static str> = vec![
        r#"(import|require).*("|')%name%.*('|")"#
    ];

}

// ignore few npm packages by default
fn should_ignore_by_text(package: &String) -> bool {
    let ignore_conditions: Vec<Regex> = IGNORE_CONDITIONS_REGEX.to_vec();
    for ignore_condition in ignore_conditions {
        let is_matched = ignore_condition.is_match(package);
        if is_matched {
            return true;
        }
    }
    return false;
}

pub fn check_deps() -> Result<()> {
    for key in vec!["dependencies", "devDependencies"] {
        let inputs = cli::parse_cli();
        let dir_path = PathBuf::new().join(inputs.source);
        let pkg_path = PathBuf::new().join(inputs.package);
        let packages = read_dev_packages(&pkg_path, key)?;

        let mut package_counter: PackageCounterType = HashMap::new();
        let mut regex_map: HashMap<String, Vec<Regex>> = HashMap::new();

        let cloned_regex_vector = REGEX_VECTOR.to_vec();

        // iterate through HashMap and get package names
        for (package, _) in &packages {
            if !should_ignore_by_text(&package) {
                package_counter.insert(package.to_string(), 0);
                let mut insert_regex_vector = vec![];
                for regex in &cloned_regex_vector {
                    let regex_by_replace = regex.replace("%name%", package.as_str());
                    insert_regex_vector.push(Regex::new(regex_by_replace.as_str()).unwrap());
                }
                regex_map.insert(package.to_string(), insert_regex_vector);
            }
        }

        iterate_dir_files(
            &PathBuf::new().join(dir_path),
            &mut package_counter,
            regex_map,
        )?;

        let mut un_used_packages: Vec<String> = vec![];
        for (k, v) in package_counter {
            if v == 0 {
                un_used_packages.push(k);
            }
        }

        if un_used_packages.len() == 0 {
            println!("No unused {} packages", key);
        } else {
            un_used_packages.sort();
            println!("unused {} packages", key);
            for un_used_package in un_used_packages {
                println!("* {:?}", un_used_package);
            }
        }
    }
    Ok(())
}
