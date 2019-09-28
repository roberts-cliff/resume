extern crate argparse;
extern crate resume;
#[macro_use]
extern crate serde_derive;

use std::io::BufWriter;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate serde_yaml;
use serde_yaml::{Error, Result};
use argparse::{ArgumentParser, StoreTrue, Store};
use resume::data_structure::*;

fn main() {
    println!("------------------------------fff");
    match io_shit() {
        Ok(person) => println!("{:?}", person),
        Err(e) => println!("....{:?}", e)
    }
}

//#[test]

fn io_shit() -> Result<Person>{
    let f = File::open("resume.yml").unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> = serde_yaml::from_reader(reader);
    println!("{:?}", p);
    p
}
