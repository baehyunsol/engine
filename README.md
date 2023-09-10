# Engine

This is a web frontend framework for [my personal blog](https://baehyunsol.github.io).

## Usage

This repository contains articles for my personal blog. I recommend you to empty the `./mdxts/articles` directory after you clone this.

In order to use this engine, you have to know how to use [MDxt], [tera] and [scss]. I assume you already know html, javascript and css.

[MDxt]: https://github.com/baehyunsol/MDxt
[tera]: https://github.com/Keats/tera
[scss]: https://sass-lang.com/guide/

The repository contains sample style sheets, scripts and template files. I recommend you to modify the sample files than writing everything from scratch.

This is how it works.

1. It reads config files in `./configs`. The files are `./configs/articles.yaml` and `./configs/documents.yaml`
  - If the files don't exist, it uses the default config.
2. It reads `.md` files in `./mdxts/articles` and `./mdxts/documents`. The MDxt files are converted to html files.
  - This is where you write your articles and documents. The engine only accepts MDxt files. Since MDxt is (almost) a superset of github-flavored markdown, any markdown file would work.
  - Articles vs Documents
    - The engine does extra stuffs to the articles. For example, you can set tags and dates of articles. It also renders index page and a few other pages for articles.
  - With `--doc` option, it only renders documents. It renders both documents and articles by default.
3. It renders style sheets in `./templates/scss`. There are 2 steps. It first renders `.tera` files to `.scss` files. Then it renders `.scss` files to `.css` files. The style sheet must be written in either tera or scss. You cannot use css files directly.
4. It renders scripts files in `./templates/js`. You can either use `.tera` format or `.js` format.
5. It renders tera files in `./templates/articles`. Unfortunately, not much thing is configurable for now. You can modify the paragraphs in `index.tera`, but I guess that's all for it.
6. It renders pages in `./templates/pages`. `article.tera` is the template for all the articles and documents. It's not configurable because most stuffs are hard-coded in the engine. You're free to modify `footer.md`, `header.md` and `nav.md`. There are style sheets and script files dedicated for the header and nav, so don't forget to take care of them.
7. It copies multimedia files in `./mdxts/articles` and `./mdxts/documents` to the result directory.
  - It copies `.jpg`, `.jpeg`, `.png`, `.svg`, `.gif`, `.m4a`, `.mp4`, `.mp3`, `.wav`, `.ogg` and `.webm`.

## Metadata

It uses yaml syntax.

```rust
// only for articles
// if true, it's not on the articles list
hide: bool;

// if true, it loads all the script files and style sheets into the html file.
// it cannot load img files tho
one_file: bool;

// only for articles
// example: [2023, 6, 19]
date: [i32; 3];

// only for articles
tags: Vec<String>;

// only for articles
preview: String;

// sets <title> tag
title: String;

// available: cursor.js
// You're free to add yours.
extra_scripts: Vec<String>;

// available: cursor.css
// You're free to add yours.
extra_styles: Vec<String>;
```

## Config files

Below is an example config file.

```yaml
{
    "has_header": true,
    "has_nav": true,
    "has_footer": true,

    # uses scss units
    "article_width_landscape": "82%",
    "article_top_margin_landscape": "$header-height + $padding-big",
    "article_bottom_margin_landscape": "0px",
    "article_left_margin_landscape": "9%",
    "article_right_margin_landscape": "9%",
    "article_width_portrait": "90%",
    "article_top_margin_portrait": "$header-height + $padding-big",
    "article_bottom_margin_portrait": "0px",
    "article_left_margin_portrait": "5%",
    "article_right_margin_portrait": "5%",

    # px
    "font_size_landscape": 21,
    "font_size_portrait": 16,

    # px
    "default_horiz_padding": 0,

    # it means the title of `index.html` is `Blog`
    "titles": {
        "index": "Blog"
    },

    # if it has `XXX`, the engine doesn't render `XXX.md`
    "ignores": []
}
```
