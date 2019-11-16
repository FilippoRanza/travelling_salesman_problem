
extern crate ndarray;
use std::collections::HashMap;
use ndarray::prelude::Array2;

pub fn build_path(tree: HashMap<usize, usize>, from: usize, to: usize) -> Vec<usize> {
    let mut out = Vec::new();

    let mut curr = to;
    while curr != from {
        let prev = tree.get(&curr).unwrap();
        out.push(curr);
        curr = *prev;
    }
    out.push(curr);

    out.reverse();
    out
}

pub fn initialize(graph: &Array2<f64>, from: usize) -> (HashMap<usize, usize>, HashMap<usize, f64>) {
    let mut cost = HashMap::new();
    let mut path = HashMap::new();

    let row = graph.row(from);
    for (i, v) in row.iter().enumerate().filter(|(_, v)| **v != 0.0) {
        path.insert(i, from);
        cost.insert(i, *v);
    }

    (path, cost)
}


#[cfg(test)]
mod test {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_build_path() {
        let mut path = HashMap::new();
        path.insert(3, 1);
        path.insert(1, 2);
        path.insert(2, 5);
        path.insert(5, 7);
        path.insert(7, 4);
        path.insert(4, 6);

        let ans = build_path(path, 6, 3);
        assert_eq!(ans.len(), 7);
        let correct = vec![6, 4, 7, 5, 2, 1, 3];
        assert_eq!(correct, ans);
    }


    #[test]
    fn test_initialize() {
        let graph = get_graph();
        let from = 0;
        let (path, cost) = initialize(&graph, from);
        assert_eq!(path.len(), cost.len());
        assert_eq!(path.len(), 2);

        assert_eq!(*path.get(&1).unwrap(), 0);
        assert_eq!(*path.get(&2).unwrap(), 0);

        assert_eq!(*cost.get(&1).unwrap(), 9.0); 
        assert_eq!(*cost.get(&2).unwrap(), 6.0);
    }


    fn get_graph() -> Array2<f64> {
        array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]]
    }


}