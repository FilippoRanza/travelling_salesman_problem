
extern crate ndarray;
use ndarray::prelude::Array2;
use std::cmp::Ordering::{Less, Greater};


pub fn kruskal(graph: &Array2<f64>) -> Vec<(usize, usize)> {
    let arcs = extract_arcs(&graph);
    let mut selected: Vec<bool> = (0..arcs.len()).map(|_| false).collect();
    let mut out = Vec::new();
    let mut missing = arcs.len() - 1;

    for arc in arcs.iter() {

        if missing == 0 {
            break;
        }

        if (!selected[arc.from]) || (!selected[arc.to]) {
            out.push((arc.from, arc.to));
            selected[arc.from] = true;
            selected[arc.to] = true;
            missing -= 1;
        }

    }
    
    out
}


fn extract_arcs(graph: &Array2<f64>) -> Vec<Arc> {
    let mut tmp: Vec<Arc> = graph.indexed_iter().filter(|(_, w)| **w != 0.0).map(|((i, j), w)| Arc::new(i, j, *w)).collect();
    tmp.sort_by(|a, b| if a.weight < b.weight {Less}else{Greater});
    tmp
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

    fn get_graph() -> Array2<f64> {
        array![[0.0, 1.0, 3.0, 5.0],
               [1.0, 0.0, 2.0, 3.0],
               [3.0, 2.0, 0.0, 4.0],
               [5.0, 3.0, 4.0, 0.0]]
    }

}