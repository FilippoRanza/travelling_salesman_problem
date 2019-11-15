
extern crate ndarray;

use ndarray::prelude::{Array2, ArrayView, Ix1};

pub fn farthest_inserction(graph: &Array2<f64>, from: usize, to: usize) -> Option<Vec<usize>> {
    let mut output = Vec::new();
    output.push(from);
    let mut curr = from;
    let len = graph.shape()[0] - 1;
    for i in 0..len {
        let row = graph.row(curr);
        let next = if i == (len - 1) {
            select_next_max(row, &output, from)
        } else {
            select_next_max(row, &output, to)
        };

        match next {
            Some(n) => {
                output.push(n);
                curr = n;
            },
            None => return None
        }
    }

    Some(output)
}


fn select_next_max(col: ArrayView<f64, Ix1>, path: &Vec<usize>, end: usize) -> Option<usize> {
    let mut out = None;
    let mut max = -1.0;
    for (i, v) in col.iter().enumerate()
        .filter(|v| *v.1 != 0.0).filter(|v| !(path.contains(&v.0))).filter(|v| v.0 != end) {
        if *v > max {
            max = *v;
            out = Some(i);
        }
    }
    out
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

        let ans = farthest_inserction(&graph, 2, 0);
        match ans {
            Some(v) => {
                println!("{:?}", v);
                assert_eq!(3, v.len()); 
                assert_eq!(vec![2, 1, 0], v);
            }
            None => assert!(false)
        }
        
    }


    #[test]
    fn test_path_farthest() {
        let graph = array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                            [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                            [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                            [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                            [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                            [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                            [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]];
        
        let ans = farthest_inserction(&graph, 0, 2);
        match ans {
            Some(_) => assert!(false),
            None => assert!(true)
        }

        let ans = farthest_inserction(&graph, 5, 2);
        match ans {
            Some(v) => assert_eq!(vec![5, 3, 6, 4, 1, 0, 2], v),
            None => assert!(true)
        }
    }

}