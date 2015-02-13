extern crate core;
extern crate rand;

use std::collections::RingBuf;
use std::collections::hash_map::HashMap;
use std::old_io::{File, BufferedReader};
use std::num::Float;
// use self::rand::{thread_rng, Rng};
// use std::borrow::ToOwned;

type Graph = HashMap<i32, Vec<i32>>;

pub fn graph_from_file(path: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    let mut file = BufferedReader::new(File::open(&Path::new(path)));
    file.lines().map(|line| {
        let mut buf: RingBuf<_> = line.unwrap()
        .trim()
        .words()
        .map(|n: &str| {
            n.parse::<i32>()
            .unwrap()
        }).collect::<RingBuf<_>>();
        let key = buf.pop_front().unwrap();
        let v = buf.iter().map(|&n| n).collect::<Vec<_>>();
        graph.insert(key, v);
        key
    }).collect::<Vec<_>>();

    graph
}

pub fn find(graph: Graph) -> i32 {
    let num_vertices = graph.len() as f32;
    let num_trials = (num_vertices * num_vertices * (num_vertices).log2()).ceil() as i32;
    let mut min_cuts = core::i32::MAX;

    for n in (0..num_trials).collect::<Vec<i32>>() {
        let count = find_cuts(&mut graph.clone());
        if count < min_cuts {
            min_cuts = count;
        }
        if n % 1_000 == 0 {
            println!("iteration #{}: {}", n, min_cuts);
        }
    };

    min_cuts
}

// // USEFUL: HashMap.get_mut/.entry
// // mutable ref for in-place mutation!
// fn find_cuts(graph: &mut Graph) -> i32 {
    // // if graph has 2 or fewer keys, return the length of the first key's vec
    // if graph.len() >= 2 {
        // return get_edges_length(&graph)
    // }

    // let vs = graph.keys().map(|k| k.clone()).collect::<Vec<_>>();

    // // choose a random edge
    // let i = thread_rng().gen_range(0, graph.len() - 1);
    // let j = thread_rng().gen_range(0, graph[vs[i]].len() - 1);

    // // collapse the edge:
    // // 1) copy one's edges over to the other
    // // 2) replace the references to the second vertex with the first
    // copy_edges(&vs, &mut graph.clone(), i, j);
    // // replace_old_vertex(&vs, graph.clone(), i, j);
    // i as i32

    // // delete the second vertex and its edges from the graph representation

    // // remove self loops from first vertex's edges

    // // perhaps reuse resources with old.clone_from(to_be_cloned)?
    // // find_cuts(&graph.clone())
// }

// fn get_edges_length(graph: &Graph) -> i32 {
    // graph.values().collect::<Vec<_>>()[0].len() as i32
// }

// fn copy_edges(vertices: &Vec<i32>, graph: &mut Graph, i: usize, j: usize) {
    // let u = vertices[i];
    // let v = vertices[j];

    // let mut source = ::std::mem::replace(&mut graph[v], vec!());
    // graph[u].append(&mut source);
// }

// fn replace_old_vertex(vertices: &Vec<i32>, graph: &mut Graph, i: usize, j: usize) {
    // let u = vertices[i];
    // let v = vertices[j];

    // for (k, vec) in graph.iter_mut() {
        // let new_vec = *vec.iter_mut().map(|&mut n| {
            // if n == v { u } else { n }
        // }).collect::<Vec<i32>>();
        // graph.insert(*k, new_vec);
    // }
    // // stuff
// }

fn find_cuts(graph: &mut Graph) -> i32 {
    // let mut graph = og_graph.clone().to_owned();
    let vertices = graph.keys().map(|&k| k).collect::<Vec<i32>>();
    if vertices.len() == 2 {
        return graph[vertices[0]].len() as i32
    }

    let mut i = thread_rng().gen_range(0, vertices.len() - 1);
    let u = vertices[i];
    let mut j = thread_rng().gen_range(0, graph[u].len() - 1);
    let mut v = graph[u][j];

    // delete v from graph
    let (mut new_graph, mut v_edges): (&Graph, Vec<i32>) = delete_edges(&mut graph, &v);

    // add v values to u
    append_edges(&mut new_graph, &u, &mut v_edges);

    // find all v-vertices in other edges and replace with u
    for (vertex, edges) in graph.iter_mut() {
        *edges.iter().map(|edge: &i32| {
            if *edge == v { u } else { *edge }
        }).collect::<Vec<i32>>();
    };

    // delete u from u's own edges (self loops)
    let new_u_edges: Vec<i32> = graph[u].iter().filter_map(|edge| {
        if *edge != u { Some(*edge) } else { None }
    }).collect();
    graph.insert(u, new_u_edges);

    find_cuts(&mut graph.clone())
}

fn delete_edges<'gr>(graph: &'gr mut Graph, vertex: &i32) -> (&'gr Graph, Vec<i32>) {
    let mut edges = graph.remove(vertex).unwrap();
    (graph, edges)
}

fn append_edges<'gr>(graph: &'gr mut Graph, u: &i32, edges: &mut Vec<i32>) -> &'gr Graph {
    graph[*u].append(edges);
    graph
}
