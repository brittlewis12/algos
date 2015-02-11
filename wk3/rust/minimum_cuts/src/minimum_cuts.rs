type Graph = HashMap<_, Vec<_>>;

pub fn graph_from_file(path: &str) {
    let path = Path::new(path);
    // RingBuf here, or don't use a temporary data
    // store at all? Just convert directly into my Graph?
    // Basically, skip the collect step and insert directly
    // into the graph?
    let raw_input: RingBuf = IO.readlines(path)
        .map(|x| x.split())
        .map(|row| row.map(|x| x.to_i()))
        .collect();

    let mut graph = Graph::new();
    raw_input.each(|row| {
        graph.insert(row.shift, row)
    });

    graph
}

// pub fn find(graph: Graph) {
    // graph.freeze;
    // let num_vertices = graph.keys.len();
    // let num_trials = (num_vertices ** 2 * Math.log(num_vertices)).ceil;
    // let mut min_cuts;

    // num_trials.times(|n| {
        // count = find_cuts(graph.dup);
        // min_cuts = if min_cuts.is_nil() {
            // count
        // } else if count < min_cuts {
            // count
        // } else {
            // min_cuts
        // };

        // println!("iteration #{n}: #{min_cuts}" if n % 1_000 == 0);
    // });

    // min_cuts
// }

// fn find_cuts(graph: Graph) {
    // let vertices = graph.keys;
    // if vertices.len() == 2 {
        // return graph[vertices[0]].len()
    // }

    // let mut i = rand(vertices.count - 1);
    // let mut u = vertices[i];
    // let edges = graph[u];
    // let mut j = rand(edges.count - 1);
    // let v = edges[j];

    // // add v values to u
    // graph[u] = graph[u] + graph[v];

    // // find all v references in other edges and replace with u
    // graph.keys.each(|vertex| {
        // new_edges = graph[vertex].map(|edge| {
            // if edge == v { u } else { edge };
        // });
        // graph[vertex] = new_edges;
    // });

    // // delete v from graph
    // graph.delete(v);

    // // delete u from u's own edges (self loops)
    // graph[u] = graph[u].iter().filter(|edge| edge != u ).collect();

    // find_cuts(graph)
// }
