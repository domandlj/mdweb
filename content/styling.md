# Styling

[*⇐ Go back 🦀*](main.md)

`styles.css`
```css
h1, h2, h3 {
  color: #ffffff;
}

body {
  font-family: Arial, sans-serif;
  line-height: 1.6;
  margin: 0;
  padding: 20px;
  background-color: #121212;
  color: #e0e0e0;
}


a {
  color: #64b5f6;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

code {
  padding: 2px 5px;
  border-radius: 3px;
  background-color: #444;
  font-family: 'Courier New', monospace;
  color: #fefefe;
  transition: background-color 0.3s ease;
}

code:hover {
  background-color: #7f7f7f;
}
pre {
  background-color: #212121;
  padding: 10px;
  border-radius: 5px;
  overflow-x: auto;
  color: #e0e0e0;
}

blockquote {
  border-left: 4px solid #555;
  padding-left: 15px;
  color: #b0b0b0;
  margin-left: 0;
}
```