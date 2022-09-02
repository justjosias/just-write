use jw_internals::{notebooks, Error, Notebook};

use std::{
    env, fs, path,
    process::{self, ExitCode},
};

use rand::Rng;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: jw [subcommand]");
        return ExitCode::FAILURE;
    }

    let notebook = if args.len() > 1 {
        Notebook::load(&args[1])
    } else {
        None
    };

    match args[0].as_str() {
        "version" => {
            println!("jw {}", env!("CARGO_PKG_VERSION"));
        }
        "help" => {
            println!(
                r#"Usage: jw [subcommand]
A micro-journaling tool

  help                     display this help text
  version                  display version information

  new     NOTEBOOK         make a new notebook
  post    NOTEBOOK         write a post
  search  NOTEBOOK  QUERY  list posts containing query
  tags    NOTEBOOK         list hashtags in a notebook
  edit    NOTEBOOK  [NUM]  edit last NUM post
  list                     list existing notebooks"#
            );
        }

        // Notebook-related subcommands
        "new" => {
            if args.len() < 2 {
                eprintln!("Usage: jw new NOTEBOOK");
                return ExitCode::FAILURE;
            }

            let id = &args[1];
            match Notebook::generate(id) {
                Ok(_) => {}
                Err(e) => {
                    if let Error::Exists = e {
                        eprintln!("jw: notebook {id} already exists");
                        return ExitCode::FAILURE;
                    }
                    eprintln!("Error: {}", e);
                    return ExitCode::FAILURE;
                }
            };
        }

        "post" => {
            if args.len() < 2 {
                eprintln!("Usage: jw post NOTEBOOK");
                return ExitCode::FAILURE;
            }

            if let Some(notebook) = notebook {
                match get_text() {
                    Ok(text) => match notebook.post(&text) {
                        Ok(path) => {
                            println!("Wrote post to {:?}", path);
                        }
                        Err(e) => {
                            eprintln!("Error writing post: {}", e);
                            return ExitCode::FAILURE;
                        }
                    },
                    Err(p) => {
                        eprintln!("Failed to read temporary file: {}", p.display());
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                open_error(&args[1]);
                return ExitCode::FAILURE;
            }
        }

        "list" => match notebooks::list_notebooks() {
            Ok(notebooks) => {
                for notebook in notebooks {
                    println!("{}", notebook.id);
                }
            }
            Err(e) => {
                eprintln!("Failed to open notebooks parent folder: {:?}", e);
                return ExitCode::FAILURE;
            }
        },

        "search" => {
            if args.len() < 3 {
                eprintln!("Usage: jw search NOTEBOOK QUERY");
                return ExitCode::FAILURE;
            }

            if let Some(notebook) = notebook {
                if args.len() < 3 {
                    eprintln!("Usage: jw search {} QUERY", &args[1]);
                }

                let results = notebook.search(&args[2]);
                for path in results {
                    println!("{}", path.display());
                }
            } else {
                open_error(&args[1]);
                return ExitCode::FAILURE;
            }
        }

        "tags" => {
            if args.len() < 2 {
                eprintln!("Usage: jw tags NOTEBOOK");
                return ExitCode::FAILURE;
            }

            if let Some(notebook) = notebook {
                let tags = notebook.tags();
                let mut tags: Vec<(&String, &usize)> = tags.iter().collect();
                tags.sort();

                for (tag, count) in tags {
                    println!("#{tag}: {count}");
                }
            } else {
                open_error(&args[1]);
                return ExitCode::FAILURE;
            }
        }

        "edit" => {
            if args.len() < 2 {
                eprintln!("Usage: jw edit NOTEBOOK [NUM]");
                return ExitCode::FAILURE;
            }

            if let Some(notebook) = notebook {
                let posts = notebook.get_posts().unwrap();

                let num = if args.len() < 3 {
                    0
                } else if let Ok(num) = args[2].parse::<usize>() {
                    num
                } else {
                    eprintln!("Error: index must be a number");
                    return ExitCode::FAILURE;
                };

                if let Some(path) = posts.iter().rev().nth(num) {
                    match open_editor(notebook.path.join(path)) {
                        Ok(_) => {}
                        Err(p) => {
                            eprintln!("Failed to open file: {}", p.display());
                        }
                    }
                } else {
                    eprintln!("Error: number too large. No such post.");
                }
            } else {
                open_error(&args[1]);
                return ExitCode::FAILURE;
            }
        }

        arg => {
            eprintln!("Error: unknown subcommand: {}", arg);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

fn open_error(id: &str) {
    eprintln!(
        "Error when opening notebook: {}\nUse jw new NOTEBOOK to create a new one",
        id
    );
}

fn get_text() -> Result<String, path::PathBuf> {
    let mut path = env::temp_dir();
    path.push("jw-".to_owned() + &random_string(10));
    open_editor(path)
}

fn open_editor(path: path::PathBuf) -> Result<String, path::PathBuf> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_owned());

    if process::Command::new(editor).arg(&path).status().is_err() {
        eprintln!("Failed to find editor. Set EDITOR or install vi to resolve.");
        return Err(path);
    }

    if let Ok(s) = fs::read_to_string(&path) {
        let _ = fs::remove_file(&path);
        Ok(s)
    } else {
        Err(path)
    }
}

fn random_string(len: usize) -> String {
    // from https://github.com/HKarimiA/rust-generate-random-string
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let i = rng.gen_range(0..CHARS.len());
            CHARS[i] as char
        })
        .collect()
}
