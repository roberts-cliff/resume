# Resume Generator

A Rust-based resume generator that creates professional resumes from YAML input. This tool allows you to maintain your resume in a structured YAML format and generate Markdown, HTML, and PDF versions with optimal page layout control.

## Features

- YAML-based resume definition with schema validation
- Multiple output formats (Markdown, HTML, and PDF)
- Professional HTML template with modern, print-optimized styling
- Intelligent page break control for PDF generation
- Compact, two-page layout design
- Command-line interface with flexible options
- IDE support with autocomplete (IntelliJ, Eclipse)
- Headless Chrome-based PDF generation for consistent output

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

This generates:
- `resume.md` - Markdown version
- `resume.html` - HTML version
- `resume.pdf` - PDF version (with optimized page layout)

### Command Line Options

- `-i, --input`: Specify input YAML file (default: resume.yml)
- `-o, --output-dir`: Specify output directory (default: current directory)
- `-p, --prefix`: Specify output filename prefix (default: resume)

Example:
```bash
# Generate with custom options
cargo run -- -i my_resume.yml -o output -p my_resume
```

### PDF Generation

The PDF output is automatically generated and features:
- Optimized two-page layout
- Intelligent page break control
- Professional typography and spacing
- Print-ready formatting

## Project Structure

- `resume.yml`: Main resume content in YAML format
- `template.md`: Markdown template
- `template.html`: HTML template with modern, print-optimized styling
- `person_schema.json`: JSON Schema for YAML validation
- `src/`: Source code
  - `main.rs`: Main application entry point with CLI handling
  - `lib.rs`: Library code and data structures

## Customization

### Templates

You can modify the output format by editing:
- `template.md` for Markdown output
- `template.html` for HTML and PDF output (includes modern CSS styling with print optimization)

### YAML Schema

The `person_schema.json` defines the structure for your resume YAML. Key sections include:
- Personal information (name, contact, purpose)
- Skills (categorized technical skills)
- Work experience (companies, roles, achievements)
- Education

### Page Layout Control

The HTML template includes sophisticated CSS for PDF generation:
- Automatic page break control
- Skills and experience sections kept together
- Compact, professional formatting
- Optimized for two-page layout

## IDE Support

The project includes a JSON Schema for the YAML file (`person_schema.json`), enabling autocomplete in supported IDEs:

- **IntelliJ**: Enable YAML schema support and point to `person_schema.json`
- **Eclipse**: Install YAML editor plugin and configure schema
- **VS Code**: Install YAML extension and configure schema

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
