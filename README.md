# Resume Generator

A Rust-based resume generator that creates professional resumes from YAML input. This tool allows you to maintain your resume in a structured YAML format and generate both Markdown and HTML versions.

## Features

- YAML-based resume definition
- Schema validation with JSON Schema
- Multiple output formats (Markdown and HTML)
- Customizable templates
- Command-line interface for easy use
- IDE support with autocomplete (IntelliJ, Eclipse)

## Prerequisites

- Rust (latest stable version)
- Visual Studio Build Tools (Windows) or GCC/Clang (Linux/macOS)

### Installing Rust

- **Windows**: Download and install Visual Studio Build Tools, then run `rustup-init.exe`
- **macOS/Linux**: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/resume.git
cd resume
```

2. Build the project:
```bash
cargo build
```

## Usage

### Basic Usage

Generate resume from default `resume.yml`:
```bash
cargo run
```

### Command Line Options

- `-i, --input`: Specify input YAML file (default: resume.yml)
- `-o, --output-dir`: Specify output directory (default: current directory)
- `-p, --prefix`: Specify output filename prefix (default: resume)

Example:
```bash
cargo run -- -i my_resume.yml -o output -p my_resume
```

### IDE Support

The project includes a JSON Schema for the YAML file (`person_schema.json`), enabling autocomplete in supported IDEs:

- **IntelliJ**: Enable YAML schema support and point to `person_schema.json`
- **Eclipse**: Install YAML editor plugin and configure schema

## Project Structure

- `resume.yml`: Main resume content in YAML format
- `template.md`: Markdown template
- `template.html`: HTML template
- `person_schema.json`: JSON Schema for YAML validation
- `src/`: Source code
  - `main.rs`: Main application entry point
  - `lib.rs`: Library code and data structures

## Customization

### Templates

You can modify the output format by editing:
- `template.md` for Markdown output
- `template.html` for HTML output

### YAML Schema

The `person_schema.json` defines the structure for your resume YAML. Key sections include:
- Personal information
- Education
- Work experience
- Skills
- Projects and achievements

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
