use std::vec;

pub type Graph = Vec<(i32, i32)>;

// recursively visit all the vertices possible
fn dfs(root: i32, graph: &Graph, discovered: &mut Vec<i32>) {
    discovered.push(root);

    for adj in adjacent(root, &graph) {
        if !discovered.contains(&adj) {
            dfs(adj, graph, discovered);
        }
    }
}

// find adjacent vertices of a root
fn adjacent(root: i32, graph: &Graph) -> Vec<i32> {
    let mut adj = vec![];

    for (r, l) in graph {
        if r == &root {
            adj.push(*l);
        } else if l == &root {
            adj.push(*r);
        }
    }

    adj
}

// public interface
pub fn traverse(root: i32, graph: Graph) -> Vec<i32> {
    let mut discovered: Vec<i32> = vec![];
    dfs(root, &graph, &mut discovered);
    discovered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let graph = vec![(1, 2), (1, 3), (1, 5), (2, 4), (2, 6), (3, 7), (5, 6)];
        assert_eq!(traverse(1, graph), vec![1, 2, 4, 6, 5, 3, 7]);
    }
}
