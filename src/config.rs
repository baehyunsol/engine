use crate::file_io::*;
use crate::yaml_hash;
use tera::Context;
use yaml_rust::{Yaml, YamlLoader};

pub fn load_articles_config() -> Config {
    let mut config = Config::default_article();

    match read_string("./configs/articles.yaml") {
        Ok(y) => match YamlLoader::load_from_str(&y) {
            Ok(y) => {
                config.merge_yaml(y[0].clone());
            }
            _ => {}
        },
        _ => {}
    }

    config
}

pub fn load_documents_config() -> Config {
    let mut config = Config::default_document();

    match read_string("./configs/documents.yaml") {
        Ok(y) => match YamlLoader::load_from_str(&y) {
            Ok(y) => {
                config.merge_yaml(y[0].clone());
            }
            _ => {}
        },
        _ => {}
    }

    config
}

pub struct Config {
    pub has_header: bool,
    pub has_nav: bool,
    pub has_footer: bool,

    article_width_landscape: String,
    article_top_margin_landscape: String,
    article_bottom_margin_landscape: String,
    article_left_margin_landscape: String,
    article_right_margin_landscape: String,
    article_width_portrait: String,
    article_top_margin_portrait: String,
    article_bottom_margin_portrait: String,
    article_left_margin_portrait: String,
    article_right_margin_portrait: String,

    default_horiz_padding: u32,  // in pixels
}

impl Config {

    pub fn to_tera_context(&self) -> Context {
        let mut context = Context::new();

        context.insert("has_header", &self.has_header);
        context.insert("has_nav", &self.has_nav);
        context.insert("has_footer", &self.has_footer);

        context.insert("article_width_landscape", &self.article_width_landscape);
        context.insert("article_top_margin_landscape", &self.article_top_margin_landscape);
        context.insert("article_bottom_margin_landscape", &self.article_bottom_margin_landscape);
        context.insert("article_left_margin_landscape", &self.article_left_margin_landscape);
        context.insert("article_right_margin_landscape", &self.article_right_margin_landscape);

        context.insert("article_width_portrait", &self.article_width_portrait);
        context.insert("article_top_margin_portrait", &self.article_top_margin_portrait);
        context.insert("article_bottom_margin_portrait", &self.article_bottom_margin_portrait);
        context.insert("article_left_margin_portrait", &self.article_left_margin_portrait);
        context.insert("article_right_margin_portrait", &self.article_right_margin_portrait);

        context.insert("default_horiz_padding", &self.default_horiz_padding);

        context
    }

    fn merge_yaml(&mut self, yaml: Yaml) {

        if !yaml_hash::is_hash(&yaml) {
            return;
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("has_header")) {
            Some(b) => match b.as_bool() {
                Some(b) => {
                    self.has_header = b;
                },
                _ => {}
            }
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("has_nav")) {
            Some(b) => match b.as_bool() {
                Some(b) => {
                    self.has_nav = b;
                },
                _ => {}
            }
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("has_footer")) {
            Some(b) => match b.as_bool() {
                Some(b) => {
                    self.has_footer = b;
                },
                _ => {}
            }
            _ => {}
        }

    }

    fn default_article() -> Self {
        Config {
            has_header: true,
            has_nav: true,
            has_footer: true,
            article_width_landscape: "82%".to_string(),
            article_top_margin_landscape: "$header-height + $padding-big".to_string(),
            article_bottom_margin_landscape: "0px".to_string(),
            article_left_margin_landscape: "9%".to_string(),
            article_right_margin_landscape: "9%".to_string(),
            article_width_portrait: "90%".to_string(),
            article_top_margin_portrait: "$header-height + $padding-big".to_string(),
            article_bottom_margin_portrait: "0px".to_string(),
            article_left_margin_portrait: "5%".to_string(),
            article_right_margin_portrait: "5%".to_string(),
            default_horiz_padding: 0,
        }
    }

    fn default_document() -> Self {
        Config {
            has_header: false,
            has_nav: true,
            has_footer: false,
            article_width_landscape: "94%".to_string(),
            article_top_margin_landscape: "0px".to_string(),
            article_bottom_margin_landscape: "0px".to_string(),
            article_left_margin_landscape: "3%".to_string(),
            article_right_margin_landscape: "3%".to_string(),
            article_width_portrait: "94%".to_string(),
            article_top_margin_portrait: "0px".to_string(),
            article_bottom_margin_portrait: "0px".to_string(),
            article_left_margin_portrait: "3%".to_string(),
            article_right_margin_portrait: "3%".to_string(),
            default_horiz_padding: 96,
        }
    }

}