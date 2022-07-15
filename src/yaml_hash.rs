use yaml_rust::{Yaml, yaml::Hash};

pub fn new() -> Yaml {
    Yaml::Hash(Hash::new())
}

pub fn from_yaml(yaml: Yaml) -> Yaml {
    match yaml.as_hash() {
        None => new(),
        _ => yaml
    }
}

pub fn insert(hash: Yaml, key: Yaml, value: Yaml) -> Yaml {

    let mut hash = match hash.into_hash() {
        Some(h) => h,
        _ => panic!()
    };

    hash.insert(key, value);

    Yaml::Hash(hash)
}

pub fn get<'a, 'b>(hash: &'a Yaml, key: &'b Yaml) -> Option<&'a Yaml> {

    let hash = match hash.as_hash() {
        Some(h) => h,
        _ => panic!()
    };

    hash.get(key)
}