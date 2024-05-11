use std::fs::File;
use std::io::{self, BufRead, BufReader};
use cluster::Graph as ClusterGraph;

mod cluster;
mod test;

struct Edge {
    state: String,
    age_group: String,
    condition: String,
}

impl From<Edge> for (String, String, String) {
    fn from(edge: Edge) -> (String, String, String) {
        (edge.state, edge.age_group, edge.condition)
    }
}

fn main() -> io::Result<()> {
    // opening the csv file
    let file = File::open("/Users/kirstensaint-fort/Desktop/Project/KSaintFortFinal/Conditions_Contributing_to_COVID-19_Deaths__by_State_and_Age__Provisional_2020-2023 (1).csv")?;
    let reader = BufReader::new(file);

    // vector to store the edges
    let mut edges: Vec<(String, String, String)> = Vec::new();

    // iteration
    for line in reader.lines() {
        let line = line?;

        
        let fields: Vec<&str> = line.split(',').collect();

        // extracting state, age group, and condition from each line
        let state = fields[6].trim().to_string();
        let age_group = fields[10].trim().to_string();
        let condition = fields[8].trim().to_string();

        // putting edges in the edges list
        let edge = Edge {
            state,
            age_group,
            condition,
        };
        edges.push(edge.into());
    }

    // graph construction from the edges
    let cluster_graph = ClusterGraph::new(edges);

    // building the adjacency list from the edges
    let adjacency_list = cluster_graph.build_adjacency_list();
    
    //best value of k
    let max_k = 10; 
    let max_iterations = 100; 
    let best_k = cluster_graph.find_best_k_silhouette(max_k, max_iterations);

    println!("Best value of k: {}", best_k);
    
    

    Ok(())
}