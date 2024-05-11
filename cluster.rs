use std::collections::{HashMap, HashSet};
extern crate rand; 
use rand::{self, seq::SliceRandom};
use euclid::default::Point2D;
use silhouette::Silhouette;

pub struct Graph {
    pub edges: Vec<(String, String, String)>,
}

impl Graph {
    pub fn new(edges: Vec<(String, String, String)>) -> Self {
        Graph { edges }
    }

    // building an auxiliarry function to create an adjacency list
    pub fn build_adjacency_list(&self) -> HashMap<usize, HashSet<usize>> {
        let mut adjacency_list: HashMap<usize, HashSet<usize>> = HashMap::new();

        // assigning labels to numbers
        let mut label_to_number: HashMap<String, usize> = HashMap::new();
        let mut current_label_number = 0;

        // iteration 
        for (src_label, dst_label, _) in &self.edges {
            let src_number = *label_to_number.entry(src_label.clone()).or_insert_with(|| {
                let number = current_label_number;
                current_label_number += 1;
                number
            });
            let dst_number = *label_to_number.entry(dst_label.clone()).or_insert_with(|| {
                let number = current_label_number;
                current_label_number += 1;
                number
            });

            // updating the adjacency list
            adjacency_list
                .entry(src_number)
                .or_insert_with(HashSet::new)
                .insert(dst_number);
            adjacency_list
                .entry(dst_number)
                .or_insert_with(HashSet::new)
                .insert(src_number);
        }

        adjacency_list
    }
    
    pub fn kmeans_cluster(&self, k: usize, max_iterations: usize) -> HashMap<usize, usize> {
        let adjacency_list = self.build_adjacency_list();
        let vertices: Vec<usize> = adjacency_list.keys().copied().collect();
        let mut rng = rand::thread_rng();
        let mut centroids: Vec<usize> = vertices.choose_multiple(&mut rng, k).copied().collect();
        let mut assignments: HashMap<usize, usize> = HashMap::new();
        
        let mut new_assignments: HashMap<usize, Vec<usize>>; 

        for _ in 0..max_iterations {
            new_assignments = HashMap::new(); 
            for vertex in &vertices {
                let mut min_distance = usize::MAX;
                let mut closest_centroid = centroids[0];
    
                for &centroid in &centroids {
                    let distance = (*vertex as isize - centroid as isize).abs() as usize;
                    if distance < min_distance {
                        min_distance = distance;
                        closest_centroid = centroid;
                    }
                }
    
                new_assignments.entry(closest_centroid).or_insert_with(Vec::new).push(*vertex);
            }
    
            let mut new_centroids: Vec<usize> = Vec::with_capacity(k);
            for (_, assigned_vertices) in &new_assignments {
                let centroid = assigned_vertices.iter().sum::<usize>() / assigned_vertices.len();
                new_centroids.push(centroid);
            }
    
            if centroids == new_centroids {
                break;
            }
    
            centroids = new_centroids;
        }
    
        for (centroid, assigned_vertices) in new_assignments {
            for vertex in assigned_vertices {
                assignments.insert(vertex, centroid);
            }
        }
    
        assignments
    }
    
    pub fn find_best_k_silhouette(&self, max_k: usize, max_iterations: usize) -> usize {
        let mut best_k = 2; // begin with k=2
        let mut max_silhouette = 0.0;

        for k in 2..=max_k {
            let assignments = self.kmeans_cluster(k, max_iterations);

            // converting assignments to Point2D format to make my life easier lol
            let points: Vec<Point2D<usize, f64>> = assignments.iter()
                .map(|(&vertex, &cluster)| Point2D::new(vertex as usize, cluster as f64))
                .collect();

            // silhouette coefficient!
            let silhouette = Silhouette::new(&points);

            if silhouette.avg_score() > max_silhouette {
                best_k = k;
                max_silhouette = silhouette.avg_score();
            }
        }

        best_k
    }
}
