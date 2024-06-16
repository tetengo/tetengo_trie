/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::Ordering;

use crate::node::Node;

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator {
    current: i32,
    max: i32,
}

impl NBestIterator {
    /**
     * Creates an N-best lattice path iterator.
     *
     * # Arguments
     * * `max` - The maximum number of paths.
     */
    pub fn new(max: i32) -> Self {
        NBestIterator { current: 0, max }
    }
}

impl Iterator for NBestIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

#[derive(Eq)]
struct _Cap<'a> {
    tail_path: Vec<Node<'a>>,
    tail_path_cost: i32,
    whole_path_cost: i32,
}

impl<'a> _Cap<'a> {
    fn _new(tail_path: Vec<Node<'a>>, tail_path_cost: i32, whole_path_cost: i32) -> Self {
        _Cap {
            tail_path,
            tail_path_cost,
            whole_path_cost,
        }
    }

    fn _tail_path(&self) -> &[Node<'a>] {
        self.tail_path.as_slice()
    }

    fn _tail_path_cost(&self) -> i32 {
        self.tail_path_cost
    }

    fn _whole_path_cost(&self) -> i32 {
        self.whole_path_cost
    }
}

impl Ord for _Cap<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.whole_path_cost.cmp(&other.whole_path_cost)
    }
}

impl PartialEq for _Cap<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.whole_path_cost == other.whole_path_cost
    }
}

impl PartialOrd for _Cap<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.whole_path_cost.cmp(&other.whole_path_cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let iter = NBestIterator::new(3);
        let mut values = Vec::new();
        iter.for_each(|e| values.push(e));
        assert_eq!(values, vec![1, 2, 3]);
    }

    mod cap {
        use std::rc::Rc;

        use super::*;

        #[test]
        fn new() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let _cap = _Cap::_new(nodes, 24, 42);
        }

        #[test]
        fn ord() {
            let preceding_edge_costs1 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node1 = Node::eos(1, preceding_edge_costs1, 5, 42);
            let nodes1 = vec![node1];
            let cap1 = _Cap::_new(nodes1, 24, 42);

            let preceding_edge_costs2 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node2 = Node::eos(1, preceding_edge_costs2, 5, 42);
            let nodes2 = vec![node2];
            let cap2 = _Cap::_new(nodes2, 24, 42);

            let preceding_edge_costs3 = Rc::new(vec![2, 7, 1, 8, 2, 8]);
            let node3 = Node::eos(2, preceding_edge_costs3, 3, 31);
            let nodes3 = vec![node3];
            let cap3 = _Cap::_new(nodes3, 12, 4242);

            assert!(cap1 == cap2);
            assert!(cap1 < cap3);
        }

        #[test]
        fn tail_path() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs.clone(), 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._tail_path().len(), 1);
            assert_eq!(
                cap._tail_path()[0].preceding_edge_costs(),
                preceding_edge_costs.as_slice()
            );
        }

        #[test]
        fn tail_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._tail_path_cost(), 24);
        }

        #[test]
        fn whole_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._whole_path_cost(), 42);
        }
    }
}
