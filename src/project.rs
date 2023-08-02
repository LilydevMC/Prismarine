use std::{env, fs::{self, File}, path::PathBuf, process::exit, io::{Read, Write}};
use colored::Colorize;
use ignore::{DirEntry, WalkBuilder};
use zip::{self, write::FileOptions};

use crate::{
    models::{
        Config,
        General, PackMeta, Pack
    },
    utils::{pris_err, pris_export_dir, pris_export_file}
};


pub fn create_project(name: String, path: Option<PathBuf>) {
    let default_new_dir: PathBuf = match env::var("PRIS_TEST_FOLDER") {
        Ok(new_path) => {
            PathBuf::from(new_path)
        },
        Err(_) => PathBuf::new()
    };

    let project_path = match path.clone() {
        Some(path_buf) => {
            if path_buf == PathBuf::from(".") {
                env::current_dir().unwrap()
            } else {
                match path_buf.is_relative() {
                    true => env::current_dir().unwrap().join(default_new_dir).join(path_buf).join(&name),
                    false => path_buf.join(default_new_dir).join(&name)
                }
            }
        },
        None => {
            env::current_dir().unwrap().join(default_new_dir).join(&name)
        }
    };

    match project_path.exists() {
        true => {
            match project_path.read_dir() {
                Ok(mut dir_contents) => {
                    if path.is_some() {
                        match dir_contents.next() {
                            Some(_) => {
                                println!("{}{} Directory isn't empty!", "Error".red().bold(), ":".bold());
                                exit(1)
                            },
                            None => ()
                        }
                    }
                },
                Err(err) => {
                    println!("{}", err.to_string());
                    exit(1)
                }
            }
        },
        false => {
            match fs::create_dir_all(&project_path) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err.to_string());
                    exit(1)
                }
            }
        }
    }


    let config = Config {
        general: General {
            name: name.clone(),
            description: "This is an example pack description!".to_string(),
            pack_version: "0.1.0".to_string(),
            minecraft_version: "1.20.1".to_string(),
            license: "ARR".to_string(),
            name_template: None
        }
    };

    let config_string = toml::to_string_pretty(&config).expect("config thing");

    match fs::write(&project_path.join("prismarine.toml"), config_string) {
        Ok(_) => (),
        Err(err) => {
            pris_err!(format!("Couldn't write to file `prismarine.toml`: {}", err.to_string()));
            exit(1)
        }
    }

    match fs::write(&project_path.join(".prisignore"), ".git\n.gitignore\n/source\n*.env\n*.zip\n") {
        Ok(_) => (),
        Err(err) => {
            pris_err!(format!("Couldn't write to file `.prisignore`: {}", err.to_string()));
            exit(1)
        }
    }

    match fs::create_dir(&project_path.join("source")) {
        Ok(_) => (),
        Err(err) => {
            pris_err!(format!("Couldn't create directory: {}", err.to_string()));
            exit(1)
        }
    }

    match fs::create_dir_all(&project_path.join("pack").join("minecraft")) {
        Ok(_) => (),
        Err(err) => {
            pris_err!(format!("Couldn't create directory: {}", err.to_string()));
            exit(1)
        }
    }
    
    match fs::write(&project_path.join("README.md"), format!("# {}\n\nThis is a resource pack made with Prismarine!", &name)) {
        Ok(_) => (),
        Err(err) => {
            pris_err!(format!("Couldn't write to file `README.md`: {}", err.to_string()));
            exit(1)
        }
    }
    
    

    // let config: Config = toml::Table::new();

}

