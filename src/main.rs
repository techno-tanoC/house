use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, PartialEq, Debug, StructOpt)]
#[structopt(name = "house", version = "0.1")]
struct Opt {
    #[structopt(short, long)]
    silent: bool,
    #[structopt(short, long)]
    dry_run: bool,
    path: PathBuf,
}

fn find_dir<'a>(file: &PathBuf, dirs: &'a Vec<PathBuf>) -> Result<Option<&'a PathBuf>> {
    let file_name = file
        .file_name()
        .ok_or_else(|| anyhow!("failed to get filename: from {:?}", file))?
        .to_string_lossy();

    for dir in dirs {
        if let Some(dir_name) = dir.file_name() {
            if file_name.contains(&*dir_name.to_string_lossy()) {
                return Ok(Some(dir))
            }
        }
    }

    Ok(None)
}

fn rename(from: &PathBuf, dest: &PathBuf, opt: &Opt) -> Result<()> {
    let from_name = from
        .file_name()
        .ok_or_else(|| anyhow!("failed to get file_name: from: {:?}", from))?;

    let dest_name = dest
        .file_name()
        .ok_or_else(|| anyhow!("failed to get file_name: dest: {:?}", dest))?;

    if !opt.dry_run {
        let to = dest.join(from_name);
        fs::rename(&from, &to)?;
    }

    if !opt.silent {
        println!("from: {:?}", &from_name);
        println!("to  : {:?}", &dest_name);
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut files = vec![];
    let mut dirs = vec![];

    if let Ok(entries) = fs::read_dir(&opt.path) {
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
        if let Ok(Some(dest)) = find_dir(&from, &dirs) {
            if let Err(err) = rename(&from, &dest, &opt) {
                println!("{}", err);
            }
        }
    }

    Ok(())
}
