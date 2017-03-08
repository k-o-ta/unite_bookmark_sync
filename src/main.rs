extern crate yaml_rust;
use yaml_rust::{yaml, Yaml};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Bookmark<'a> {
    local_bookmark: &'a Path,
    shared_bookmark: &'a Path,
}

fn main() {
    match load_from_file() {
        Ok(yaml_vec) => {
            for yaml in yaml_vec {
                println!("{:?}", yaml["local_bookmark_repository"]);
                println!("{:?}", yaml["shared_bookmark_repository"]);
                for project in yaml["projects"].as_vec() {
                    // println!("{:?}", project);
                    // println!("{:?}", project["directory"]);
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
