#[macro_use] extern crate rocket;
extern crate fern;
extern crate yrs;
extern crate walkdir;
extern crate humantime;
extern crate regex;


use std::path;
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use std::fs;
use log::{trace, debug, info, warn, error};
use yrs::{Doc, GetString, ReadTxn, StateVector, Text, Transact, Update};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use walkdir::{DirEntry, WalkDir};
use std::time::SystemTime;
use clap::Parser;
use clap_verbosity_flag::Verbosity;


#[derive(Parser, Debug)]
#[command(name = "crew")]
#[command(author = "James Clemer")]
#[command(version = "0.0.1")]
#[command(about = "A tool for sharing state via CRDTs in the YJS style")]
struct Arguments {
    #[arg(short, long, default_value="crewdoc")]
    name: String,

    #[arg(short, long, default_value=".")]
    path: String,

    #[arg(short, long)]
    exclude: Vec<String>,

    #[command(flatten)]
    verbose: Verbosity,
}

fn setup_logger(args: &Arguments) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(args.verbose.log_level_filter())
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
/**
1. Represent the dir as a mapref
2. Expose files as subdocuments therein
3. Serve
*/

fn is_included(exclusions: &Vec<String>, entry: &DirEntry) -> bool {
    let path = entry.path();
    let path = path.canonicalize().unwrap();

    trace!("Path is: {}", path.display());
    for ancestor in path.ancestors() {
        trace!("\tAncestor path is: {}", ancestor.display());
        for exclude in exclusions {
            let exclude = Path::new(&exclude);
            let exclude = exclude.canonicalize().unwrap();

            trace!("\t\tExclude path is: {}", exclude.display());
            if ancestor == exclude {
                info!("Excluding '{}' because '{}' is excluded", path.display(), exclude.display());
                return false;
            }
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Arguments::parse();
    setup_logger(&args)?;

    let doc_root = Path::new(&args.path).canonicalize()?;

    let dir = WalkDir::new(&args.path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| is_included(&args.exclude, &e))
        .filter_map(|e| e.ok());

    let mut doc = Doc::new();
    for entry in dir {
        let absolute_path = entry.path().canonicalize()?;
        let doc_path = absolute_path.strip_prefix(&doc_root)?;

        // add sub-directories as nested maps
        println!("{} -> {}", doc_path.display(), absolute_path.display());
        for c in doc_path.components() {
            println!("{:?}", c);
        }
    }

 /*   let doc = Doc::new();
    let dir_map = doc.get_or_insert_map(name);
    for entry in dir {
        match entry {
            Ok(entry) => ???,
            Error(err) => ???,
        }
    }
*/
    //let dir = doc.get_or_insert_map(name);
    Ok(())
}