pub fn export_project() {
    let current_path = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            pris_err!("Couldn't get current directory");
            exit(1)
        }
    };

    if !current_path.join("prismarine.toml").exists() {
        pris_err!("`prismarine.toml` file not present in current directory");
        exit(1)
    }
    if !current_path.join(".prisignore").exists() {
        pris_err!("`.prisignore` file not present in current directory");
        exit(1)
    }

    let mut included_items: Vec<DirEntry> = Vec::new();

    let ignore_walk = WalkBuilder::new(".")
        .git_ignore(false)
        .add_custom_ignore_filename(".prisignore")
        .build();

    for result in ignore_walk {
        match result {
            Ok(entry) => {
                if entry.path().to_str().unwrap() != "." {
                    included_items.push(entry.clone());
                    println!("{}", entry.path().to_str().unwrap())
                }
            },
            Err(_) => {
                pris_err!("Unknown result error for path walk");
                exit(1)
            }
        }
    }

    let config_string = match fs::read_to_string("prismarine.toml") {
        Ok(string) => string,
        Err(_) => {
            pris_err!("Couldn't read `prismarine.toml` file");
            exit(1)
        }
    };

    let config: Config = match toml::from_str(config_string.as_str()) {
        Ok(conf) => conf,
        Err(_) => {
            pris_err!("Couldn't parse `prismarine.toml`");
            exit(1)
        }
    };

    let file_writer = match File::create(format!("{}.zip", &config.general.name)) {
        Ok(file) => file,
        Err(_) => {
            pris_err!(format!("Couldn't create file `{}`.zip", &config.general.name));
            exit(1)
        }
    };

    let mut file_buffer: Vec<u8> = Vec::new();
    let mut zip_writer = zip::ZipWriter::new(file_writer);
    let zip_options = FileOptions::default();

    for entry in included_items {
        let path = entry.path();
        let mut name = match path.strip_prefix(".") {
            Ok(res) => res,
            Err(_) => {
                pris_err!(format!("Couldn't strip prefix for item: {}", path.to_str().unwrap()));
                exit(1)
            }
        };

        if name.starts_with("pack") {
            name = match name.strip_prefix("pack") {
                Ok(res) => res,
                Err(_) => {
                    pris_err!(format!("Couldn't strip prefix `pack` from: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            }
        }

        if name.as_os_str().is_empty() {
            println!("{}", "Skipping empty directory".italic());
            continue
        }

        if path.is_dir() {
            pris_export_dir!(&name.to_str().unwrap());
            
            #[allow(deprecated)] // .add_directory_from_path() is deprecated, but it works well for now!
            match zip_writer.add_directory_from_path(&name, zip_options) {
                Ok(_) => (),
                Err(_) => {
                    pris_err!(format!("Couldn't add directory to zip: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            }
        } else if path.is_file() {
            pris_export_file!(&name.to_str().unwrap());

            #[allow(deprecated)] // .start_file_from_path is deprecated, but it works well for now!
            match zip_writer.start_file_from_path(name, zip_options) {
                Ok(_) => (),
                Err(_) => {
                    pris_err!(format!("Couldn't start file: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            };

            let mut file = match File::open(path) {
                Ok(file) => file,
                Err(_) => {
                    pris_err!(format!("Couldn't open file: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            };

            match file.read_to_end(&mut file_buffer) {
                Ok(_) => (),
                Err(_) => {
                    pris_err!(format!("Couldn't read file: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            };

            match zip_writer.write(&*file_buffer) {
                Ok(_) => (),
                Err(_) => {
                    pris_err!(format!("Couldn't write to zip from file: {}", &name.to_str().unwrap()));
                    exit(1)
                }
            };
            file_buffer.clear();
        }

    }


    match zip_writer.start_file("pack.mcmeta", zip_options) {
        Ok(_) => (),
        Err(_) => {
            pris_err!("Couldn't start new file in zip: pack.mcmeta");
            exit(1)
        }
    };

    let pack_meta = PackMeta {
        pack: Pack {
            pack_format: 15,
            description: config.general.description
        }
    };

    let pack_meta_string = match serde_json::to_string_pretty(&pack_meta) {
        Ok(res) => res,
        Err(_) => {
            pris_err!("Couldn't encode file data to string: pack.mcmeta");
            exit(1)
        }
    };

    match zip_writer.write(pack_meta_string.as_bytes()) {
        Ok(_) => (),
        Err(_) => {
            pris_err!("Couldn't write to file in zip: pack.mcmeta");
            exit(1)
        }
    };


}
