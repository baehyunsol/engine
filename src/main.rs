mod file_io;
mod engine;
mod error;
mod log;
mod yaml_hash;

use engine::*;

fn main() {

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
    );

    render_directory(
        "./output/styles", "scss",
        EngineType::Scss,
        "./output/styles", "css",
        &None,
        &None,
        false
    ).unwrap();

    render_directory(
        "./mdxts", "md",
        EngineType::MDxt,
        "./htmls", "html",
        &None,
        &None,
        true
    ).unwrap();

    render_templates(
        "./templates/pages/article.tera",
        "./htmls", "html",
        "./output/htmls", "html",
        None,
        Some(article_context()),
        true
    ).unwrap();

    copy_all(
        "./output/styles", "css",
        "./output/htmls", "css",
        true
    ).unwrap();
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

fn article_context() -> tera::Context {

    let mut context = tera::Context::new();

    context.insert("csses", &vec!["markdown.css", "page.css"]);

    context
}