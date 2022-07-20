use crate::yaml_hash;
use crate::file_io::*;
use crate::graph::Graph;
use yaml_rust::Yaml;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Article {
    name: String,
    date: [usize; 3],  // [year, month, day]  -> will replace it later
    tags: Vec<String>
}

impl Article {

    pub fn new(name: String, date: [usize; 3], tags: Vec<String>) -> Self {
        Article { name, date, tags }
    }

    pub fn date_to_int(&self) -> usize {
        self.date[0] * 1_0000 + self.date[1] * 100 + self.date[2]
    }

}

pub fn from_yaml(yaml: Yaml) -> HashMap<String, Article> {

    match yaml.as_hash() {
        None => HashMap::new(),
        Some(hash) => {
            let mut result = HashMap::with_capacity(hash.len());

            for (name, article) in hash.iter() {

                let date = match yaml_hash::get(&article, &Yaml::from_str("date")) {
                    Some(d) => match d.as_vec() {
                        Some(v) if v.len() == 3 => {
                            let year = v[0].as_i64();
                            let month = v[1].as_i64();
                            let day = v[2].as_i64();

                            if year.is_none() || month.is_none() || day.is_none() {
                                continue;
                            }

                            let year = year.unwrap();
                            let month = month.unwrap();
                            let day = day.unwrap();

                            if month * day == 0 || month > 12 || day > 31 {
                                continue;
                            }

                            [year as usize, month as usize, day as usize]
                        },
                        _ => {continue;}
                    },
                    _ => {continue;}
                };

                let tags = match yaml_hash::get(&article, &Yaml::from_str("tags")) {
                    Some(t) => match t.as_vec() {
                        Some(v) => v.into_iter().filter_map(|tag| tag.clone().into_string()).collect::<Vec<String>>(),
                        _ => {continue;}
                    },
                    _ => {continue;}
                };

                let name = match name.clone().into_string() {
                    Some(n) => n,
                    _ => {continue;}
                };

                result.insert(name.clone(), Article::new(name, date, tags));
            }

            result
        }
    }

}

pub fn get_recent_articles(articles: &HashMap<String, Article>, length: usize) -> Vec<String> {

    let mut articles_vec = articles.values().collect::<Vec<&Article>>();
    articles_vec.sort_unstable_by_key(|article| usize::MAX - article.date_to_int());

    articles_vec[0..length.min(articles_vec.len())].iter().map(
        |article| file_name(&article.name).unwrap()
    ).collect()
}

pub fn get_articles_by_month(articles: &HashMap<String, Article>) -> Vec<([usize; 2], String)> {  // Vec<([year, month], name)>

    let mut articles_vec = articles.values().collect::<Vec<&Article>>();
    articles_vec.sort_unstable_by_key(|article| usize::MAX - article.date_to_int());

    articles_vec.iter().map(
        |article| ([article.date[0], article.date[1]], file_name(&article.name).unwrap())
    ).collect()
}

pub fn get_tags(articles: &HashMap<String, Article>) -> Graph {

    let mut graph = Graph::new();

    for article in articles.values() {

        for tag in article.tags.iter() {
            graph.add_vertex(tag.clone());
        }

        if article.tags.len() > 1 {

            for i in 0..article.tags.len() {

                for j in i + 1..article.tags.len() {
                    graph.add_edge((article.tags[i].clone(), article.tags[j].clone()));
                }

            }

        }

    }

    graph
}