#[macro_use]
extern crate log;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use spinners::{Spinner, Spinners};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(name = "cargo-kill", author = "beni69",version = env!("CARGO_PKG_VERSION"))]
struct Args {
    dir: PathBuf,
}

fn main() {
    pretty_env_logger::init();

    let args = Args::parse();

    if !args.dir.is_dir() {
        eprintln!("ERROR: {} is not a directory", args.dir.display());
        std::process::exit(1);
    }

    let sp = Spinner::new(Spinners::Dots, "Reading directories".into());

    let mut target = Vec::new();
    {
        // read directories "recursively"
        // doesnt actually use recursion
        let mut dirs = vec![args.dir.clone()];
        while let Some(dir) = dirs.pop() {
            info!("{}", dir.display());
            if let Ok(d) = fs::read_dir(&dir) {
                for entry in d {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_dir() {
                        match entry.file_name().to_str().unwrap() {
                            ".git" => continue,
                            "target" => target.push((path.clone(), dir_size(&path))),
                            _ => dirs.push(path),
                        }
                    }
                }
            } else {
                warn!("failed to read dir: {}", &dir.display());
            }
        }
    }

    sp.stop_with_message(format!("found {} directories\n", target.len()));

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select directories to delete")
        .items(
            &target
                .iter()
                .map(|p| format!("{} ({})", p.0.display(), size_str(p.1)))
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();
    println!("selected: {:?}", selected);

    let sp = Spinner::new(Spinners::Dots, "Deleting directories".into());
    let mut freed = 0u64;
    for i in selected {
        let path = &target[i].0;
        info!("{}", &path.display());
        if let Err(e) = fs::remove_dir_all(&path) {
            warn!("failed to delete dir: {}", e);
        } else {
            freed += target[i].1;
        }
    }
    sp.stop_with_message(format!("Done! Freed {}\n", size_str(freed)));
}

fn dir_size(path: &PathBuf) -> u64 {
    let mut dirs = vec![path.clone()];
    let mut size = 0u64;

    while let Some(dir) = dirs.pop() {
        if let Ok(d) = fs::read_dir(&dir) {
            for entry in d {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    size += path.metadata().unwrap().len();
                }
            }
        } else {
            warn!("failed to read dir: {}", &dir.display());
        }
    }

    size
}

fn size_str(size: u64) -> String {
    let mut s = String::new();
    let mut size = size;
    let mut i = 0;
    while size > 0 {
        let unit = match i {
            0 => "B",
            1 => "KiB",
            2 => "MiB",
            3 => "GiB",
            4 => "TiB",
            5 => "PiB",
            6 => "EiB",
            _ => "ZiB",
        };
        let val = size % 1024;
        size /= 1024;
        s = format!("{} {}", val, unit).to_string();
        i += 1;
    }
    s
}
