use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub occurances: usize,
}

pub type Tags = HashMap<String, usize>;

/// Searches each file for a query and returns the paths that contain it.
/// Skips IO errors.
pub fn search_files(paths: &[PathBuf], query: &str) -> Vec<PathBuf> {
    let mut new_paths = Vec::new();
    for path in paths {
        if let Ok(contents) = read_to_string(&path) {
            if contents.contains(query) {
                new_paths.push(path.to_owned());
            }
        }
    }
    new_paths
}

/// Extracts the tags from all files and counts them.
/// Skips IO errors.
pub fn tags(paths: &[PathBuf]) -> Tags {
    let mut tags = HashMap::new();
    for path in paths {
        if let Ok(f) = File::open(path) {
            for line in BufReader::new(f).lines() {
                if let Ok(line) = line {
                    let mut line = line;
                    line.push('\n');
                    let mut in_tag = false;
                    let mut tag = String::new();
                    for c in line.chars() {
                        if in_tag {
                            if c == ' ' || c.is_ascii_punctuation() || c == '\n' {
                                in_tag = false;
                                tags.entry(tag.clone()).and_modify(|t| *t += 1).or_insert(1);
                                tag.clear();
                            } else {
                                tag.push(c);
                            }
                        } else {
                            if c == '#' {
                                in_tag = true;
                            }
                        }
                    }
                }
            }
        }
    }
    tags
}
