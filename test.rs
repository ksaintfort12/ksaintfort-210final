#[cfg(test)]
mod tests {
    use crate::cluster::Graph;
    use std::collections::{HashMap, HashSet};
    
    #[test]
    fn test_build_adjacency_list() {
        let edges = vec![
            ("A".to_string(), "B".to_string(), "1".to_string()),
            ("B".to_string(), "C".to_string(), "2".to_string()),
            ("C".to_string(), "A".to_string(), "3".to_string()),
        ];
        let graph = Graph::new(edges);
        
        let adjacency_list = graph.build_adjacency_list();
        
        assert_eq!(adjacency_list.len(), 3);
        assert_eq!(adjacency_list[&0], HashSet::from_iter(vec![1]));
        assert_eq!(adjacency_list[&1], HashSet::from_iter(vec![0, 2]));
        assert_eq!(adjacency_list[&2], HashSet::from_iter(vec![1]));
    }
    #[test]
    fn test_find_best_k_silhouette() {
        // graph with test data
        let edges = vec![
            ("A".to_string(), "B".to_string(), "1".to_string()),
            ("B".to_string(), "C".to_string(), "2".to_string()),
            ("C".to_string(), "A".to_string(), "3".to_string()),
            ("D".to_string(), "E".to_string(), "4".to_string()),
            ("E".to_string(), "F".to_string(), "5".to_string()),
        ];
        let graph = Graph::new(edges);

        
        let best_k = find_best_k_silhouette_test(&graph);

        
        assert_eq!(best_k, 3);
    }

    
    fn silhouette_score(_: &HashMap<usize, usize>) -> f64 {
        // Return a fixed silhouette score for testing
        0.5
    }

    // Function to find best k using silhouette score (mock implementation)
    fn find_best_k_silhouette_test(graph: &Graph) -> usize {
        let mut best_k = 2; // begin with k=2
        let mut max_silhouette = 0.0;

        for k in 2..=5 { // Change the range based on your test case
            let silhouette = silhouette_score(&HashMap::new()); // Pass a mock assignment
            
            if silhouette > max_silhouette {
                best_k = k;
                max_silhouette = silhouette;
            }
        }

        best_k
    }
}
