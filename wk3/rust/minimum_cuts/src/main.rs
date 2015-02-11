mod minimum_cuts;

fn main() {
    let graph = minimum_cuts::graph_from_file("KargerMinCut.txt");

    println!("{?}", graph);
    // println!("{}", minimum_cuts::find(graph));
}
