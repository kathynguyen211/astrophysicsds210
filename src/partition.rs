use std::collections::HashMap;
use ndarray:: Array2;
use ndarray_linalg::{Eigh,UPLO };
use crate:: graph::Graph;
// this module is responsible for partitioning the graph repetitively as desire
fn spectral_bisect(
    graph: &Graph, // reference graph
    subset: &[usize],// input a slice of a number 
) -> (Vec<usize>, Vec<usize>){ // return a matrix of vec and vec
    let n = subset.len(); 
    // Basically in here, you create a Laplacian matrix - represent of the graph that contains all the connectivity
    let mut lap = Array2::<f64>::zeros((n,n));//create a matrix with n x n
    let mut index_map = HashMap::with_capacity(n);// create a look up map
    for (i, &node) in subset.iter().enumerate(){
        index_map.insert(node,i);
    }
    for &(u,v) in &graph.edges{ // for each edge in the graph
        if let (Some(i), Some(j)) = (index_map.get(&u).copied(),index_map.get(&v).copied()){ //if both endpoints are in the subset
            lap[(i,j)] += 1.0; // incease the adjency on the laplacian matrix by 1 in both way since it is undirected
            lap[(j,i)] += 1.0;
        }
    }
// computing unnormalized graph Laplacian
    for g in 0..n{ // for each node 
        let degree: f64 = lap.row(g).iter().sum(); // computing the node's degree by sum of its adjacency row
        lap[(g,g)] = degree; 
        for f in 0..n{ // for every other column f is different than g
            if g != f{
                lap[(g,f)] = -lap[(g,f)]; // negate it
            }
        }
    }
    let (_vals, vecs) = lap.eigh(UPLO::Lower).unwrap();//caling .eigh() from ndarray- linalg to calculate eigenvalue and eigenvector of lower triangle
    let fielder = vecs.column(1).to_owned(); // taking second smallest eigenvector (idx = 1)
    //each node in subset pair with fielder vector and sort
    let mut pairs: Vec<(usize, f64)> = subset.iter().enumerate().map(|(i, &node)| (node, fielder[i])).collect();
    pairs.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    // spliting into 2 partitions
    let mid = pairs.len() /2;
    (
        pairs[..mid].iter().map(|&(node,_)| node).collect(),
        pairs[mid..].iter().map(|&(node,_)| node).collect(),
    )
}

//Recursively partion node in k cluster by performing spectral bisection
pub fn graph_partition(
    graph: &Graph, //  calling the graph 
    k: usize, //desired number of partitons - subfields of research 
) -> Vec<Vec<usize>>{ // return vector of listing the node-indices in that partition.
    let mut parts: Vec<_> = vec![(0..graph.nodes.len()).collect::<Vec<_>>()]; // start with first part containing all the node indices.
    while parts.len() < k { // calling recursively
        let index = parts.iter().enumerate().max_by_key(|(_,p)| p.len()).unwrap().0;// pick the largest current part to split
        let subset = parts.remove(index);//remove it from the list
        let(p1,p2) = spectral_bisect(graph, &subset); //bisect the subset into two smaller subsets
        parts.push(p1); // add those two new subsets back into the list
        parts.push(p2);
    }
    parts // return parts
}
// compute a vector of degrees for each node index 
fn degree_distribution(graph: &Graph) -> Vec<usize> { 
    let mut degree = vec![0; graph.nodes.len()]; // create a vector of zeroes with n length
    for &(u,v) in &graph.edges { // for each tuple of edge in graph, increase degree by 1
        degree[u] +=1;
        degree[v] += 1;
    }
    degree
}
//compute average degree
pub fn avg_degree(graph: &Graph) -> f64{ // taking the average of all the degree of nodes above to get the average
    let deg = degree_distribution(graph);
    let sum: usize = deg.iter().sum();
    sum as f64 / graph.nodes.len() as f64
}
// calculate number of edges cross the subfields
pub fn edge_cut(graph: &Graph, parts:&[Vec<usize>]) -> usize{ 
    let mut idx = vec![0; graph.nodes.len()];// create vector of zeros with n length of node
    for (i, subset) in parts.iter().enumerate(){ // for each i as partion number and subset - vector of node indice
        for &node in subset{ // for each node in subset
            idx[node] = i; // set the index of node = i so we know which part is in
        }
    }
    // return count of edges that have endpoints in different part
    graph.edges.iter().filter(|&&(u,v)| idx[u]!= idx[v]).count()
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::graph::load_data;

    #[test]
    fn test_partition() {
        let graph = load_data("test_data_file.txt").unwrap();
        let parts = graph_partition(&graph, 2);
        assert_eq!(parts.len(), 2);
    }
}