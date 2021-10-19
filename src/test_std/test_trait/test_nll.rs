use std::collections::{BinaryHeap, HashMap};

// #[test]
fn main() {
    fn dfs(graph: &mut HashMap<&str, BinaryHeap<&str>>) {
        // let x = graph.get_mut(from).unwrap_or(&mut BinaryHeap::new());
        // while let Some(to) = x.pop() {
        //     dfs(graph, stack, to);
        // }
        while let Some(to) = graph.get_mut("").unwrap_or(&mut BinaryHeap::new()).pop() {
            dfs(graph);
        }
    }
}
