
extern crate ndarray;
use std::collections::HashMap;
use ndarray::prelude::Array2;



pub fn shortest_path(graph: &Array2<f64>, from: usize, to: usize) -> Option<Vec<usize>> {

    let mut output = Vec::new();
    let mut curr = from;

    let mut dist = initialize_distances(graph, from);
    
    while curr != to {

    }

    Some(output)
}


fn initialize_distances(graph: &Array2<f64>, origin: usize) -> Distances {
    let len = graph.shape()[0];
    let mut dist = Distances::new(len, origin);
    let row = graph.row(origin);
    for (i, v) in row.iter().enumerate().filter(|(_, v)|  **v != 0.0) {
        println!("{:?} {:?}", i, v);
        dist.fix_distance(i, *v);
    }

    dist
}

struct Distances {
    dist: HashMap<usize, Value>,
}

impl Distances {
    fn new(len: usize, origin: usize) -> Distances {
        let mut out = HashMap::new();
        for i in 0..len {
            if i != origin {
                out.insert(i, Value::new());     
            }
        } 
        Distances{ dist: out}
    }

    fn find_min(&mut self) -> Option<usize> {
        let mut min_d = -1.0;
        let mut index = None;
        for (k, v) in self.dist.iter().filter(|(_, v)| !v.fix) {
            match v.val {
                Some(d) => {
                    if min_d == -1.0 || d < min_d {
                        min_d = d;
                        index = Some(*k);
                    }
                }
                None => {}
            }
        }

        match index {
            Some(k) => {
                self.dist.get_mut(&k).unwrap().fix = true;
            }
            None => {} 
        };

        index
    }

    fn set_dist(&mut self, node: usize, dist: f64)  {
        let value = self.dist.get_mut(&node).unwrap();
        match value.val {
            Some(d) => {
                if value.fix {
                    panic!("The algorithm found a shorter path after a fix!!!");
                }
                if d > dist {
                    value.val = Some(dist);
                }
            }
            None => {
                value.val = Some(dist);
            }
        }
    }

    fn fix_distance(&mut self, node: usize, dist: f64) {
        let value = self.dist.get_mut(&node).unwrap();
        value.fix = true;
        value.val = Some(dist);
    }

    fn get_value(&self, k: usize) -> Option<f64> {
        self.dist.get(&k).unwrap().val
    }

}

struct Value {
    val: Option<f64>,
    fix: bool
}

impl Value {
    fn new() -> Value {
        Value{val: None, fix: false}
    }
}



#[cfg(test)]
mod test{

    use super::*;
    use ndarray::array;

    #[test]
    fn test_distancs() {
        let mut dist = Distances::new(10, 1);
        dist.set_dist(5, 5.6);
        dist.set_dist(4, 3.4);

        assert_eq!(dist.find_min().unwrap(), 4);
        assert_eq!(dist.find_min().unwrap(), 5);

        match dist.find_min() {
            Some(_) => assert!(false),
            None => assert!(true)
        };

        dist.set_dist(3, 4.5);
        dist.set_dist(8, 1.0);
        dist.set_dist(3, 0.5);

        assert_eq!(dist.get_value(3).unwrap(), 0.5);

        assert_eq!(dist.find_min().unwrap(), 3);
        assert_eq!(dist.find_min().unwrap(), 8);

        match dist.find_min() {
            Some(_) => assert!(false),
            None => assert!(true)
        };

    
        dist.fix_distance(2, 1.2);
        match dist.find_min() {
            Some(_) => assert!(false),
            None => assert!(true)
        };

        let val = dist.dist.get(&(2 as usize)).unwrap();
        assert_eq!(val.val, Some(1.2));
        assert!(val.fix)

    }

    #[should_panic]
    #[test]
    fn test_add_error() {
        let mut dist = Distances::new(10, 1);
        dist.set_dist(5, 5.6);
        dist.set_dist(4, 3.4);

        assert_eq!(dist.find_min().unwrap(), 4);
        assert_eq!(dist.find_min().unwrap(), 5);

        match dist.find_min() {
            Some(_) => assert!(false),
            None => assert!(true)
        }
        dist.set_dist(4, 1.4);

    }



    #[test]
    fn test_initialize_dist() {
        let graph = array![[0.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0],
                           [9.0, 0.0, 11.0, 1.0, 20.0, 0.0, 0.0],
                           [6.0, 11.0, 0.0, 4.0, 0.0, 18.0, 0.0],
                           [0.0, 1.0, 2.0, 0.0, 13.0, 28.0, 15.0],
                           [0.0, 20.0, 0.0, 13.0, 0.0, 0.0, 3.0],
                           [0.0, 0.0, 18.0, 28.0, 0.0, 0.0, 25.0],
                           [0.0, 0.0, 0.0, 15.0, 3.0, 25.0, 0.0]];

        let dist = initialize_distances(&graph, 0);
        assert_eq!(dist.dist.len(), 6);
        assert_eq!(dist.get_value(1).unwrap(), 9.0);
        assert_eq!(dist.get_value(2).unwrap(), 6.0);

        for i in 3..7 {
            let a = dist.get_value(i as usize);
        }


    }

}