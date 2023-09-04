use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
pub fn find<P : AsRef<Path>>(root: P, regex: &Regex, flag: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches, flag)?;
    Ok(matches)
}
pub fn walk_tree(
    dir : &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
    flag : usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches, flag)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if flag == 1 {
                    println!("遍历：{}", path.to_string_lossy().to_string());
                }
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    else {
        println!("目录错误");
        process::exit(1);
    }
    Ok(())
}