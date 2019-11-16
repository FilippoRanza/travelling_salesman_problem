

extern crate ndarray;
use ndarray::prelude::Array2;



pub fn bellman_ford(graph: &Array2<f64>, from: usize, to: usize) -> Option<Vec<usize>> {
    
    let (mut cost, mut path) = initialize(graph, from);
    let len = graph.shape()[0] - 1;
    for _ in 0..len {
        for ((i, j), w) in graph.indexed_iter().filter(|(_, w)| **w != 0.0) {
            match cost[j] {
                Some(cj) => {
                    match cost[i] {
                        Some(ci) => {
                            if ci + w < cj {
                                cost[j] = Some(ci + w);
                                path[j] = Some(i);
                            }
                        },
                        None => {}
                    }
                },
                None => {
                    match cost[i] {
                        Some(ci) => {
                            cost[j] = Some(ci + w);
                            path[j] = Some(i);
                        },
                        None => {}
                    }
                }
            }
        }
    }
    println!("{:?}", path);
    build_path(path, from, to)
}




fn initialize(graph: &Array2<f64>, from: usize) -> (Vec<Option<f64>>, Vec<Option<usize>>){
    let len = graph.shape()[0];
    let mut cost = vec![None; len];
    let path = vec![None; len];
    cost[from] = Some(0.0);

    (cost, path)
}

fn build_path(path: Vec<Option<usize>>, from: usize, to: usize) -> Option<Vec<usize>> {
    let mut out = Vec::new();
    out.push(to);

    let mut curr = to;

    while curr != from {
        match path[curr] {
            Some(prev) => {
                out.push(prev);
                curr = prev;
            },
            None => return None
        };
    }
    
    out.reverse();
    Some(out)
}

#[cfg(test)]
mod test {

    use super::*;
    use ndarray::array;

    #[test]
    fn test_initialize() {
        let graph = get_graph();
        let start = 0;
        let (cost, path) = initialize(&graph, start);
        assert_eq!(cost.len(), path.len());
        assert_eq!(cost.len(), 7);

        assert_eq!(cost[start].unwrap(), 0.0);

    }

    #[test]
    fn test_bellman_ford() {
        let graph = get_graph();
        let path = bellman_ford(&graph, 2, 1);
        match path {
            Some(p) => assert_eq!(p, vec![2, 3, 1]),
            None => assert!(false)
        };   
    }


    #[test]
    fn test_bellman_ford2() {
        let graph = array![[0.0, -3.0, 0.0, 0.0, 0.0],
                           [0.0, 0.0, 1.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0, 1.0, 0.0],
                           [0.0, 0.0, 0.0, 0.0, 1.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0]];
        let path = bellman_ford(&graph, 0, 4);
        match path {
            Some(p) => assert_eq!(p, vec![0, 1, 2, 3, 4]),
            None => assert!(false)
        }
                        
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