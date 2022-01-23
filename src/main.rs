use clap::Parser;
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::{DirEntry, WalkDir};

mod args;
use args::Args;

// if the item is a directory or a file that starts with a `.` then we will skip it, this function
// just checks the names and creates a map with only the non-hidden files
fn is_not_hidden(entry: &DirEntry, hidden: bool) -> bool {
    if entry.path().is_dir() {
        return true;
    }
    if hidden {
        return false;
    }

    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    // use clap to parse arguments
    let mut args = Args::parse();

    // if there is no specified directory flag then default to the current one
    if args.directory.is_none() {
        args.directory = Some(".".to_string());
    }

    traverse(
        args.directory.unwrap(),
        args.prefix_ignore.unwrap_or_default(),
        args.suffix_ignore.unwrap_or_default(),
        args.extensions.unwrap_or_default(),
        args.colons.unwrap_or_default(),
        args.hidden.unwrap_or_default(),
    );
}

// this function does all the work, it takes in the path and some arguments then spits out to the
// terminal
fn traverse(
    path: String,
    prefix_ignore: String,
    suffix_ignore: String,
    extensions: Vec<String>,
    semicolons: bool,
    hidden: bool,
) {
    // initialize counts for files, lines, and semicolons
    let (mut files, mut sum, mut sc) = (0, 0, 0);

    // if it is just a single file, then we can operate on it
    if !PathBuf::from(path.to_owned()).is_dir() {
        files = 1;
        sum = count_lines(path.to_owned());
        if semicolons {
            // if we are unable to read the file into a string then just skip it with a closure
            // statement
            (|| {
                let content = match read_to_string(path) {
                    Ok(v) => v,
                    Err(_) => return,
                };
                // count the semicolins and add it to the total
                sc += content.rmatches(";").count();
            })();
        }
    } else {
        let dir = WalkDir::new(path.to_owned())
            .into_iter()
            .filter_entry(|e| is_not_hidden(e, hidden))
            .filter_map(|v| v.ok());

        for i in dir {
            let path = i.path().display().to_string();
            // if the path is too short to parse on then skip it
            if path.len() < 2 {
                continue;
            }

            let exten = match i.path().extension() {
                Some(v) => v,
                None => continue,
            };

            // if there is a matching file prefix, a matching suffix, or if there are extensions to
            // look for we skip when needed
            if path[..prefix_ignore.len()] == prefix_ignore && prefix_ignore.len() > 0
                || path[..path.len() - suffix_ignore.len()] == suffix_ignore
                    && suffix_ignore.len() > 0
                || (extensions.len() > 0
                    && !extensions.contains(&exten.to_string_lossy().into_owned()))
            {
                continue;
            }
            files += 1;

            // if we need to look for semicolons, then we can read to string and count
            if semicolons {
                let content = match read_to_string(i.path()) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                sc += content.rmatches(";").count();
            }
            sum += count_lines(path);
        }
    }
    // fancy color output must use octals in rust, they are standard bash color codes
    let mut output =
        format!("\x1b[32mFiles: \x1b[31m{files}\n\x1b[32mLines: \x1b[31m{sum}\x1b[37m");
    if semicolons {
        output.push_str(&format!("\n\x1b[32mSemicolons: \x1b[31m{sc}\x1b[37m"));
    }
    println!("{output}");
}

// take in a string and count how many lines there are with BufReader
fn count_lines(path: String) -> usize {
    let file = match File::open(path.to_owned()) {
        Ok(v) => v,
        Err(_) => {
            println!("Failed to open file {}!", path);
            return 0;
        }
    };
    let reader = BufReader::new(file);
    reader.lines().count()
}
