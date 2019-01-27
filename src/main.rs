#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

fn count_digits(n: usize) -> usize {
    let mut ret = 0;
    let mut n = n;
    while n > 0 {
        ret += 1;
        n /= 10;
    }
    ret
}

fn main() {
    let matches = clap();
    let do_it = matches.is_present("do-it");
    let verbose = matches.is_present("verbose");

    let start = match matches.value_of("start") {
        Some(s) => s.parse().unwrap(), /* if this fails, we shouldnt continue anyway */
        None => 0,
    };

    let fnames: Vec<String> = matches
        .values_of("FILES")
        .unwrap() /* safe */
        .into_iter()
        .map(From::from)
        .collect();

    let mut files: Vec<PathBuf> = fnames
        .iter()
        .map(|a| Path::new(a).to_path_buf())
        .filter(|p| p.exists())
        .map(|p| fs::canonicalize(p).unwrap())
        .collect();

    files.sort_by(|p1, p2| {
        let s1 = p1.as_os_str();
        let s2 = p2.as_os_str();

        match s1.len().cmp(&s2.len()) {
            Ordering::Equal => s1.cmp(s2),
            c => c,
        }
    });
    let ndigs = count_digits(files.len());

    let mut files: Vec<(PathBuf, PathBuf)> = files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            /* 0000i.ext */
            let new_f = f.with_file_name(format!("{:0width$}", i + start, width = ndigs));

            let new_f = if let Some(ext) = f.extension() {
                new_f.with_extension(ext)
            } else {
                new_f
            };

            /* (from, to) */
            (f.clone(), new_f)
        })
        .collect();

    files.reverse();

    for (f, t) in files {
        if !do_it || verbose {
            println!("{} -> {}", f.display(), t.display());
        }

        if do_it {
            /* in case of error, report */
            if let Err(e) = fs::rename(&f, &t) {
                if !verbose {
                    println!("{} -> {}: {:?}", f.display(), t.display(), e);
                } else {
                    println!("{:?}", e);
                }
            }
        }
    }
}

fn clap() -> ArgMatches<'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about("Rename files to the shortest name that maintains order")
        .arg(
            Arg::with_name("do-it")
                .help("Make the changes")
                .long("do-it")
                .required(false)
                .multiple(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Show changes as they happen")
                .long("verbose")
                .short("v")
                .required(false)
                .multiple(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("start")
                .help("Numeration starts at this argument")
                .long("start")
                .short("s")
                .required(false)
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILES")
                .help("Files to rename")
                .multiple(true)
                .required(true),
        )
        .help_short("H")
        .get_matches()
}
