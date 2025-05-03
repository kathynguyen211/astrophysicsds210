mod graph; 
mod partition;
use std::error::Error;
 // calling each public function in each module
fn main() -> Result<(), Box<dyn Error>> {
    let graph = graph::load_data("CA-GrQc.txt")?;
    println!("Graph has {} nodes and {} edges", graph.nodes.len(), graph.edges.len());
    let parts = partition::graph_partition(&graph, 11);
    for (i, subset) in parts.iter().enumerate(){
        println!("Subfield {:>2} : {} authors ", i +1, subset.len());
    }
    let average_degree = partition::avg_degree(&graph);
    println!("Average degree: {:.2}", average_degree);
    let edge_cut= partition::edge_cut(&graph, &parts);
    println!("edge-cut:{}", edge_cut);
Ok(())
}