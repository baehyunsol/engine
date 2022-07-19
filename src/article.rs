pub struct Article {
    name: String,
    date: [u32; 3],  // [year, month, day]  -> will replace it later
    tags: Vec<String>
}

pub fn from_yaml(yaml: Yaml) -> HashMap<String, Article> {}