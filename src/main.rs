// Daniel Lung
// HW10
// Collaborators: None

mod pagerank;
use pagerank::PageRank;

fn main() {
    let data = PageRank::new("pagerank_data.txt");

    let iterations = 100;

    let pagerank_values = data.calculate_pagerank(iterations);

    let mut x: Vec<_> = pagerank_values.iter().collect();
    x.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    x.truncate(5);

    for (vertex, rank) in x {
        println!("The vertex {}: approximate PageRank {}", vertex, rank);
    }
}
