
extern crate ndarray;
use std::collections::HashMap;
use ndarray::prelude::Array2;


pub fn dijkstra(graph: &Array2<f64>, from: usize, to:usize) -> Option<Vec<usize>> {
    let mut curr = from;
    let mut path = HashMap::new();
    let mut dist = Distances::new(graph, from);

    let mut selected = Vec::new();
    selected.push(from);
   

    while curr != to {
        let missing = missing_nodes(graph, &selected);
        match dist.min_value(&missing) {
            Some(next) => {
               selected.push(next);
               path.insert(next, curr);
               dist.add_distances(graph, curr, next, &missing);
               curr = next;
            },
            None => return None
        }
    }
    Some(build_path(path, from, to))
}

fn missing_nodes(graph: &Array2<f64>, selected: &Vec<usize>) -> Vec<usize> {
    let len = graph.shape()[0];
    (0..len).filter(|x| ! selected.contains(x)).collect::<Vec<usize>>()
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

struct Distances {
    dist: HashMap<usize, f64>,
    start: usize,
}

impl Distances {
    fn new(graph: &Array2<f64>, origin: usize) -> Distances {
        let mut dist = HashMap::new();
        let row = graph.row(origin);
        for (i, v) in row.iter().enumerate().filter(|(_, v)| **v != 0.0) {
            dist.insert(i, *v);
        }
        Distances{dist: dist, start: origin}
    } 

    fn get_dist(&self, node: usize) -> Option<f64> {
        if node == self.start { 
            Some(0.0)
        }
        else {
            match self.dist.get(&node) {
                Some(v) => Some(*v),
                None => None
            }
        }
    }

    fn add_distances(&mut self, graph: &Array2<f64>, curr: usize, next: usize, missing: &Vec<usize>) {
        let row = graph.row(next);
        let curr = self.get_dist(curr).unwrap();
        for m in missing.iter() {
            let tmp = row.get(*m).unwrap();
            if *tmp != 0.0 {
                self.set_dist(*m, tmp + curr);
            }
        }
    }

    fn set_dist(&mut self, node: usize, value: f64) {
        match self.dist.get_mut(&node) {
            Some(v) => *v = value,
            None => {
                self.dist.insert(node, value);
            }
        };
    }

    fn min_value(&self, nodes: &Vec<usize>) -> Option<usize> {
        let mut out = None;
        let mut curr = 0.0;
        for n in nodes.iter() {
            let tmp = self.get_dist(*n);
            match tmp {
                Some(v) => {
                    if out == None || curr > v {
                        curr = v;
                        out = Some(*n); 
                    }
                },
                None => {}
            }
        }

        out
    }

}





#[cfg(test)]
mod test{

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
    fn test_distances() {
        let graph = array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                            [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                            [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                            [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                            [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                            [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                            [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]];

        let mut dist = Distances::new(&graph, 0);
        assert_eq!(dist.get_dist(0).unwrap(), 0.0);
        assert_eq!(dist.get_dist(1).unwrap(), 9.0);
        assert_eq!(dist.get_dist(2).unwrap(), 6.0);

        for i in 3..7 {
            match dist.get_dist(i as usize) {
                Some(_) => assert!(false),
                None => assert!(true)
            };
        }

        dist.set_dist(1, 4.5);
        assert_eq!(dist.get_dist(1).unwrap(), 4.5);
        assert_eq!(dist.get_dist(2).unwrap(), 6.0);

        let min = dist.min_value(&vec![1, 2]);
        match min {
            Some(v) => assert_eq!(v, 1),
            None => assert!(false)
        };

        let min = dist.min_value(&vec![5, 6]);
        match min {
            Some(_) => assert!(false),
            None => assert!(true)
        };

        dist.add_distances(&graph, 0, 1, &vec![3, 4]);
        assert_eq!(dist.get_dist(1).unwrap(), 4.5);
        assert_eq!(dist.get_dist(2).unwrap(), 6.0);
        assert_eq!(dist.get_dist(3).unwrap(), 1.0);
        assert_eq!(dist.get_dist(4).unwrap(), 20.0);

        for i in 5..7 {
            match dist.get_dist(i as usize) {
                Some(_) => assert!(false),
                None => assert!(true)
            };
        }

    }
    
    #[test]
    fn test_missing() {
        let graph = array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                           [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                           [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                           [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                           [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                           [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                           [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]];

        let missing = missing_nodes(&graph, &vec![0, 2, 4]);
        assert_eq!(missing, vec![1, 3, 5, 6]);
    }

    #[test]
    fn test_dijkstra1() {
        let graph = array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                            [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                            [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                            [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                            [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                            [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                            [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]];
        let path = dijkstra(&graph, 2, 1);
        match path {
            Some(p) => assert_eq!(p, vec![2, 3, 1]),
            None => assert!(false)
        };           
    }
    
    #[test]
    fn test_dijkstra2() {
        let graph = array![[0.0, 1.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                           [0.0, 0.0, 8.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0, 7.0, 10.0, 0.0, 0.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0],
                           [0.0, 0.0, 0.0, 10.0, 0.0, 12.0, 0.0, 12.0, 0.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 14.0],
                           [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]];

        let path = dijkstra(&graph, 0, 8);
        match path {
            Some(p) => {
                let mut stat = false;
                println!("{:?}", p);
                let possible_results: [Vec<usize>; 3] = [vec![0, 1, 4, 5, 6, 8], vec![0, 1, 4, 7, 6, 8], vec![0, 1, 4, 7, 8]];
                for res in possible_results.iter() {
                    if p == *res {
                        stat = true;
                        break;
                    }
                }
                assert!(stat);
            }
            None => assert!(false)
        };  
    }

}