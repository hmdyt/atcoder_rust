use petgraph::{algo, graph};
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut g = graph::DiGraph::<usize, usize>::new();
    let mut sunuke_node_index = Vec::new();

    // add node and edge (sunuke)
    for i in 0..n {
        sunuke_node_index.push(g.add_node(i));
    }
    for i in 0..(n - 1) {
        g.add_edge(sunuke_node_index[i], sunuke_node_index[i + 1], s[i]);
    }
    g.add_edge(sunuke_node_index[n - 1], sunuke_node_index[0], s[n - 1]);

    // add node and edge (takahashi)
    let takahashi_node_index = g.add_node(200000 + 1);
    for i in 0..n {
        g.add_edge(takahashi_node_index, sunuke_node_index[i], t[i]);
    }

    let path = algo::dijkstra(&g, takahashi_node_index, None, |e| *e.weight());

    for i in 0..n {
        println!("{}", path.get(&graph::NodeIndex::new(i)).unwrap());
    }
}
