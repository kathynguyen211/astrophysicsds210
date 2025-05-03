use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
//this module is for loadting the dataset into graph

//creating graph structure contains list of node ID and undirected edges
pub struct Graph{
    pub nodes: Vec<String>, //map each author ID to a base index
    _idx: HashMap<String,usize>,// lookup table to avoid duplicate
    pub edges: Vec<(usize,usize)>, //index pairs
}

//load file into the graph
pub fn load_data(path:&str) -> Result<Graph, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
// create empty Vec and Hashmap to store data
    let mut nodes = Vec::new();
    let mut idx = HashMap::new();
    let mut edges = Vec::new();

    for lines in reader.lines(){ 
        let line = lines?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect(); // trimming the whitespace between value
// convert first and second node of part into string 
        let x = parts[0].to_string();
        let y = parts[1].to_string();

//either assign into node if it is not there yet or look up index and return index
        let idx_x = *idx.entry(x.clone()).or_insert_with(|| {nodes.push(x.clone()); nodes.len()-1});
        let idx_y = *idx.entry(y.clone()).or_insert_with(|| {nodes.push(y.clone()); nodes.len()-1});

        edges.push((idx_x,idx_y));
    }
    Ok(Graph{nodes, _idx: idx, edges})
} 
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_load_data() {
         let test_data = load_data("test_data_file.txt").unwrap();
         assert_eq! (test_data.nodes.len(),3 );
         assert_eq! (test_data.edges.len(), 3);
    }
}