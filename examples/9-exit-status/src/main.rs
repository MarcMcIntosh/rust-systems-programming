use serde::Deserialize;
use serde_json;
use std::{collections::HashSet, os::unix::fs::MetadataExt, path::PathBuf};
use glob::glob;
use std::os::unix::fs::PermissionsExt;

// load the regex-fules.json file to provide configs
const JSON: &str = include_str!("../rules.json");


#[derive(Deserialize, Debug, Clone)]
struct ComplianceRule {
    path_regex: String,
    file_permissions: u32,
    required_files: Vec<String>,
}

impl ComplianceRule {
    fn new(path_regex: String, file_permissions: u32, required_files: Vec<String>) -> Self {
        Self {
            path_regex,
            file_permissions,
            required_files,
        }
    }
}

// Load the rules from a configuration file (JSON)
fn load_rules() -> Result<Vec<ComplianceRule>, serde_json::Error> {
    // error case if json fails to parse
    let loaded_json: Vec<ComplianceRule> = serde_json::from_str(JSON)?;

    let mut rules: Vec<ComplianceRule> = Vec::new();
    for rule in loaded_json {
        rules.push(ComplianceRule::new(
            rule.path_regex,
            rule.file_permissions,
            rule.required_files,
        ));
    }
    Ok(rules)
}

fn run_rule_on_paths(rule: &ComplianceRule, paths: glob::Paths) -> Result<HashSet<PathBuf>, i32> {
    let init = (HashSet::<PathBuf>::new(), 0);
    let (results, code) = paths.fold(init, |(seen, exit_code), curr| {
        if exit_code > 0 {
            return (seen, exit_code);
        }
        match curr {
            Err(e) => {
                println!("{:?}", e);
                (seen, exit_code)
            },
            Ok(path_buff) => {
                if path_buff.is_dir() || seen.contains(&path_buff) {
                    return (seen, exit_code);
                }
                let tmp = HashSet::<PathBuf>::from([path_buff.clone()]);
                let next_seen = seen.union(&tmp).cloned().collect();
                let metadata = path_buff.metadata();
                let next_code = match metadata {
                    Err(_) => 1,
                    Ok(data) => if data.mode() != rule.file_permissions {3} else {exit_code}
                };

                (next_seen, next_code)

            }
        } 
    });
    if code > 0 {
        Err(code)
    } else {
        Ok(results)
    }
}

fn run_rules(rules: Vec<ComplianceRule>) -> Result<(), i32> {
    if let Some((rule, rest)) = rules.split_first() {
        let entry = glob(&rule.path_regex).map_err(|_| 1)?;
        let checked_files_path_buff = run_rule_on_paths(rule, entry)?;
        let checked_files_str = checked_files_path_buff.iter().filter_map(|pb| pb.as_path().to_str()).collect::<HashSet<&str>>();
        let all_found = rule.required_files.iter().all(|req| checked_files_str.contains(req.as_str()));
        if !all_found {
            return Err(3);
        }
        return run_rules(rest.to_vec());
    } 
    
    return Ok(());
}

fn main() {
    let mut exit_code: i32 = 0;
    let rules = load_rules().map_or_else(|_| {
        exit_code = 1;
        return Vec::new();
    }, |res| res);

    let run_rules_result = run_rules(rules);
    println!("{:?}", run_rules_result.clone());

    match run_rules_result {
        Err(code) => {
            exit_code = code;
        },
        Ok(results) => {
            println!("{:?}", results)
        }
    };

    std::process::exit(exit_code);

}
