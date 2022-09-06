mod article;
mod config;
mod engine;
mod error;
mod file_io;
mod graph;
mod log;
mod xml;
mod yaml_hash;

use config::*;
use engine::*;
use lazy_static::lazy_static;
use file_io::*;
use log::Log;
use std::collections::HashMap;
use yaml_rust::{
    Yaml,
    YamlEmitter,
    YamlLoader,
};

fn main() {
    let mut only_docs = false;
    let first_time = true;

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {

        if args[1] == "--doc".to_string() || args[1] == "-d".to_string() {
            only_docs = true;
        }

        else if args[1] == "--all".to_string() || args[1] == "-a".to_string() {
            only_docs = false;
        }

        else if args[1] == "--clear".to_string() || args[1] == "-c".to_string() {
            clean_up_results();
            remove_results();
            return;
        }

        else if args[1] == "--help".to_string() || args[1] == "-h".to_string(){
            println!("Engine v 0.1.0 (c) Baehyunsol

--all -a : render docs and articles
--clear -c : clear results
--doc -d : render only docs
--help -h : help message");
            return;
        }

    }

    render(only_docs, first_time);
}

// affected by configs
fn render(only_docs: bool, first_time: bool) {

    remove_results();

    let doc_configs = load_documents_config();
    let article_configs = load_articles_config();

    let articles = match read_string("./output/articles.yaml") {
        Ok(data) => article::from_yaml(YamlLoader::load_from_str(&data).unwrap()[0].clone()),
        Err(_) => article::from_yaml(YamlLoader::load_from_str("{}").unwrap()[0].clone())
    };

    let tags_graph = article::get_tags(&articles);

    if !only_docs {
        render_directory(
            "./templates/articles", "tera",
            EngineType::Tera,
            "./mdxts/articles", "md",
            Some(&meta_article_context(&articles, &tags_graph)),
            None,
            None,
            true
        ).unwrap();

        render_tag_pages(&tags_graph);
    }

    let articles_metadata = render_articles_mdxt(only_docs);
    render_articles_html(only_docs, articles_metadata);
    render_styles_and_scripts();

    if first_time && !only_docs {
        render(only_docs, !first_time);
    }

    else {
        clean_up_results();
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

fn render_articles_mdxt(only_docs: bool) -> HashMap<String, Yaml> {

    copy_all(
        "./mdxts", "jpg",
        "./output/htmls", "jpg",
        true
    ).unwrap();

    copy_all(
        "./mdxts", "png",
        "./output/htmls", "png",
        true
    ).unwrap();

    let mdxts_logs = render_directory(
        "./mdxts", "md",
        EngineType::MDxt,
        "./htmls", "html",
        None,
        None,
        None,
        true
    ).unwrap();

    if !only_docs {
        copy_all(
            "./htmls/tag_pages", "html",
            "./htmls/articles", "html",
            true
        ).unwrap();
    }

    update_articles_metadata(mdxts_logs)
}

fn update_articles_metadata(mdxts_logs: Vec<Log>) -> HashMap<String, Yaml> {

    let mut logs_hash = yaml_hash::new();
    let mut articles_metadata = HashMap::with_capacity(mdxts_logs.len());

    for Log { file_from, file_to, metadata } in mdxts_logs.into_iter() {
        logs_hash = yaml_hash::insert(logs_hash, Yaml::from_str(&file_from), metadata.clone());
        articles_metadata.insert(file_name(&file_from).unwrap(), metadata);
    }

    let mut yaml_hash_string = String::new();
    let mut emitter = YamlEmitter::new(&mut yaml_hash_string);
    emitter.dump(&logs_hash).unwrap();

    write_to_file("./output/articles.yaml", yaml_hash_string.as_bytes()).unwrap();

    articles_metadata
}

// affected by configs
fn render_articles_html(only_docs: bool, articles_metadata: HashMap<String, Yaml>) {

    render_directory(
        "./templates/pages", "md",
        EngineType::MDxt,
        "./templates/pages", "html",
        None,
        None,
        None,
        true
    ).unwrap();

    if !only_docs {
        render_templates(
            "./templates/pages/article.tera",
            "./htmls/articles", "html",
            "./output/htmls/articles", "html",
            None,
            Some(article_context()),
            true
        ).unwrap();

        render_directory(
            "./output/htmls/articles", "html",
            EngineType::XML,
            "./output/htmls/articles", "html",
            None,
            None,
            Some(&articles_metadata),
            true
        ).unwrap();
    }

    render_templates(
        "./templates/pages/article.tera",
        "./htmls/documents", "html",
        "./output/htmls/documents", "html",
        None,
        Some(document_context()),
        true
    ).unwrap();

    render_directory(
        "./output/htmls/documents", "html",
        EngineType::XML,
        "./output/htmls/documents", "html",
        None,
        None,
        Some(&articles_metadata),
        true
    ).unwrap();
}


// affected by configs
fn render_styles_and_scripts() {

    let color_context = get_colors();

    // Render Javascripts

    render_directory(
        "./templates/js", "tera",
        EngineType::Tera,
        "./templates/js", "js",
        Some(&color_context),
        None,
        None,
        false
    ).unwrap();

    write_to_file(
        "./templates/js/collapsible_tables.js",
        mdxt::collapsible_table_javascript().as_bytes()
    ).unwrap();

    copy_all(
        "./templates/js", "js",
        "./output/styles_and_scripts", "js",
        true
    ).unwrap();

    // Render SCSS files

    render_directory(
        "./templates/scss", "tera",
        EngineType::Tera,
        "./output/styles_and_scripts", "scss",
        Some(&color_context),
        None,
        None,
        false
    ).unwrap();

    copy_all(
        "./templates/scss", "scss",
        "./output/styles_and_scripts", "scss",
        true
    ).unwrap();

    render_directory(
        "./output/styles_and_scripts", "scss",
        EngineType::Scss,
        "./output/styles_and_scripts", "css",
        None,
        None,
        None,
        false
    ).unwrap();

    for sub_dir in get_sub_directories_recursive("./output") {
        copy_all(
            "./output/styles_and_scripts", "css",
            &sub_dir, "css",
            true
        ).unwrap();
        copy_all(
            "./output/styles_and_scripts", "js",
            &sub_dir, "js",
            true
        ).unwrap();
    }
}

fn render_tag_pages(tags_graph: &graph::Graph) {

    let mut tera_instance = tera::Tera::default();
    tera_instance.add_template_file("./templates/pages/tag.tera", Some("tag_page")).unwrap();
    mkdir("./mdxts/tag_pages");

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

        for ([year, month, day], name, preview) in articles_by_month.into_iter() {

            if [year, month] != curr_month {

                if articles_by_month_page_tmp.len() > 0 {
                    articles_by_month_page.push(articles_by_month_page_tmp.join("\n"));
                }

                articles_by_month_page_tmp = vec![format!("### {} {}", MONTHS[month], year), String::new()];
                curr_month = [year, month];
            }

            articles_by_month_page_tmp.push(format!("- [[giant]] [{}]({}.html) [[/giant]] [[blank=3]]- {}.{}.{}\n  - {}", name, name, day, MONTHS[month], year, preview));
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

// affected by configs
fn article_context() -> tera::Context {

    let mut context = tera::Context::new();
    let nav = read_string("./templates/pages/nav.html").unwrap();
    let header = read_string("./templates/pages/header.html").unwrap();
    let footer = read_string("./templates/pages/footer.html").unwrap();

    context.insert("nav", &nav);
    context.insert("header", &header);
    context.insert("footer", &footer);
    context.insert("csses", &vec!["markdown.css", "page_common.css", "page_blog.css", "nav.css", "header.css"]);

    context
}

// affected by configs
fn document_context() -> tera::Context {

    let mut context = tera::Context::new();
    let nav = read_string("./templates/pages/nav.html").unwrap();

    context.insert("nav", &nav);
    context.insert("csses", &vec!["markdown.css", "page_common.css", "page_doc.css", "nav.css"]);

    context
}

// DO NOT unwrap these!!
fn remove_results() {
    rmdir("./htmls");
    rmdir("./output/htmls");
    rmdir("./output/styles_and_scripts");
    rmdir("./mdxts/tag_pages");
}

// DO NOT unwrap these!!
fn clean_up_results() {
    rmdir("./htmls/tag_pages");
    rmdir("./mdxts/tag_pages");
    rmdir("./output/htmls/tag_pages");
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