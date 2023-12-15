use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct PageRank {
    mod_list: Vec<Vec<usize>>,
    num: usize,
}

impl PageRank {
    // Read graph data from a file.
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Could not open file");
        let buf_reader = io::BufReader::new(file).lines();

        let mut mod_list: Vec<Vec<usize>> = Vec::new();
        let mut num: usize = 0;

        // Iterate over lines in the file.
        for (i, line) in buf_reader.enumerate() {
            let line_str = line.expect("Error reading line");
            if i == 0 {
                // The first line contains the number of vertices.
                num = line_str.parse::<usize>().unwrap();
                mod_list = vec![vec![]; num];
            } else {
                // Parse the vertex indices and update the mod_list.
                let v: Vec<&str> = line_str.trim().split(' ').collect();
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                mod_list[x].push(y);
            }
        }

        return PageRank { mod_list, num }
    }

    // Conducts random walk starting from the given vertex and counts occurrences
    fn random_walk(&self, start: usize, iterations: usize) -> i32 {
        let mut count: i32 = 0;
        let mut rand_num = rand::thread_rng();

        for _ in 0..self.mod_list.len() {
            let mut current: usize = start;

            // Simulate a random walk through the graph.
            for _ in 0..iterations {
                if self.mod_list[current].is_empty() {
                    current = rand_num.gen_range(0..self.mod_list.len());
                } else {
                    let p: i32 = rand_num.gen_range(1..=10);
                    if p <= 9 {
                        let index: usize = rand_num.gen_range(0..self.mod_list[current].len());
                        current = self.mod_list[current][index];
                    } else {
                        current = rand_num.gen_range(0..self.mod_list.len());
                    }
                }
                if current == start {
                    count += 1;
                }
            }
        }

        count
    }

    

    // Calculates the PageRank values for each vertex in the graph.
    pub fn calculate_pagerank(&self, iterations: usize) -> HashMap<usize, f64> {
        let mut vertex_count: HashMap<usize, f64> = HashMap::new();

        // Iterate over all vertices and conduct random walks.
        for i in 0..self.mod_list.len() {
            let count = self.random_walk(i, iterations);
            let pg_rank_val: f64 = (count as f64) / ((self.num*iterations) as f64);
            vertex_count.insert(i, pg_rank_val);
        }

        vertex_count
    }
}
