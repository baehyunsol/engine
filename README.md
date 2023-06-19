## Available fields of metadata

Use yaml

```rust
// only for articles
// if true, it's not on the articles list
hide: bool;

// only for articles
// [2023, 6, 19]
date: [i32; 3];

// only for articles
tags: Vec<String>;

// only for articles
preview: String;

// sets <title> tag
title: String;

// available: cursor.js
extra_scripts: Vec<String>;

// available: cursor.css
extra_styles: Vec<String>;
```

## Config files

Read example config files

## Write documents

write md files in `./mdxts/articles` and `./mdxts/documents`

the engine distinguishes files by their name, not path