use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
pub fn find<P : AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
}
pub fn walk_tree(
    dir : &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //如果不是，应该怎么办呢？
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
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