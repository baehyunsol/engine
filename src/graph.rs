use std::collections::HashMap;

// matrix implementation
// vertexes are the tags
pub struct Graph {
    edges: Vec<Vec<usize>>,  // undirected weighted graph
    vertex_indexes: HashMap<String, usize>,  // use `String`s as indexes
    reverse_vertex_indexes: HashMap<usize, String>,
    tag_articles: Vec<Vec<String>>  // articles that have the tag
}

impl Graph {

    pub fn new() -> Self {
        Graph::with_capacity(8)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Graph {
            edges: vec![vec![0; capacity]; capacity],
            vertex_indexes: HashMap::with_capacity(capacity),
            reverse_vertex_indexes: HashMap::with_capacity(capacity),
            tag_articles: Vec::with_capacity(capacity)
        }
    }

    pub fn len(&self) -> usize {
        self.vertex_indexes.len()
    }

    pub fn iter(&self) -> Vec<String> {
        let mut result: Vec<String> = self.vertex_indexes.keys().map(|s| s.to_string()).collect();
        result.sort_unstable();

        result
    }

    pub fn add_vertex(&mut self, tag_name: String, article_name: String) {

        if self.vertex_indexes.contains_key(&tag_name) {
            self.tag_articles[*self.vertex_indexes.get(&tag_name).unwrap()].push(article_name);
        }

        else {
            let index = self.vertex_indexes.len();

            self.vertex_indexes.insert(tag_name.clone(), index);
            self.reverse_vertex_indexes.insert(index, tag_name);
            self.tag_articles.push(vec![article_name]);

            if self.edges.len() <= index {
                let mut new_edges = vec![vec![0; index * 2]; index * 2];

                for i in 0..self.edges.len() {

                    for j in 0..self.edges.len() {
                        new_edges[i][j] = self.edges[i][j];
                    }

                }

                self.edges = new_edges;
            }

        }

    }

    pub fn add_edge(&mut self, edge: (String, String)) {

        if !self.vertex_indexes.contains_key(&edge.0) || !self.vertex_indexes.contains_key(&edge.1) {
            panic!();
        }

        let v1 = self.vertex_indexes.get(&edge.0).unwrap();
        let v2 = self.vertex_indexes.get(&edge.1).unwrap();

        self.edges[*v1][*v2] += 1;
        self.edges[*v2][*v1] += 1;
    }

    // it sorts the result in descending order
    pub fn get_vertex_weights(&self) -> Vec<(String, usize)> {  // Vec<(vertex_name, vertex_weight)>
        let mut result = self.vertex_indexes.iter().map(
            |(name, index)|
            (name.clone(), self.tag_articles[*index].len())
        ).collect::<Vec<(String, usize)>>();

        // vertices with the same weights are sorted alphabetically
        result.sort_unstable_by_key(|(name, _)| name.to_string());
        result.sort_by_key(|(_, weight)| usize::MAX - *weight);

        result
    }

    pub fn get_articles(&self, tag: String) -> Vec<String> {

        match self.vertex_indexes.get(&tag) {
            Some(tag_index) => self.tag_articles[*tag_index].clone(),
            _ => vec![]
        }

    }

    // it sorts the result in descending order
    pub fn get_adjacent_vertexes(&self, tag: String) -> Vec<(String, usize)> {  // Vec<(vertex_name, edge_weight)>
        
        match self.vertex_indexes.get(&tag) {
            None => vec![],
            Some(ind) => {
                let mut result = vec![];

                for (edge_index, edge_weight) in self.edges[*ind].iter().enumerate() {

                    if *edge_weight > 0 {
                        result.push((self.reverse_vertex_indexes.get(&edge_index).unwrap().clone(), *edge_weight));
                    }

                }

                result.sort_unstable_by_key(|(name, _)| name.to_string());
                result.sort_by_key(|(_, weight)| usize::MAX - *weight);

                result
            }

        }

    }

}