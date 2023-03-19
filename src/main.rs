use git2::{Error, Repository, StatusOptions};

use std::{io, process::Command};

fn collect_files() -> Result<Vec<String>, Error> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open; {}", e),
    };

    let mut opts = StatusOptions::new();

    let statuses = repo.statuses(Some(&mut opts))?;

    let mut file_names: Vec<String> = Vec::new();
    let stdin = io::stdin();

    for entry in statuses.iter() {
        let path = match entry.path() {
            Some(path) => path,
            None => panic!("NO PATH"),
        };

        let file_name = path;

        let mut confirm = String::new();

        if file_name.ends_with(".cpp") || file_name.ends_with(".h") {
            println!(
                "{} has been modified. Do you want to format it?([Y,y]/N)",
                file_name
            );

            stdin
                .read_line(&mut confirm)
                .unwrap_or_else(|_| panic!("Could not read from stdin! Aborting..."));

            let mut ch = confirm.chars().next().unwrap();
            ch = ch.to_ascii_uppercase();

            if ch != 'Y' {
                continue;
            }

            file_names.push(path.to_string());
        }
    }

    Ok(file_names)
}

fn run() -> Result<(), Error> {
    let file_names = collect_files().unwrap();
    if file_names.len() < 1 {
        println!("No files to format!");
        return Ok(());
    }

    let mut clang_format = Command::new("clang-format");
    clang_format.arg("-i");

    for file_name in file_names.iter() {
        clang_format.arg(file_name);
    }

    clang_format.status().unwrap_or_else(|e| {
        panic!("Clang_Format failed: {e}");
    });

    Ok(())
}

fn main() {
    println!("Hello, world!");

    let _ = run();
}
