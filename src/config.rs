use crate::file_io::*;
use crate::yaml_hash;
use std::collections::{HashMap, HashSet};
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

    font_size_landscape: u32,    // in pixels
    font_size_portrait: u32,     // in pixels

    default_horiz_padding: u32,  // in pixels

    pub titles: HashMap<String, String>,  // not in context
    pub ignores: HashSet<String>,         // not in context
}

impl Config {
    pub fn to_tera_context(&self) -> Context {
        let mut context = Context::new();

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

        context.insert("font_size_landscape", &self.font_size_landscape);
        context.insert("font_size_portrait", &self.font_size_portrait);

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

        match yaml_hash::get(&yaml, &Yaml::from_str("article_width_landscape")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_width_landscape = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_width_portrait")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_width_portrait = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_top_margin_landscape")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_top_margin_landscape = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_top_margin_portrait")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_top_margin_portrait = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_bottom_margin_landscape")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_bottom_margin_landscape = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_bottom_margin_portrait")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_bottom_margin_portrait = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_left_margin_landscape")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_left_margin_landscape = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_left_margin_portrait")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_left_margin_portrait = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_right_margin_landscape")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_right_margin_landscape = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("article_right_margin_portrait")) {
            Some(s) => match s.as_str() {
                Some(s) => {
                    self.article_right_margin_portrait = s.to_string();
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("font_size_landscape")) {
            Some(s) => match s.as_i64() {
                Some(s) if 0 <= s && s < u32::MAX as i64 => {
                    self.font_size_landscape = s as u32;
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("font_size_portrait")) {
            Some(s) => match s.as_i64() {
                Some(s) if 0 <= s && s < u32::MAX as i64 => {
                    self.font_size_portrait = s as u32;
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("default_horiz_padding")) {
            Some(s) => match s.as_i64() {
                Some(s) if 0 <= s && s < u32::MAX as i64 => {
                    self.default_horiz_padding = s as u32;
                },
                _ => {}
            },
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("titles")) {
            Some(s) => match s.as_hash() {
                Some(h) => {
                    let mut titles = HashMap::new();

                    for (k, v) in h.iter() {
                        let (k, v) = (k.as_str(), v.as_str());

                        if k.is_none() || v.is_none() {
                            continue;
                        }

                        titles.insert(k.unwrap().to_string(), v.unwrap().to_string());
                    }

                    self.titles = titles;
                },
                _ => {}
            }
            _ => {}
        }

        match yaml_hash::get(&yaml, &Yaml::from_str("ignores")) {
            Some(s) => match s.as_vec() {
                Some(v) => {
                    let mut ignores = HashSet::new();

                    for i in v.iter() {

                        match i.as_str() {
                            Some(ii) => {
                                ignores.insert(ii.to_string());
                            }
                            _ => {}
                        }

                    }

                    self.ignores = ignores;
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn default_article() -> Self {
        let mut titles = HashMap::new();
        titles.insert("index".to_string(), "Blog".to_string());

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
            font_size_landscape: 21,
            font_size_portrait: 16,
            default_horiz_padding: 0,
            titles,
            ignores: HashSet::new()
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
            font_size_landscape: 19,
            font_size_portrait: 14,
            default_horiz_padding: 96,
            titles: HashMap::new(),
            ignores: HashSet::new()
        }
    }
}
