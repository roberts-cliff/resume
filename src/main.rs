extern crate argparse;
extern crate handlebars;
extern crate resume;
extern crate serde_json;
extern crate serde;
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
    let mut input_file = String::from("resume.yml");
    let mut output_dir = String::from(".");
    let mut output_prefix = String::from("resume");

    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Generate resume from YAML file");
        ap.refer(&mut input_file)
            .add_option(&["-i", "--input"], argparse::Store,
                "Input YAML file path")
            .metavar("FILE");
        ap.refer(&mut output_dir)
            .add_option(&["-o", "--output-dir"], argparse::Store,
                "Output directory (default: current directory)")
            .metavar("DIR");
        ap.refer(&mut output_prefix)
            .add_option(&["-p", "--prefix"], argparse::Store,
                "Output filename prefix (default: resume)")
            .metavar("PREFIX");
        ap.parse_args_or_exit();
    }

    let schema = schema_for!(Person).unwrap();
    let json = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("person_schema.json", json.as_str()).unwrap();

    let person = read_person(&input_file).unwrap();
    let template_md = fs::read_to_string("template.md".to_string()).unwrap_or_default();
    let template_html = fs::read_to_string("template.html".to_string()).unwrap_or_default();
    let handlebars = Handlebars::new();
    let md_output = handlebars.render_template(template_md.as_str(), &person).unwrap();
    let html_output = handlebars.render_template(template_html.as_str(), &person).unwrap();

    let md_path = format!("{}/{}.md", output_dir, output_prefix);
    let html_path = format!("{}/{}.html", output_dir, output_prefix);
    
    fs::write(&md_path, md_output).unwrap();
    fs::write(&html_path, html_output).unwrap();
    println!("Generated resume files:");
    println!("  - {}", md_path);
    println!("  - {}", html_path);
}

fn read_person(input_file: &str) -> Result<Person> {
    let f = File::open(input_file).unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> = serde_yaml::from_reader(reader);
    p
}
