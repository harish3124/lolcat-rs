#[macro_use]
extern crate clap;

use clap::ArgMatches;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;


pub fn get_args<'a>() -> ArgMatches<'a> {
    let matches = clap_app!(lolcat_rs =>
        (version: "1.0")
        (author: "Harish <harishg3124@gmail.com>")
        (about: "An high performance rewrite of lolcat")
        (@arg INPUT: "Sets the input file to use")
    )
    .get_matches();
    return matches;
}

pub fn read_stdin(){
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.as_ref().unwrap());
    };
}

pub fn read_file(file_name: &str) {
    let file_path = Path::new(file_name);
    let file = File::open(file_path).expect("Unable to open file !");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.as_ref().unwrap());
    }
}