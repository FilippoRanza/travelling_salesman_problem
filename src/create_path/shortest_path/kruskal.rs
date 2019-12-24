
extern crate ndarray;
use ndarray::prelude::Array2;
use std::cmp::Ordering::{Less, Greater};
use std::collections::HashSet;


pub fn kruskal(graph: &Array2<f64>) -> Vec<(usize, usize)> {
    let arcs = extract_arcs(&graph);
    //let mut selected: Vec<bool> = (0..arcs.len()).map(|_| false).collect();
    let mut out = Vec::new();
    let mut nodes = HashSet::new();

    for arc in arcs.iter() {

        if nodes.len() == (arcs.len() - 1) && connected(&out){
            break;
        }

        if !(nodes.contains(&arc.from)) || !(nodes.contains(&arc.to))  {
            out.push((arc.from, arc.to));
            nodes.insert(arc.from);
            nodes.insert(arc.to);
        }
    }
    
    out
}


fn extract_arcs(graph: &Array2<f64>) -> Vec<Arc> {
    let mut tmp: Vec<Arc> = graph.indexed_iter().filter(|(_, w)| **w != 0.0).map(|((i, j), w)| Arc::new(i, j, *w)).collect();
    tmp.sort_by(|a, b| if a.weight < b.weight {Less}else{Greater});
    tmp
}

fn connected(tree: &Vec<(usize, usize)>) -> bool {
    let mut reach = HashSet::new();
    for (f, t) in tree.iter() {
        if reach.len() == 0{
            reach.insert(f);
            reach.insert(t);
        }
        else {
            if reach.contains(&f) {
                reach.insert(t);
            }
            else if reach.contains(&t) {
                reach.insert(f);
            }
            else {
                return false;
            }
        }
    }

    true
}

struct Arc {
    from: usize,
    to: usize,
    weight: f64 
}


impl Arc {
    fn new(from: usize, to: usize, weight: f64) -> Arc {
        Arc{from, to, weight}
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use ndarray::array;
    

    #[test]
    fn test_kruskal() {
        let graph = get_graph();
        let ans = kruskal(&graph);
        assert_eq!(ans, vec![(0, 1), (1, 2), (1, 3)]);
    }


    #[test]
    fn test_kruscal_2() {
        let graph = array![[0.0, 3.0, 5.0, 10.0, 0.0],
                           [3.0, 0.0, 0.0, 13.0, 7.0],
                           [5.0, 0.0, 0.0, 9.0, 1.0],
                           [10.0, 13.0, 9.0, 0.0, 1.0],
                           [0.0, 7.0, 1.0, 1.0, 0.0]];

        let corr = vec![(2,4), (3, 4), (0, 1), (0, 2)];
        let ans = kruskal(&graph);
        assert_eq!(ans, corr);
    }


    #[test]
    fn test_extract_arcs() {
        let graph = get_graph();
        let arcs = extract_arcs(&graph);

        let corr_ord: Vec<f64> = vec![
        1.0,1.0,
        2.0,2.0,
        3.0,3.0,3.0,3.0,
        4.0,4.0,
        5.0,5.0];
        
        assert_eq!(corr_ord.len(), arcs.len());
        for (a, c) in arcs.iter().zip(corr_ord.iter()){ 
            assert_eq!(a.weight, *c);
        }
        
    }

    #[test]
    fn test_connected() {
        let connected_tree = vec![(2,4), (3, 4), (0, 1), (0, 2)];
        let not_connected = vec![(2, 4), (3, 4), (0, 1)];
        assert_eq!(connected(&connected_tree), true);
        assert_eq!(connected(&not_connected), false);
    }

    fn get_graph() -> Array2<f64> {
        array![[0.0, 1.0, 3.0, 5.0],
               [1.0, 0.0, 2.0, 3.0],
               [3.0, 2.0, 0.0, 4.0],
               [5.0, 3.0, 4.0, 0.0]]
    }

}