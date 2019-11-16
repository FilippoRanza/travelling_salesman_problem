extern crate ndarray;
use ndarray::prelude::Array2;
use std::collections::HashMap;

pub fn dijkstra(graph: &Array2<f64>, from: usize, to: usize) -> Option<Vec<usize>> {
    let mut selected = nodes(graph, from);
    let (mut path, mut cost) = initialize(graph, from);

    let mut curr = from;
    while curr != to {
        let min = find_min_cost(&cost, &selected);
        match min {
            Some(node) => {
                selected[node] = true;
                update_cost(graph, node, &mut path, &mut cost, &selected);
                curr = node;
            }
            None => return None,
        };
    }

    Some(build_path(path, from, to))
}

fn nodes(graph: &Array2<f64>, begin: usize) -> Vec<bool> {
    let len = graph.shape()[0];
    (0..len).map(|x| x == begin).collect::<Vec<bool>>()
}

fn update_cost(
    graph: &Array2<f64>,
    node: usize,
    path: &mut HashMap<usize, usize>,
    cost: &mut HashMap<usize, f64>,
    selected: &Vec<bool>,
) {
    let row = graph.row(node);
    let node_cost = cost[&node];
    for (i, v) in row
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != 0.0)
        .filter(|(i, _)| !selected[*i])
    {
        match cost.get(&i) {
            Some(curr) => {
                if *curr > (*v + node_cost) {
                    cost.insert(i, *v + node_cost);
                    path.insert(i, node);
                }
            }
            None => {
                cost.insert(i, *v + node_cost);
                path.insert(i, node);
            }
        }
    }
}

fn find_min_cost(cost: &HashMap<usize, f64>, selected: &Vec<bool>) -> Option<usize> {
    let mut out = None;
    let mut curr = 0.0;
    for (i, v) in cost.iter().filter(|(k, _)| !selected[**k]) {
        if out == None || curr > *v {
            out = Some(*i);
            curr = *v;
        }
    }

    out
}

fn build_path(tree: HashMap<usize, usize>, from: usize, to: usize) -> Vec<usize> {
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

fn initialize(graph: &Array2<f64>, from: usize) -> (HashMap<usize, usize>, HashMap<usize, f64>) {
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
    fn test_selected() {
        let graph = get_graph();
        let from = 0;
        let selected = nodes(&graph, from);
        assert_eq!(selected.len(), 7);
        for (i, s) in selected.iter().enumerate() {
            if i == from {
                assert!(s);
            } else {
                assert!(!s);
            }
        }
    }

    #[test]
    fn test_find_min_cost() {
        let graph = get_graph();
        let from = 0;
        let selected = nodes(&graph, from);
        let (_, cost) = initialize(&graph, from);

        let ans = find_min_cost(&cost, &selected);
        match ans {
            Some(node) => assert_eq!(node, 2),
            None => assert!(false),
        };
    }

    #[test]
    fn test_update_cost() {
        let graph = get_graph();
        let from = 0;
        let mut selected = nodes(&graph, from);
        let (mut path, mut cost) = initialize(&graph, from);

        //first iteration
        let curr = 2;
        selected[curr] = true;
        update_cost(&graph, curr, &mut path, &mut cost, &selected);

        assert_eq!(path.len(), cost.len());
        assert_eq!(path.len(), 4);

        assert_eq!(path[&5], 2);
        assert_eq!(path[&3], 2);

        //(0,2) + (2, 5)
        assert_eq!(cost[&5], 6.0 + 18.0);
        //(0, 2) + (2, 3)
        assert_eq!(cost[&3], 6.0 + 4.0);
    }

    #[test]
    fn test_dijkstra1() {
        let graph = get_graph();
        let path = dijkstra(&graph, 2, 1);
        match path {
            Some(p) => assert_eq!(p, vec![2, 3, 1]),
            None => assert!(false),
        };
    }

    #[test]
    fn test_dijkstra2() {
        let graph = array![
            [0.0, 1.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 8.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 7.0, 10.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0],
            [0.0, 0.0, 0.0, 10.0, 0.0, 12.0, 0.0, 12.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 14.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        ];

        let path = dijkstra(&graph, 0, 8);
        match path {
            Some(p) => {
                let mut stat = false;
                println!("{:?}", p);
                let possible_results: [Vec<usize>; 3] = [
                    vec![0, 1, 4, 5, 6, 8],
                    vec![0, 1, 4, 7, 6, 8],
                    vec![0, 1, 4, 7, 8],
                ];
                for res in possible_results.iter() {
                    if p == *res {
                        stat = true;
                        break;
                    }
                }
                assert!(stat);
            }
            None => assert!(false),
        };
    }

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
        array![
            [0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
            [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
            [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
            [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
            [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
            [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
            [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]
        ]
    }
}
