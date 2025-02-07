use pulldown_cmark::{Parser as MdParser, Options, html, Event, Tag,CowStr , CodeBlockKind};
use std::fs;
use std::path::Path;

fn markdown_to_html(markdown: &str, title: String) -> String {
    let options = Options::ENABLE_TABLES; // Enable table support

    let parser = MdParser::new_ext(markdown, options);

    let mut html_output = String::new();
    let mut events = Vec::new();
    let mut in_script_block = false;

    // Process each Markdown event
    for event in parser {
        match event {
            // Detect links and replace .md with .html
            Event::Start(Tag::Link(link_type, dest, title)) => {
                let new_dest = if dest.ends_with(".md") {
                    dest.replace(".md", ".html")
                } else {
                    dest.to_string()
                };
                events.push(Event::Start(Tag::Link(link_type, new_dest.into(), title)));
            }
           
            // Detect script code blocks
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if lang == CowStr::Borrowed("script") => {
                in_script_block = true;
                events.push(Event::Html("<div>".into()));
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if lang == CowStr::Borrowed("script") => {
                in_script_block = false;
                events.push(Event::Html("</div>".into()));
            }
            Event::Text(text) if in_script_block => {
                events.push(Event::Html(text.into()));
            }

            // Pass through other events unchanged
            _ => events.push(event),
        }
    }

    // Render the modified events to HTML
    html::push_html(&mut html_output, events.into_iter());

    html_template(html_output, title)
}


// Wrap the HTML content in a basic HTML template with KaTeX support
fn html_template(html_output: String, title: String) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    {}
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js"
        onload="renderMathInElement(document.body);"></script>
</body>
</html>"#,
        title, html_output
    )

}

pub fn process_markdown_file(input_path: &Path, output_dir: &Path, title: String) {
    let markdown_content = fs::read_to_string(input_path).expect("Failed to read Markdown file");

    // Convert Markdown to HTML
    let html_content = markdown_to_html(&markdown_content, title);

    // Create output file path
    let output_filename = input_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned() + ".html";
    let output_path = output_dir.join(output_filename);

    // Write HTML to file
    fs::write(output_path, html_content).expect("Failed to write HTML file");
}