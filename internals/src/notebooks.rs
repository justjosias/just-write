use std::{
    fs,
    io::{BufRead, Write},
    path::PathBuf,
};

use serde_derive::{Deserialize, Serialize};

use crate::{config::get_config, search, time::Timestamp, Error};

#[derive(Default, Deserialize, Serialize)]
struct Config {
    name: String,
    description: String,
    post_path: String,
    metadata: bool,
}

#[derive(Default)]
pub struct Notebook {
    pub id: String,
    pub path: PathBuf,
}

impl Notebook {
    /// Loads a notebook from any path. Returns `None` if nonexistent.
    fn load_from_path(path: &std::path::Path) -> Option<Self> {
        if path.exists() && path.join("notebook.toml").exists() {
            Some(Self {
                // We already checked for the filename, and assume the path is valid UTF-8
                id: path.file_name().unwrap().to_string_lossy().into_owned(),
                path: path.to_owned(),
            })
        } else {
            None
        }
    }

    /// Loads the notebook from the root directory
    pub fn load(id: &str) -> Option<Self> {
        if let Ok(config) = get_config() {
            Self::load_from_path(&config.root.join(id))
        } else {
            None
        }
    }

    /// Generates a new notebook. Fails if notebook alrady exists.
    pub fn generate(id: &str) -> Result<Notebook, Error> {
        let root = get_config()?.root;

        let path = root.join(id);
        if path.exists() {
            return Err(Error::Exists);
        } else {
            fs::create_dir_all(&path)?;
        }

        let config = Config {
            name: id.to_string(),
            description: String::new(),
            post_path: "%Y-%m-%d-%s.md".to_string(),
            metadata: true,
        };

        let mut f = fs::File::create(path.join("notebook.toml"))?;
        f.write_all(toml::to_string(&config)?.as_bytes())?;

        // Errors are returned earlier, so unwrapping is okay
        Ok(Notebook::load(id).unwrap())
    }

    fn read_config(&self) -> Result<Config, Error> {
        let config_str = fs::read_to_string(self.path.join("notebook.toml"))?;
        Ok(toml::from_str(&config_str)?)
    }
    fn _write_config(&self, conf: Config) -> Result<(), Error> {
        fs::write(
            self.path.join("notebook.toml"),
            toml::to_string(&conf)?.as_bytes(),
        )?;
        Ok(())
    }

    pub fn post(&self, text: String) -> Result<PathBuf, Error> {
        let mut first_text = String::new();
        for (i, ch) in text.chars().enumerate() {
            first_text.push(ch);
            if i > 20 {
                break;
            }
        }

        let first_text = {
            let mut ft = String::new();
            let mut lc = ' ';
            for (i, c) in first_text.chars().enumerate() {
                if c.is_alphanumeric() {
                    ft.push(c);
                    lc = c;
                } else if lc != '-' && i + 1 < first_text.len() {
                    ft.push('-');
                    lc = '-';
                }
            }
            if lc == '-' {
                ft.pop();
            }
            ft
        };

        let dt = Timestamp::now();
        let year = format!("{:04}", dt.year);
        let month = format!("{:02}", dt.month);
        let day = format!("{:02}", dt.day);

        let config = self.read_config()?;

        let path = PathBuf::from(
            config
                .post_path
                .replace("%Y", &year)
                .replace("%m", &month)
                .replace("%d", &day)
                .replace("%s", &first_text),
        );
        let metadata = format!("---\ntimestamp: {}\n---\n", dt.timestamp);

        let mut full_path = self.path.join(&path);
        if full_path.exists() {
            // TODO find a reasonable way to alter a path
            full_path = full_path
                .components()
                .map(|c| {
                    if c.as_os_str() == path.as_os_str() {
                        let mut path = path.as_os_str().to_owned();
                        path.push(".1");
                        path
                    } else {
                        c.as_os_str().to_owned()
                    }
                })
                .collect();
        }

        let mut f1 = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.path.join("posts.txt"))?;

        // unwrap is okay because full_path was constructed with a safe filename.
        // Written filename could be inaccurate if is not valid Unicode.
        f1.write_all(full_path.file_name().unwrap().to_string_lossy().as_bytes())?;
        f1.write_all(b"\n")?;

        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&full_path)?;

        if config.metadata {
            f.write_all(metadata.as_bytes())?;
        }
        f.write_all(text.as_bytes())?;

        Ok(full_path)
    }

    pub fn get_posts(&self) -> Result<Vec<PathBuf>, Error> {
        let f = fs::File::open(self.path.join("posts.txt"))?;
        let reader = std::io::BufReader::new(f);
        let mut posts = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                posts.push(PathBuf::from(line));
            }
        }

        Ok(posts)
    }

    fn absolute_paths(&self) -> Vec<PathBuf> {
        self.get_posts()
            .unwrap()
            .iter()
            .map(|p| self.path.join(p))
            .collect::<Vec<PathBuf>>()
    }

    pub fn search(&self, query: &str) -> Vec<PathBuf> {
        search::search_files(&self.absolute_paths(), query)
    }

    pub fn tags(&self) -> search::Tags {
        search::tags(&self.absolute_paths())
    }
}

pub fn list_notebooks() -> Result<Vec<Notebook>, Error> {
    let root = get_config()?.root;
    let dir = fs::read_dir(root)?;

    let notebooks = dir
        .filter_map(|entry| match entry {
            Ok(e) => Notebook::load_from_path(&e.path()),
            Err(_) => None,
        })
        .collect();

    Ok(notebooks)
}
