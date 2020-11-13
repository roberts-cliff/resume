extern crate argparse;
extern crate handlebars;
extern crate resume;
extern crate serde_json;
extern crate serde_derive;
extern crate serde_yaml;
extern crate schemars;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use schemars::schema_for;
use handlebars::Handlebars;
use serde_yaml::Result;

use resume::data_structure::*;

fn main() {
    let schema = schema_for!(Person).unwrap();
    println!("{:?}", schema);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("person_schema.json", json.as_str()).unwrap();

    let person = read_person().unwrap();
    let template_md = fs::read_to_string("template.md".to_string()).unwrap_or_default();
    let template_html = fs::read_to_string("template.html".to_string()).unwrap_or_default();
    let handlebars = Handlebars::new();
    let md_output = handlebars.render_template(template_md.as_str(), &person).unwrap();
    let html_output = handlebars.render_template(template_html.as_str(), &person).unwrap();
    fs::write("resume.md", md_output).unwrap();
    fs::write("resume.html", html_output).unwrap();
}

fn read_person() -> Result<Person> {
    let f = File::open("resume.yml").unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> =
        serde_yaml::from_reader(reader);
    p
}
