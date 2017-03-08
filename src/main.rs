extern crate yaml_rust;
use yaml_rust::{yaml, Yaml};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Bookmark<'a> {
    local_bookmark_path: &'a Path,
    shared_bookmark_path: &'a Path,
    local_project_path: &'a Path,
}

fn main() {
    match load_from_file() {
        Ok(yaml_vec) => {
            let yaml = &yaml_vec[0];
            // println!("{}", yaml["local_bookmark_repository"].as_str().unwrap());
            // println!("{:?}", yaml["shared_bookmark_repository"].as_str().unwrap());
            // println!("{:?}", yaml["projects"]);
            match yaml["projects"].as_vec() {
                Some(projects) => {
                    for project in projects {
                        let bookmark = Bookmark {
                            local_bookmark_path: Path::new(yaml["local_bookmark_repository"]
                                .as_str()
                                .unwrap_or("/")),
                            shared_bookmark_path: Path::new(yaml["shared_bookmark_repository"]
                                .as_str()
                                .unwrap_or("/")),
                            local_project_path: Path::new(project["directory"]
                                .as_str()
                                .unwrap_or("/")),
                        };
                        println!("{:?}", bookmark);
                    }
                }
                None => {
                    println!("there is no project");
                }
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn load_from_file() -> Result<Vec<Yaml>, String> {
    env::home_dir()
        .ok_or("no home directory".to_owned())
        .and_then(|mut home| {
            home.push(".unite_bookmark_sync.yml");
            Ok(home)
        })
        .and_then(|path| File::open(path).map_err(|err| err.to_string()))
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut yaml_string = String::new();
            file.read_to_string(&mut yaml_string)
                .map_err(|err| err.to_string())
                .map(|_| yaml_string)
        })
        .and_then(|mut yaml_string| {
            yaml::YamlLoader::load_from_str(&mut yaml_string).map_err(|err| err.to_string())
        })
}
