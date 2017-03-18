extern crate yaml_rust;
extern crate clap;
use yaml_rust::{yaml, Yaml};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

use clap::{App, SubCommand};

#[derive(Debug)]
struct Bookmark<'a> {
    local_bookmark_path: &'a Path,
    shared_bookmark_path: &'a Path,
    projects: Vec<Project<'a>>,
}

#[derive(Debug)]
struct Project<'a> {
    name: &'a str,
    directory: &'a Path,
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Yaml(yaml_rust::ScanError),
    None(String),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<yaml_rust::ScanError> for CliError {
    fn from(err: yaml_rust::ScanError) -> CliError {
        CliError::Yaml(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> CliError {
        CliError::None(String::from(err))
    }
}

fn main() {
    let matches = App::new("Syncronize unite_bookmarks")
        .version("0.1")
        .subcommand(SubCommand::with_name("push").about("push bookmarks to shared repository"))
        .subcommand(SubCommand::with_name("pull").about("pull bookmarks from shared repository"))
        .get_matches();

    match load_from_file() {
        Ok(yaml_vec) => {
            let yaml = &yaml_vec[0];
            let bookmark = build_bookmark(yaml);
            match bookmark {
                Ok(b) => {
                    match matches.subcommand_name() {
                        Some("push") => {
                            b.push();

                        }
                        Some("pull") => {
                            b.fetch();
                        }
                        _ => {
                            println!("no such command");
                            return;
                        }
                    }
                }
                Err(err) => println!("err:{:?}", err),
            }
        }
        Err(err) => println!("err:{:?}", err),
    }
}

impl<'a> Bookmark<'a> {
    fn fetch(&self) {
        for project in &self.projects {
            let shared_file_name = format!("{}/{}",
                                           self.shared_bookmark_path.to_str().unwrap(),
                                           project.name);
            let local_file_name = format!("{}/{}",
                                          self.local_bookmark_path.to_str().unwrap(),
                                          project.name);
            let mut local_file = File::create(local_file_name).unwrap();
            match File::open(shared_file_name) {
                Ok(f) => {
                    let file = BufReader::new(&f);
                    let mut lines = file.lines();
                    lines.next();
                    match local_file.write_fmt(format_args!("0.1.0\n")) {
                        Ok(_) => {}
                        Err(err) => println!("err:{:?}", err),
                    }
                    for line in lines {
                        match line {
                            Ok(l) => {
                                let bookmark_info: Vec<&str> = l.split('\t').collect();
                                let bookmarked_file = format!("{}{}",
                                                              project.directory.to_str().unwrap(),
                                                              bookmark_info[1].to_string());
                                match local_file.write_fmt(format_args!("{}\t{}\t{}\t{}\n",
                                                                        bookmark_info[0],
                                                                        bookmarked_file,
                                                                        bookmark_info[2],
                                                                        bookmark_info[3])) {
                                    Ok(_) => {}
                                    Err(err) => println!("err:{:?}", err),
                                }
                            }
                            Err(err) => println!("err:{:?}", err),
                        }
                    }
                }
                Err(err) => println!("err:{:?}", err),
            }
        }
    }

    fn push(&self) {
        for project in &self.projects {
            let shared_file_name = format!("{}/{}",
                                           self.shared_bookmark_path.to_str().unwrap(),
                                           project.name);
            let local_file_name = format!("{}/{}",
                                          self.local_bookmark_path.to_str().unwrap(),
                                          project.name);
            let mut shared_file = File::create(shared_file_name).unwrap();
            match File::open(local_file_name) {
                Ok(f) => {
                    let file = BufReader::new(&f);
                    let mut lines = file.lines();
                    lines.next();
                    match shared_file.write_fmt(format_args!("0.1.0\n")) {
                        Ok(_) => {}
                        Err(err) => println!("err:{:?}", err),
                    }
                    for line in lines {
                        match line {
                            Ok(l) => {
                                let bookmark_info: Vec<&str> = l.split('\t').collect();
                                let bookmarked_file = bookmark_info[1]
                                    .to_string()
                                    .replacen(project.directory.to_str().unwrap(), "", 1);
                                match shared_file.write_fmt(format_args!("{}\t{}\t{}\t{}\n",
                                                                         bookmark_info[0],
                                                                         bookmarked_file,
                                                                         bookmark_info[2],
                                                                         bookmark_info[3])) {
                                    Ok(_) => {}
                                    Err(err) => println!("err:{:?}", err),
                                }
                            }
                            Err(err) => println!("err:{:?}", err),
                        }
                    }
                }
                Err(err) => println!("err:{:?}", err),
            }
        }
    }
}

fn load_from_file() -> Result<Vec<Yaml>, CliError> {
    let bookmark_sync_path = env::home_dir().ok_or("no home directory".to_owned())
        .and_then(|mut home| {
            home.push(".unite_bookmark_sync.yml");
            Ok(home)
        })?;

    let mut bookmark_sync_string = File::open(bookmark_sync_path).and_then(|mut file| {
            let mut yaml_string = String::new();
            file.read_to_string(&mut yaml_string)
                .map(|_| yaml_string)
        })?;
    let yaml = yaml::YamlLoader::load_from_str(&mut bookmark_sync_string)?;
    Ok(yaml)
}

fn build_bookmark<'a>(yaml: &'a Yaml) -> Result<Bookmark, CliError> {
    let projects = yaml["projects"].as_vec().ok_or("no project".to_owned())?;
    let mut vec = Vec::new();
    for pro in projects.into_iter() {
        if let Some(name) = pro["name"].as_str() {
            if let Some(directory_string) = pro["directory"].as_str() {
                let directory = Path::new(directory_string);
                vec.push(Project {
                    name: name,
                    directory: directory,
                });
            }
        }
    }
    Ok(Bookmark {
        local_bookmark_path: Path::new(yaml["local_bookmark_repository"]
            .as_str()
            .unwrap_or("/")),
        shared_bookmark_path: Path::new(yaml["shared_bookmark_repository"]
            .as_str()
            .unwrap_or("/")),
        projects: vec,
    })
}
