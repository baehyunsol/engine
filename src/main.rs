mod article;
mod config;
mod engine;
mod error;
mod file_io;
mod graph;
mod log;
mod xml;
mod yaml_hash;

use rayon::prelude::*;
use config::*;
use engine::*;
use lazy_static::lazy_static;
use file_io::*;
use log::Log;
use std::collections::HashMap;
use std::time::Instant;
use yaml_rust::{
    Yaml,
    YamlEmitter,
    YamlLoader,
};

#[derive(Copy, Clone, PartialEq)]
pub enum MultiCore {
    Auto,
    Enable,
    Disable
}

pub const MULTICORE_THRESHOLD: usize = 24;

fn main() {
    let mut only_docs = false;
    let mut verbose = false;
    let mut multi_core = MultiCore::Auto;

    let args = std::env::args().collect::<Vec<String>>();

    for arg in args[1..].iter() {

        if arg == "--doc" || arg == "-d" {
            only_docs = true;
        }

        else if arg == "--all" || arg == "-a" {
            only_docs = false;
        }

        else if arg == "--clear" || arg == "-c" {
            clean_up_results();
            remove_results();
            return;
        }

        else if arg == "--verbose" {
            verbose = true;
        }

        else if arg == "-ma" {
            multi_core = MultiCore::Auto;
        }

        else if arg == "-me" {
            multi_core = MultiCore::Enable;
        }

        else if arg == "-md" {
            multi_core = MultiCore::Disable;
        }

        else if arg == "--help" || arg == "-h" {
            print_help_message();
            return;
        }

        else {
            println!("Invalid argument: {}\n", arg);
            print_help_message();
            return;
        }

    }

    render(only_docs, multi_core, verbose);
}

