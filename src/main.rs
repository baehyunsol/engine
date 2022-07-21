mod file_io;
mod engine;
mod error;
mod log;
mod yaml_hash;
mod article;
mod graph;

use lazy_static::lazy_static;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use engine::*;
use file_io::*;
use log::Log;
use std::collections::HashMap;

static mut BAD_PROGRAMMING_HABIT: bool = true;

fn main() {

    let articles = article::from_yaml((YamlLoader::load_from_str(&read_string("./output/articles.txt").unwrap()).unwrap())[0].clone());
    let tags_graph = article::get_tags(&articles);

    render_directory(
        "./templates/articles", "tera",
        EngineType::Tera,
        "./mdxts/articles", "md",
        &Some(meta_article_context(&articles, &tags_graph)),
        &None,
        true
    ).unwrap();

    render_tag_pages(&articles, &tags_graph);
    render_articles_mdxt();
    render_articles_html();
    render_styles();

    unsafe {

        // runs the program twice
        // so that the metadata is read correctly
        if BAD_PROGRAMMING_HABIT {
            BAD_PROGRAMMING_HABIT = false;
            main();
        }

    }
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

fn render_articles_mdxt() {

    copy_all(
        "./mdxts", "jpg",
        "./output/htmls", "jpg",
        true
    ).unwrap();

    let mdxts_logs = render_directory(
        "./mdxts", "md",
        EngineType::MDxt,
        "./htmls", "html",
        &None,
        &None,
        true
    ).unwrap();

    copy_all(
        "./htmls/tag_pages", "html",
        "./htmls/articles", "html",
        true
    );

    let mut logs_hash = yaml_hash::new();

    for Log { file_from, file_to, metadata } in mdxts_logs.into_iter() {
        logs_hash = yaml_hash::insert(logs_hash, Yaml::from_str(&file_from), metadata);
    }

    let mut yaml_hash_string = String::new();
    let mut emitter = YamlEmitter::new(&mut yaml_hash_string);
    emitter.dump(&logs_hash).unwrap();

    write_to_file("./output/articles.txt", yaml_hash_string.as_bytes());
}

fn render_articles_html() {

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

fn render_tag_pages(articles: &HashMap<String, article::Article>, tags_graph: &graph::Graph) {

    let mut tera_instance = tera::Tera::default();
    tera_instance.add_template_file("./templates/pages/tag.tera", Some("tag_page")).unwrap();

    for tag in tags_graph.iter() {
        let mut context = tera::Context::new();

        context.insert("tag_name", &tag);
        context.insert("articles", &tags_graph.get_articles(tag.clone()));

        let mut related_tags = tags_graph.get_adjacent_vertexes(tag.clone());

        if related_tags.len() > 3 {
            related_tags = related_tags[0..3].to_vec();
        }

        if related_tags.len() > 0 {
            context.insert("related_tags", &related_tags.into_iter().map(|(s, _)| s).collect::<Vec<String>>());
        }

        let rendered = tera_instance.render("tag_page", &context).unwrap();
        let save_to = join("./mdxts/tag_pages", &format!("tag-{}.md", tag)).unwrap();
        mkdir("./mdxts/tag_pages");
        write_to_file(&save_to, rendered.as_bytes()).unwrap();
    }

}

fn meta_article_context(articles: &HashMap<String, article::Article>, tags_graph: &graph::Graph) -> tera::Context {

    let mut context = tera::Context::new();
    let recent_articles = article::get_recent_articles(&articles, 5);

    context.insert("recent_article_names", &recent_articles);

    let articles_by_month = article::get_articles_by_month(&articles);

    if articles_by_month.len() > 0 {
        let mut articles_by_month_page = vec![];
        let mut articles_by_month_page_tmp = vec![];
        let mut curr_month = [0, 0];

        for ([year, month], name) in articles_by_month.into_iter() {

            if [year, month] != curr_month {

                if articles_by_month_page_tmp.len() > 0 {
                    articles_by_month_page.push(articles_by_month_page_tmp.join("\n"));
                }

                articles_by_month_page_tmp = vec![format!("### {} {}", MONTHS[month], year), String::new()];
                curr_month = [year, month];
            }

            articles_by_month_page_tmp.push(format!("- [{}]({}.html)", name, name));
        }

        if articles_by_month_page_tmp.len() > 0 {
            articles_by_month_page.push(articles_by_month_page_tmp.join("\n"));
        }

        context.insert("articles_by_month", &articles_by_month_page.join("\n\n"));
    }

    let mut tags = Vec::with_capacity(tags_graph.len());
    let mut tag_nums = Vec::with_capacity(tags_graph.len());

    for (tag, count) in tags_graph.get_vertex_weights().into_iter() {
        tags.push(tag);
        tag_nums.push(count);
    }

    context.insert("tags", &tags);
    context.insert("tag_nums", &tag_nums);

    context
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

lazy_static! {
    pub static ref MONTHS: Vec<String> = {
        let result = vec![
            "NULL",
            "January",
            "Feburary",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December"
        ];

        result.into_iter().map(|m| m.to_string()).collect()
    };
}