// pub struct Node {
//     val: usize,
// }

// lets first model nsw
//
//
// this is basically a graph of nodes
// for now node will have a val i.e a number
//

use std::collections::HashMap;

pub struct NSW {
    start_node: usize,

    #[cfg(feature = "adj_mat")]
    nodes: Vec<u8>,
    #[cfg(feature = "adj_mat")]
    rows: usize,
    #[cfg(feature = "adj_mat")]
    cols: usize,

    #[cfg(feature = "adj_list")]
    nodes_adj: HashMap<usize, Vec<usize>>,
}

impl Default for NSW {
    fn default() -> Self {
        Self::new()
    }
}

impl NSW {
    pub fn new() -> Self {
        Self {
            start_node: 0,

            #[cfg(feature = "adj_mat")]
            nodes: vec![0; 1000 * 1000],
            #[cfg(feature = "adj_mat")]
            rows: 1000,
            #[cfg(feature = "adj_mat")]
            cols: 1000,

            #[cfg(feature = "adj_list")]
            nodes_adj: HashMap::new(),
        }
    }
}

impl NSW {
    pub fn insert(&mut self, key: usize, neighbours: Vec<usize>) {
        #[cfg(feature = "adj_mat")]
        {
            for n in neighbours.clone() {
                if key < self.rows && n < self.cols {
                    self.nodes[key * self.cols + n] = 1;
                    self.nodes[n * self.cols + key] = 1;
                }
            }
        }

        #[cfg(feature = "adj_list")]
        {
            self.nodes_adj
                .entry(key)
                .or_default()
                .extend(neighbours.clone());

            for n in neighbours.clone() {
                self.nodes_adj.entry(n).or_default().push(key);
            }
        }
    }
    pub fn delete(&mut self, key: usize) {
        #[cfg(feature = "adj_mat")]
        {
            if key >= self.rows {
                return;
            }

            for j in 0..self.cols {
                self.nodes[key * self.cols + j] = 0;
            }
            for i in 0..self.rows {
                self.nodes[i * self.cols + key] = 0;
            }
        }

        #[cfg(feature = "adj_list")]
        {
            if let Some(neigh) = self.nodes_adj.remove(&key) {
                for n in neigh {
                    if let Some(v) = self.nodes_adj.get_mut(&n) {
                        v.retain(|&x| x != key);
                    }
                }
            }
        }
    }
    pub fn neighbors(&self, key: usize) -> Vec<usize> {
        #[cfg(feature = "adj_mat")]
        {
            let mut res = Vec::new();
            if key >= self.rows {
                return res;
            }

            for j in 0..self.cols {
                if self.nodes[key * self.cols + j] != 0 {
                    res.push(j);
                }
            }
            return res;
        }

        #[cfg(feature = "adj_list")]
        {
            self.nodes_adj.get(&key).cloned().unwrap_or_default()
        }
    }

    pub fn search(&self, key: usize) -> Vec<usize> {
        let mut res = Vec::new();

        let mut curr = self.start_node;
        let mut curr_dist = distance(curr, key);

        loop {
            let mut next = None;
            let mut best = curr_dist;

            for n in self.neighbors(curr) {
                let d = distance(n, key);
                if d < best {
                    best = d;
                    next = Some(n);
                }
            }

            match next {
                Some(n) => {
                    curr = n;
                    curr_dist = best;
                }
                None => break,
            }
        }

        res.push(curr);
        res
    }

    // pub fn idx(&self, key: usize) -> Option<usize> {
    //     if key >= self.rows {
    //         return None;
    //     }
    //
    //     // find the row it could be present
    //
    //     let row_start = key * self.cols;
    //     Some(row_start)
    // }
}

fn distance(i: usize, x: usize) -> usize {
    i.abs_diff(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_line_graph() -> NSW {
        let mut g = NSW::new();

        // 0 - 1 - 2 - 3 - 4
        g.insert(0, vec![1]);
        g.insert(1, vec![2]);
        g.insert(2, vec![3]);
        g.insert(3, vec![4]);

        g
    }

    #[test]
    fn insert_creates_edges() {
        let g = make_line_graph();

        let n = g.neighbors(2);
        assert!(n.contains(&1));
        assert!(n.contains(&3));
    }

    #[test]
    fn delete_removes_node() {
        let mut g = make_line_graph();

        g.delete(2);

        assert!(!g.neighbors(1).contains(&2));
        assert!(!g.neighbors(3).contains(&2));
        assert!(g.neighbors(2).is_empty());
    }

    #[test]
    fn search_finds_nearest() {
        let g = make_line_graph();

        let res = g.search(4);
        assert_eq!(res[0], 4);
    }

    #[test]
    fn search_stops_when_no_better_neighbor() {
        let mut g = NSW::new();

        // 0 connected to 10 and 20
        g.insert(0, vec![10, 20]);

        let res = g.search(15);
        assert!(res[0] == 10 || res[0] == 20);
    }
}
