extern crate argparse;
extern crate handlebars;
extern crate resume;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

use argparse::{ArgumentParser, Store, StoreTrue};
use handlebars::Handlebars;
use serde_yaml::{Error, Result};

use resume::data_structure::*;

fn main() {
    let person = read_person().unwrap();
    let template = read_template();
//    println!("{:?}", template);
    let mut handlebars = Handlebars::new();
    let contents = handlebars.render_template(template.as_str(), &person).unwrap();
//    println!("{:?}", contents.unwrap());
    fs::write("resume.html", contents);
}

fn read_template() -> String {
    fs::read_to_string("template.html").unwrap_or_default()
}

//#[test]

fn read_person() -> Result<Person> {
    let f = File::open("resume.yml").unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> =
        serde_yaml::from_reader(reader);
//    println!("{:?}", p);
    p
}
