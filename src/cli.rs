
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;
use crate::md_utils::process_markdown_file;

#[derive(Parser, Debug)]
#[command(name = "md_to_html_converter")]
#[command(about = "Converts a folder of Markdown files to HTML")]
struct Args {
    /// Path to the input folder containing Markdown files
    #[arg(short, long, value_name = "INPUT_FOLDER")]
    input: String,

    /// Path to the output folder where HTML files will be saved
    #[arg(short, long, value_name = "OUTPUT_FOLDER")]
    output: String,
}



pub fn cli() {
    let args = Args::parse();

    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output).expect("Failed to create output directory");

    // Find all Markdown files in the input folder
    let md_files: Vec<PathBuf> = glob(&format!("{}/*.md", args.input))
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .collect();

    // Process each Markdown file
    for md_file in &md_files {
        process_markdown_file(md_file, Path::new(&args.output));
    }

    println!(
        "Conversion complete! HTML files saved to: {}", args.output
    );
}