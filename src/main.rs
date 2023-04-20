#[macro_use] extern crate rocket;
#[macro_use] extern crate fern;
extern crate yrs;
extern crate walkdir;
extern crate humantime;


use std::path;
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use std::fs;
use log::{debug, info, warn, error};
use yrs::{Doc, GetString, ReadTxn, StateVector, Text, Transact, Update};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use walkdir::WalkDir;
use std::time::SystemTime;

fn setup_logger() -> Result<(), fern::InitError> {
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
        .level(log::LevelFilter::Debug)
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



fn main() -> Result<(), Box<dyn std::error::Error>>{
    setup_logger()?;

    let args: Vec<String> = env::args().collect();
    let path_arg = args.get(1);
    let path_var = option_env!("CREW_DIR");
    let path = if let Some(arg) = path_arg {
        Path::new(arg)
    } else if let Some(var) = path_var {
        Path::new(var)
    } else {
        Path::new(".")
    };

    let doc_arg = args.get(2);
    let doc_var = option_env!("CREW_DOC");
    let name = if let Some(arg) = doc_arg {
        arg
    } else if let Some(var) = doc_var {
        var
    } else {
        "crewDoc" //TODO: make this some UUID thing
    };


 /*   let dir = WalkDir::new(path).follow_links(true);
    let doc = Doc::new();
    let dir_map = doc.get_or_insert_map(name);
    for entry in dir {
        match entry {
            Ok(entry) => ???,
            Error(err) => ???,
        }
    }
*/
    //let dir = doc.get_or_insert_map(name);
    println!("The dir is {}", path.to_str().unwrap_or("<cannot display path>"));
    Ok(())
}
