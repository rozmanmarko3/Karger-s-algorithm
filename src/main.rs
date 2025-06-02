use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::{fs::read_to_string};
use union_find::{QuickUnionUf, UnionByRank, UnionFind};
use std::time::Instant;

#[derive(Clone,Debug)]
struct Edge {
    v1: usize,
    v2: usize,
}

fn string_to_edge(s: &str) -> Edge {
    let mut parts = s.split_whitespace();
    let v1 = parts
        .next()
        .expect("Missing first vertex")
        .parse::<usize>()
        .expect("Failed to parse first vertex as usize");
    let v2 = parts
        .next()
        .expect("Missing second vertex")
        .parse::<usize>()
        .expect("Failed to parse second vertex as usize");
    if parts.next().is_some() {
        panic!("Too many parts in edge definition: {}", s);
    }
    Edge { v1, v2 }
}

fn parse_graph_file(filename: &str) -> Vec<Edge> {
    read_to_string(filename)
        .expect("Failed reading file")
        .lines()
        .map(string_to_edge)
        .collect()
}

fn random_permutation(size:usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..size).collect();
    let mut rng = rand::rng();
    indices.shuffle(&mut rng);
    indices
}

fn number_of_vertices(edges: &[Edge]) -> usize {
    let mut v = HashSet::new();
    for edge in edges {
        v.insert(edge.v1);
        v.insert(edge.v2);
    }
    v.len()
}

//Returns the number of cut edges in the graph after performing Karger's algorithm once.
fn karger_step(edges: &[Edge], number_of_vertices: usize) -> usize {

    //We introduce all necessary randomness here by randomly permuting the edges
    let indices: Vec<usize> = random_permutation(edges.len());
    
    let mut uf = QuickUnionUf::<UnionByRank>::new(number_of_vertices);
    let mut combined_vertices_counter = 0;

    for &i in &indices {
        //when we contracted n-2 edges we have only two vertices left with cut edges between them
        if combined_vertices_counter >= (number_of_vertices - 2) {
            break;
        }

        //We union the edges vertices only if it connects two different components/unions, since otherwise the edge has been already contracted
        let edge = &edges[i];
        if uf.find(edge.v1) != uf.find(edge.v2) {
            uf.union(edge.v1, edge.v2);
            combined_vertices_counter += 1;
        }
    }

    
    //calculate number of cut edges
    let mut cut_edges = 0;
    for &i in &indices {
        let edge = &edges[i];
        if uf.find(edge.v1) != uf.find(edge.v2) {
            cut_edges += 1;
        }
    }

    cut_edges
}

//Runs the Karger's algorithm number_of_repeats times and returns the minimum number of cut edges found.
fn karger_run_n(edges: &[Edge],number_of_vertices: usize, number_of_repeats: usize) -> usize {
    let mut min_cut_edges = usize::MAX;
    for _ in 0..number_of_repeats {
        let cut_edges = karger_step(edges, number_of_vertices);
        if cut_edges < min_cut_edges {
            min_cut_edges = cut_edges;
        }
    }

    min_cut_edges
}

//Runs the Karger's algorithm until the algorithm finds optimal solution
fn karger_run_until(edges: &[Edge], number_of_vertices: usize, opt: usize) -> usize {
    const REPETITION_LIMIT:usize = 1000;
    let mut repetitions = 0;
    while repetitions < REPETITION_LIMIT {
        let cut_edges = karger_step(edges, number_of_vertices);
        repetitions += 1;
        if cut_edges == opt {
            break;
        }
    }

    repetitions
}

fn analyze_graph(filename: &str) {
    const REPETITIONS_FOR_OPT: usize = 200;
    const REPETITIONS_FOR_AVG: usize = 500;

    let edges: Vec<Edge> = parse_graph_file(filename);
    let number_of_vertices = number_of_vertices(&edges);
    let number_of_edges = edges.len();

    // Time the optimal finding phase
    let start_opt = Instant::now();
    let opt = karger_run_n(&edges,number_of_vertices, REPETITIONS_FOR_OPT);
    let opt_duration = start_opt.elapsed();

    // Time the average repetitions calculation
    let start_avg = Instant::now();
    let mut sum_repetitions = 0;
    for _ in 0..REPETITIONS_FOR_AVG {
        sum_repetitions += karger_run_until(&edges,number_of_vertices, opt);
    }
    let avg_duration = start_avg.elapsed();
    
    let avg_repetitions = sum_repetitions as f64 / REPETITIONS_FOR_AVG as f64;
    
    println!("{}, ({},{}), {}, {:.2}, opt_time: {:.3}s, avg_time: {:.3}s",
             filename, 
             number_of_vertices, 
             number_of_edges, 
             opt, 
             avg_repetitions,
             opt_duration.as_secs_f64(),
             avg_duration.as_secs_f64());
}

fn main() {
    let graphs = [
        "./grafi/g01.graph",
        "./grafi/g02.graph", 
        "./grafi/g03.graph",
        "./grafi/g04.graph",
        "./grafi/g05.graph",
        "./grafi/g06.graph",
        "./grafi/g07.graph",
        "./grafi/g08.graph",
        "./grafi/g09.graph",
        "./grafi/g10.graph",
        "./grafi/g11.graph",
        "./grafi/g12.graph",
        "./grafi/g13.graph",
    ];

    println!("filename, (vertices,edges), min_cut, avg_repetitions, opt_time, avg_time");
    
    for filename in &graphs {
        if std::path::Path::new(filename).exists() {
            analyze_graph(filename);
        } else {
            println!("File not found: {}", filename);
        }
    }
}