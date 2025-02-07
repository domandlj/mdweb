# MDWEB

MDWEB is a Rust command line tool to convet markdown files `.md` into an a static `.html` websites.

# Usage

Place your `.md` files into a folder and execute `mdewb` with

```bash
Usage: mdweb --title <TITLE> --input <INPUT_FOLDER> --output <OUTPUT_FOLDER>

Options:
  -t, --title <TITLE>           Title of website
  -i, --input <INPUT_FOLDER>    Path to the input folder containing Markdown files
  -o, --output <OUTPUT_FOLDER>  Path to the output folder where HTML files will be saved
  -h, --help                    Print help
```

# Building
Assuming you have `rust` and `cargo`.
```bash
$ cargo build --release    
```

# Installation
```bash
$ cargo install --path .
```

# Hyperlinks.
Suppose you have `A.md` and `B.md`, you can make a hyperlink from A to B with `[link to B](B.md)` inside A. This will reference B's HTML when compiled.

# Latex
It has support for writing latex equations.

# Styling
Save a `styles.css` file into your markdown website folder.

# Embeddings
You can add custom HTML tags inside your `.md` files wrapping them with 

\`\`\`script  
(your embbeded code) 
\`\`\`
