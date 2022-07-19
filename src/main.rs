mod file_io;
mod engine;
mod error;
mod log;
mod yaml_hash;

use yaml_rust::{Yaml, YamlEmitter};
use engine::*;
use file_io::*;
use log::Log;

fn main() {

    render_directory(
        "./templates/articles", "tera",
        EngineType::Tera,
        "./mdxts/articles", "md",
        &None,
        &None,
        true
    ).unwrap();

    render_articles();

    render_directory(
        "./templates/pages", "md",
        EngineType::MDxt,
        "./templates/pages", "html",
        &None,
        &None,
        true
    ).unwrap();

    render_templates(
        "./templates/pages/article.tera",
        "./htmls/articles", "html",
        "./output/htmls/articles", "html",
        None,
        Some(article_context()),
        true
    ).unwrap();

    render_templates(
        "./templates/pages/article.tera",
        "./htmls/documents", "html",
        "./output/htmls/documents", "html",
        None,
        Some(document_context()),
        true
    ).unwrap();

    render_styles();
}

use mdxt::COLORS;

fn get_colors() -> tera::Context {

    let mut context = tera::Context::new();

    let color_names = COLORS.iter().map(|color| color.name.clone()).collect::<Vec<String>>();
    context.insert("colors", &color_names);

    let hex = COLORS.iter().map(|color| color.to_hex()).collect::<Vec<String>>();
    context.insert("hex", &hex);

    let compl_hex = COLORS.iter().map(|color| color.complement().to_hex()).collect::<Vec<String>>();
    context.insert("compl_hex", &compl_hex);

    context
}

fn render_articles() {
    let mdxts_logs = render_directory(
        "./mdxts", "md",
        EngineType::MDxt,
        "./htmls", "html",
        &None,
        &None,
        true
    ).unwrap();

    let mut logs_hash = yaml_hash::new();

    for Log { file_from, file_to, metadata } in mdxts_logs.into_iter() {
        logs_hash = yaml_hash::insert(logs_hash, Yaml::from_str(&file_from), metadata);
    }

    let mut yaml_hash_string = String::new();
    let mut emitter = YamlEmitter::new(&mut yaml_hash_string);
    emitter.dump(&logs_hash).unwrap();

    write_to_file("./output/articles.txt", yaml_hash_string.as_bytes());
}

fn render_styles() {

    let color_context = get_colors();

    render_directory(
        "./templates/scss", "tera",
        EngineType::Tera,
        "./output/styles", "scss",
        &Some(color_context),
        &None,
        false
    ).unwrap();

    copy_all(
        "./templates/scss", "scss",
        "./output/styles", "scss",
        true
    ).unwrap();

    render_directory(
        "./output/styles", "scss",
        EngineType::Scss,
        "./output/styles", "css",
        &None,
        &None,
        false
    ).unwrap();

    for sub_dir in get_sub_directories_recursive("./output") {
        copy_all(
            "./output/styles", "css",
            &sub_dir, "css",
            true
        ).unwrap();
    }
}

fn article_context() -> tera::Context {

    let mut context = tera::Context::new();
    let nav = read_string("./templates/pages/nav.html").unwrap();
    let header = read_string("./templates/pages/header.html").unwrap();
    let footer = read_string("./templates/pages/footer.html").unwrap();

    context.insert("nav", &nav);
    context.insert("header", &header);
    context.insert("footer", &footer);
    context.insert("csses", &vec!["markdown.css", "blog_page.css", "nav.css", "header.css"]);

    context
}

fn document_context() -> tera::Context {

    let mut context = tera::Context::new();

    context.insert("csses", &vec!["markdown.css", "doc_page.css"]);

    context
}