fn print_help_message() {
    println!("Engine v 0.1.0 (c) Baehyunsol

--all -a : render docs and articles
    it's -a by default

--clear -c : clear results

--doc -d : render only docs

--help -h : help message

--verbose: verbose

-ma -me -md : configure multi core usage
    -ma: enable multi core rendering when there're lots of files to render
        it's -ma by default
    -me: always enable multi core rendering
    -md: always disable multi core rendering
");
}

fn render(only_docs: bool, multi_core: MultiCore, verbose: bool) {
    let start_time = Instant::now();

    let article_configs = load_articles_config();
    let doc_configs = load_documents_config();

    if verbose { show_verbose_message(start_time.clone(), "Configs loaded"); }

    let mut doc_configs_context = doc_configs.to_tera_context();
    let color_context = get_colors();

    doc_configs_context.extend(color_context);

    if verbose { show_verbose_message(start_time.clone(), "Doc Config context loaded"); }

    remove_results();

    if verbose { show_verbose_message(start_time.clone(), "`remove_result` complete"); }

    // docs & articles, mdxt

    let mut mdxts_logs = render_directory(
        "./mdxts/documents", "md",
        EngineType::MDxt,
        "./htmls/documents", "html",
        None,
        None,
        None,
        &doc_configs,
        true,  // recursive
        multi_core,
        true,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(documents, MDxt)` complete"); }

    if !only_docs {
        let mdxts_logs_articles = render_directory(
            "./mdxts/articles", "md",
            EngineType::MDxt,
            "./htmls/articles", "html",
            None,
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            true,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(articles, MDxt)` complete"); }

        mdxts_logs = vec![
            mdxts_logs,
            mdxts_logs_articles
        ].concat();
    }

    let articles_metadata = update_articles_metadata(mdxts_logs, !only_docs);

    // docs, styles

    render_directory(
        "./templates/scss", "tera",
        EngineType::Tera,
        "./output/htmls/documents", "scss",
        Some(&doc_configs_context),
        None,
        None,
        &doc_configs,
        true,  // recursive
        multi_core,
        true,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(scss -> documents, Tera)` complete"); }

    copy_all(
        "./templates/scss", "scss",
        "./output/htmls/documents", "scss",
        true,
        multi_core,
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`copy_all(scss)` complete"); }

    render_directory(
        "./output/htmls/documents", "scss",
        EngineType::Scss,
        "./output/htmls/documents", "css",
        None,
        None,
        None,
        &doc_configs,
        true,  // recursive
        multi_core,
        false,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(documents, Scss)` complete"); }

    // docs, js

    render_directory(
        "./templates/js", "tera",
        EngineType::Tera,
        "./output/htmls/documents", "js",
        Some(&doc_configs_context),
        None,
        None,
        &doc_configs,
        true,  // recursive
        multi_core,
        true,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(js -> documents, Tera)` complete"); }

    copy_all(
        "./templates/js", "js",
        "./output/htmls/documents", "js",
        true,
        multi_core,
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`copy_all(js -> documents)` complete"); }

    // doc, page_templates

    // not configurable yet
    /*render_directory(
        "./templates/pages", "tera",
        EngineType::Tera,
        "./templates/pages", "md",
        Some(&doc_configs_context),
        None,
        None,
        true
    ).unwrap();*/

    render_directory(
        "./templates/pages", "md",
        EngineType::MDxt,
        "./templates/pages", "html",
        None,
        None,
        None,
        &doc_configs,
        true,  // recursive
        multi_core,
        false,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(template, MDxt)` complete"); }

    // docs, html_template

    render_templates(
        "./templates/pages/article.tera",
        "./htmls/documents", "html",
        "./output/htmls/documents", "html",
        None,
        Some(get_page_template_context(&doc_configs)),
        &doc_configs,
        true,
        multi_core,
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_templates(documents)` complete"); }

    // docs, xml

    render_directory(
        "./output/htmls/documents", "html",
        EngineType::XML,
        "./output/htmls/documents", "html",
        None,
        None,
        Some(&articles_metadata),
        &doc_configs,
        true,  // recursive
        multi_core,
        false,  // cache_read_dir
    ).unwrap();

    if verbose { show_verbose_message(start_time.clone(), "`render_directory(documents, XML)` complete"); }

    // articles
    if !only_docs {
        let mut article_configs_context = article_configs.to_tera_context();
        let color_context = get_colors();

        article_configs_context.extend(color_context);

        // articles, scss
        render_directory(
            "./templates/scss", "tera",
            EngineType::Tera,
            "./output/htmls/articles", "scss",
            Some(&article_configs_context),
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            true,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(scss -> articles, Tera)` complete"); }

        copy_all(
            "./templates/scss", "scss",
            "./output/htmls/articles", "scss",
            true,
            multi_core,
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`copy_all(scss -> articles)` complete"); }

        render_directory(
            "./output/htmls/articles", "scss",
            EngineType::Scss,
            "./output/htmls/articles", "css",
            None,
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            false,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(articles, Scss)` complete"); }

        // articles, js

        render_directory(
            "./templates/js", "tera",
            EngineType::Tera,
            "./output/htmls/articles", "js",
            Some(&article_configs_context),
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            true,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(js -> articles, Tera)` complete"); }

        copy_all(
            "./templates/js", "js",
            "./output/htmls/articles", "js",
            true,
            multi_core,
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`copy_all(js -> articles)` complete"); }

        // articles, meta_articles

        let articles = match read_string("./output/articles.yaml") {
            Ok(data) => article::from_yaml(YamlLoader::load_from_str(&data).unwrap()[0].clone()),
            Err(_) => article::from_yaml(YamlLoader::load_from_str("{}").unwrap()[0].clone())
        };

        let tags_graph = article::get_tags(&articles);

        if verbose { show_verbose_message(start_time.clone(), "`articles.yaml` loaded"); }

        render_tag_pages(&tags_graph);

        if verbose { show_verbose_message(start_time.clone(), "`render_tag_pages()` complete"); }

        render_directory(
            "./templates/articles", "tera",
            EngineType::Tera,
            "./mdxts/articles", "md",
            Some(&meta_article_context(&articles, &tags_graph)),
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            false,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(articles, Tera)` complete"); }

        render_directory(
            "./mdxts/tag_pages", "md",
            EngineType::MDxt,
            "./htmls/articles", "html",
            None,
            None,
            None,
            &article_configs,
            true,  // recursive
            multi_core,
            false,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(tag_pages, MDxt)` complete"); }

        // articles, page_templates

        // not configurable yet
        /*render_directory(
            "./templates/pages", "tera",
            EngineType::Tera,
            "./templates/pages", "md",
            Some(&article_configs_context),
            None,
            None,
            true,
            multi_core,
        ).unwrap();*/

        // since it's not configurable yet, mdxt files don't have to be rendered twice
        /*render_directory(
            "./templates/pages", "md",
            EngineType::MDxt,
            "./templates/pages", "html",
            None,
            None,
            None,
            true,
            multi_core,
        ).unwrap();*/

        // articles, html_templates

        render_templates(
            "./templates/pages/article.tera",
            "./htmls/articles", "html",
            "./output/htmls/articles", "html",
            None,
            Some(get_page_template_context(&article_configs)),
            &article_configs,
            true,
            multi_core,
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_templates(articles)` complete"); }

        render_directory(
            "./output/htmls/articles", "html",
            EngineType::XML,
            "./output/htmls/articles", "html",
            None,
            None,
            Some(&articles_metadata),
            &article_configs,
            true,  // recursive
            multi_core,
            false,  // cache_read_dir
        ).unwrap();

        if verbose { show_verbose_message(start_time.clone(), "`render_directory(articles, XML)` complete"); }

        propagate_css_js("./output/htmls/articles", multi_core);

        if verbose { show_verbose_message(start_time.clone(), "`propagate_css_js(articles)` complete"); }

    }

    propagate_css_js("./output/htmls/documents", multi_core);

    if verbose { show_verbose_message(start_time.clone(), "`propagate_css_js(documents)` complete"); }

    // docs & articles: images, videos, and audios
    copy_media_files(only_docs, multi_core);

    if verbose { show_verbose_message(start_time.clone(), "`copy_media_files` complete"); }

    clean_up_results();

    if verbose { show_verbose_message(start_time.clone(), "`clean_up_results` complete"); }
}

fn copy_media_files(only_docs: bool, multi_core: MultiCore) {
    copy_media_files_ext("jpg", only_docs, multi_core);
    copy_media_files_ext("jpeg", only_docs, multi_core);
    copy_media_files_ext("png", only_docs, multi_core);
    copy_media_files_ext("svg", only_docs, multi_core);
    copy_media_files_ext("gif", only_docs, multi_core);
    copy_media_files_ext("m4a", only_docs, multi_core);
    copy_media_files_ext("mp4", only_docs, multi_core);
    copy_media_files_ext("mp3", only_docs, multi_core);
    copy_media_files_ext("wav", only_docs, multi_core);
    copy_media_files_ext("ogg", only_docs, multi_core);
    copy_media_files_ext("webm", only_docs, multi_core);
}

fn copy_media_files_ext(ext: &str, only_docs: bool, multi_core: MultiCore) {

    copy_all(
        "./mdxts/documents", ext,
        "./output/htmls/documents", ext,
        true,
        multi_core,
    ).unwrap();

    if !only_docs {
        copy_all(
            "./mdxts/articles", ext,
            "./output/htmls/articles", ext,
            true,
            multi_core,
        ).unwrap();
    }

}

fn propagate_css_js(path: &str, multi_core: MultiCore) {
    let mut files = read_dir(path, false).unwrap();

    files = files.into_iter().filter(
        |f| match extension(f) {
            Ok(ext) => {
                let ext_low = ext.to_lowercase();

                ext_low == "css" || ext_low == "js"
            },
            _ => false
        }
    ).collect();

    let sub_dirs = get_sub_directories_recursive(path);

    if sub_dirs.is_empty() {
        return;
    }

    if files.len() > MULTICORE_THRESHOLD && multi_core == MultiCore::Auto || multi_core == MultiCore::Enable {
        files.par_iter().map(
            |file| {

                for sub_dir in sub_dirs.iter() {
                    let file_name = basename(file).unwrap();
                    let file_dest = join(sub_dir, &file_name).unwrap();
                    write_to_file(&file_dest, &read_bytes(file).unwrap()).unwrap();
                }

                ()
            }
        ).collect::<Vec<()>>();
    }

    else {

        for file in files.iter() {

            for sub_dir in sub_dirs.iter() {
                let file_name = basename(file).unwrap();
                let file_dest = join(sub_dir, &file_name).unwrap();
                write_to_file(&file_dest, &read_bytes(file).unwrap()).unwrap();
            }

        }

    }

}

fn get_page_template_context(config: &Config) -> tera::Context {

    let mut context = tera::Context::new();
    let mut csses = vec!["page.css", "markdown.css"];

    if config.has_nav {
        let nav = read_string("./templates/pages/nav.html").unwrap();
        context.insert("nav", &nav);
        csses.push("nav.css");
    }

    if config.has_header {
        let header = read_string("./templates/pages/header.html").unwrap();
        context.insert("header", &header);
        csses.push("header.css");
    }

    if config.has_footer {
        let footer = read_string("./templates/pages/footer.html").unwrap();
        context.insert("footer", &footer);
        //csses.push("footer.css");  // `footer.css` doesn't exist
    }

    context.insert("csses", &csses);

    context

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

fn update_articles_metadata(mdxts_logs: Vec<Log>, save_to_file: bool) -> HashMap<String, Yaml> {
    let mut articles_metadata = HashMap::with_capacity(mdxts_logs.len());

    if save_to_file {
        let mut logs_hash = yaml_hash::new();

        for Log { file_from, metadata, .. } in mdxts_logs.into_iter() {
            logs_hash = yaml_hash::insert(logs_hash, Yaml::from_str(&file_from), metadata.clone());
            articles_metadata.insert(file_name(&file_from).unwrap(), metadata);
        }

        let mut yaml_hash_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_hash_string);
        emitter.dump(&logs_hash).unwrap();

        write_to_file("./output/articles.yaml", yaml_hash_string.as_bytes()).unwrap();
    }

    else {

        for Log { file_from, metadata, .. } in mdxts_logs.into_iter() {
            articles_metadata.insert(file_name(&file_from).unwrap(), metadata);
        }
    
    }

    articles_metadata
}

fn render_tag_pages(tags_graph: &graph::Graph) {
    let mut tera_instance = tera::Tera::default();
    tera_instance.add_template_file("./templates/pages/tag.tera", Some("tag_page")).unwrap();
    mkdir("./mdxts/tag_pages");  // don't unwrap this. If the path already exists, it'd raise an error, but that's fine.

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

#[inline]
fn show_verbose_message(start_time: Instant, message: &str) {
    println!("{}: {}ms", message, Instant::now().duration_since(start_time).as_millis());
}

// DO NOT unwrap these!!
fn remove_results() {
    rmdir("./htmls");
    rmdir("./output/htmls/articles");
    rmdir("./output/htmls/documents");
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