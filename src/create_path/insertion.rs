
extern crate ndarray;

use ndarray::prelude::Array2;

fn farthest_inserction(graph: &Array2<f64>, from: usize, to: usize, nodes: &[usize]) -> Vec<usize> {
    Vec::new()
}


#[cfg(test)]
mod test{

    use super::*;
    use ndarray::array;

    #[test]
    fn test_path_size() {

        let graph = array![[0.0, 5.5, 0.0],
                           [5.6, 0.0, 3.45],
                           [7.8, 12.0, 0.0]];

        let path = vec![2, 1, 0];
        let ans = farthest_inserction(&graph, 0, 1, &path);
        assert_eq!(path.len(), ans.len()); 
    }

}