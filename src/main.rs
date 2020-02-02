use std::io;
use std::fs;
use std::path::Path;
use clap::{Arg, App, ArgMatches};

fn get_args() -> ArgMatches<'static> {
    App::new("House")
        .version("0.1")
        .arg(Arg::with_name("silent")
            .short("s")
            .long("silent")
            .takes_value(false))
        .arg(Arg::with_name("dry-run")
            .short("d")
            .long("dry-run")
            .takes_value(false))
        .arg(Arg::with_name("path")
            .required(true))
        .get_matches()
}

fn find_dir<'a, P: AsRef<Path>>(file: &P, dirs: &'a Vec<P>) -> Option<&'a P> {
    let file_name = file.as_ref().file_name().unwrap().to_string_lossy();
    dirs.iter().find(|&p| {
        let dir_name = p.as_ref().file_name().unwrap().to_string_lossy();
        file_name.contains(&*dir_name)
    })
}

fn rename<P: AsRef<Path>>(from: &P, dest: &P, matches: &ArgMatches) {
    let (from, dest) = (from.as_ref(), dest.as_ref());
    let to = dest.join(from.file_name().unwrap());

    if !matches.is_present("dry-run") {
        fs::rename(&from, &to).unwrap();
    }
    if !matches.is_present("silent") {
        println!("from: {:?}", &from.file_name().unwrap());
        println!("to  : {:?}", &dest.file_name().unwrap());
        println!();
    }
}

fn main() -> io::Result<()> {
    let matches = get_args();
    let path = matches.value_of("path").expect("path is not provided");

    let mut files = vec![];
    let mut dirs = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry = entry?;
            let typ = entry.file_type()?;

            if typ.is_file() {
                files.push(entry.path());
            } else if typ.is_dir() {
                dirs.push(entry.path());
            }
        }
    }

    for from in files {
        if let Some(dest) = find_dir(&from, &dirs) {
            rename(&from, &dest, &matches);
        }
    }

    Ok(())
}

