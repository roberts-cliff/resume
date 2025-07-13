extern crate argparse;
extern crate handlebars;
extern crate resume;
extern crate serde_json;
extern crate serde;
extern crate serde_yaml;
extern crate schemars;
extern crate headless_chrome;
extern crate urlencoding;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use schemars::schema_for;
use handlebars::Handlebars;
use serde_yaml::Result;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use urlencoding::encode;

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
    println!("{:?}", schema);
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
    let pdf_path = format!("{}/{}.pdf", output_dir, output_prefix);
    
    fs::write(&md_path, md_output).unwrap();
    fs::write(&html_path, &html_output).unwrap();
    generate_pdf_from_html(&html_output, &pdf_path);
    
    println!("Generated resume files:");
    println!("  - {}", md_path);
    println!("  - {}", html_path);
    println!("  - {}", pdf_path);
}

fn read_person(input_file: &str) -> Result<Person> {
    let f = File::open(input_file).unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> = serde_yaml::from_reader(reader);
    p
}

fn generate_pdf_from_html(html_content: &str, pdf_path: &str) {
    let launch_options = LaunchOptionsBuilder::default()
        .headless(true)
        .sandbox(false)
        .build()
        .expect("Failed to create launch options");
    
    let browser = Browser::new(launch_options)
        .expect("Failed to launch browser");
    
    let tab = browser.new_tab()
        .expect("Failed to create new tab");
    
    // Create a data URL from the HTML content
    let data_url = format!("data:text/html;charset=utf-8,{}", encode(html_content));
    
    // Navigate to the data URL
    tab.navigate_to(&data_url)
        .expect("Failed to navigate to content");
    
    // Wait for the page to load
    tab.wait_until_navigated()
        .expect("Failed to wait for navigation");
    
    // Wait a bit more for CSS to render
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    // Print to PDF
    let pdf_data = tab.print_to_pdf(None)
        .expect("Failed to print to PDF");
    
    fs::write(pdf_path, pdf_data).expect("Failed to write PDF file");
}
