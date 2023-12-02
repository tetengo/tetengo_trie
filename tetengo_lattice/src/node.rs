/*!
 * A node.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use anyhow::Result;

use crate::entry::{AnyValue, EntryView};
use crate::input::Input;

/**
 * A node error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum NodeError {
    /**
     * A BOS or EOS entry is not allowed.
     */
    #[error("BOS or EOS entry is not allowed")]
    BosOrEosEntryNotAllowed,
}

/**
 * A BOS (Beginning of Sequence) node.
 */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bos<'a> {
    preceding_edge_costs: &'a Vec<i32>,
}

/**
 * A EOS (Ending of Sequence) node.
 */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Eos<'a> {
    preceding_step: usize,
    preceding_edge_costs: &'a Vec<i32>,
    best_preceding_node: usize,
    path_cost: i32,
}
/**
 * A middle node.
 */
#[derive(Clone, Copy)]
pub struct Middle<'a> {
    key: &'a dyn Input,
    value: &'a dyn AnyValue,
    index_in_step: usize,
    preceding_step: usize,
    preceding_edge_costs: &'a Vec<i32>,
    best_preceding_node: usize,
    node_cost: i32,
    path_cost: i32,
}

impl Debug for Middle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Middle")
            .field("key", &"&'a dyn Input")
            .field("value", &"&'a dyn AnyValue")
            .field("index_in_step", &self.index_in_step)
            .field("preceding_step", &self.preceding_step)
            .field("preceding_edge_costs", &self.preceding_edge_costs)
            .field("best_preceding_node", &self.best_preceding_node)
            .field("node_cost", &self.node_cost)
            .field("path_cost", &self.path_cost)
            .finish()
    }
}

impl Eq for Middle<'_> {}

impl PartialEq for Middle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key.equal_to(other.key)
            && self.index_in_step == other.index_in_step
            && self.preceding_step == other.preceding_step
            && self.preceding_edge_costs == other.preceding_edge_costs
            && self.best_preceding_node == other.best_preceding_node
            && self.node_cost == other.node_cost
            && self.path_cost == other.path_cost
    }
}

/**
 * A node.
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node<'a> {
    /// The BOS (Beginning of Sequence) node.
    Bos(Bos<'a>),

    /// The EOS (Ending of Sequence) node.
    Eos(Eos<'a>),

    /// The middle node.
    Middle(Middle<'a>),
}

impl<'a> Node<'a> {
    /**
     * Creates a BOS (Beginning of Sequence).
     *
     * # Arguments
     * * preceding_edge_costs - Preceding edge costs.
     */
    pub const fn bos(preceding_edge_costs: &'a Vec<i32>) -> Self {
        Node::Bos(Bos {
            preceding_edge_costs,
        })
    }

    /**
     * Creates an EOS (Ending of Sequence).
     *
     * # Arguments
     * * preceding_step       - An index of a preceding step.
     * * preceding_edge_costs - Preceding edge costs.
     * * best_preceding_node  - An index of a best preceding node.
     * * path_cost            - A path cost.
     */
    pub const fn eos(
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Self {
        Node::Eos(Eos {
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            path_cost,
        })
    }

    /**
     * Creates a node.
     *
     * # Arguments
     * * key                  - A key.
     * * value                - A value.
     * * index_in_step        - An index in the step.
     * * preceding_step       - An index of a preceding step.
     * * preceding_edge_costs - Preceding edge costs.
     * * best_preceding_node  - An index of a best preceding node.
     * * node_cost            - A node cost.
     * * path_cost            - A path cost.
     */
    pub const fn new(
        key: &'a dyn Input,
        value: &'a dyn AnyValue,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        node_cost: i32,
        path_cost: i32,
    ) -> Self {
        Node::Middle(Middle {
            key,
            value,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            node_cost,
            path_cost,
        })
    }

    /**
     * Creates a node from a vocabulary entry.
     *
     * # Errors
     * * When `entry` is BOS or EOS.
     */
    pub fn from(
        entry: &'a EntryView<'a>,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Result<Self> {
        let Some(key) = entry.key() else {
            return Err(NodeError::BosOrEosEntryNotAllowed.into());
        };
        let Some(value) = entry.value() else {
            return Err(NodeError::BosOrEosEntryNotAllowed.into());
        };
        Ok(Node::Middle(Middle {
            key,
            value,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            node_cost: entry.cost(),
            path_cost,
        }))
    }

    /**
     * Returns the key.
     *
     * # Returns
     * The key.
     */
    pub const fn key(&self) -> Option<&dyn Input> {
        match self {
            Node::Bos(_) => EntryView::BosEos.key(),
            Node::Eos(_) => EntryView::BosEos.key(),
            Node::Middle(middle) => Some(middle.key),
        }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub const fn value(&self) -> Option<&dyn AnyValue> {
        match self {
            Node::Bos(_) => EntryView::BosEos.value(),
            Node::Eos(_) => EntryView::BosEos.value(),
            Node::Middle(middle) => Some(middle.value),
        }
    }

    /**
     * Returns the index in the step.
     *
     * # Returns
     * The index in the step.
     */
    pub const fn index_in_step(&self) -> usize {
        match self {
            Node::Bos(_) => 0,
            Node::Eos(_) => 0,
            Node::Middle(middle) => middle.index_in_step,
        }
    }

    /**
     * Returns the preceding step.
     *
     * # Returns
     * The preceding step.
     */
    pub const fn preceding_step(&self) -> usize {
        match self {
            Node::Bos(_) => usize::MAX,
            Node::Eos(eos) => eos.preceding_step,
            Node::Middle(middle) => middle.preceding_step,
        }
    }

    /**
     * Returns the preceding edge costs.
     *
     * # Returns
     * The preceding edge costs.
     */
    pub const fn preceding_edge_costs(&self) -> &Vec<i32> {
        match self {
            Node::Bos(bos) => bos.preceding_edge_costs,
            Node::Eos(eos) => eos.preceding_edge_costs,
            Node::Middle(middle) => middle.preceding_edge_costs,
        }
    }

    /**
     * Returns the index of the best preceding node.
     *
     * # Returns
     * The index of the best preceding node.
     */
    pub const fn best_preceding_node(&self) -> usize {
        match self {
            Node::Bos(_) => usize::MAX,
            Node::Eos(eos) => eos.best_preceding_node,
            Node::Middle(middle) => middle.best_preceding_node,
        }
    }

    /**
     * Returns the node cost.
     *
     * # Returns
     * The node cost.
     */
    pub const fn node_cost(&self) -> i32 {
        match self {
            Node::Bos(_) => EntryView::BosEos.cost(),
            Node::Eos(_) => EntryView::BosEos.cost(),
            Node::Middle(middle) => middle.node_cost,
        }
    }

    /**
     * Returns the path cost.
     *
     * # Returns
     * The path cost.
     */
    pub const fn path_cost(&self) -> i32 {
        match self {
            Node::Bos(_) => 0,
            Node::Eos(eos) => eos.path_cost,
            Node::Middle(middle) => middle.path_cost,
        }
    }

    /**
     * Returns `true` if this node is the BOS.
     *
     * # Returns
     * `true` if this node is the BOS.
     */
    pub const fn is_bos(&self) -> bool {
        matches!(self, Node::Bos(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::StringInput;

    use super::*;

    #[test]
    fn bos() {
        let preceding_edge_costs = Vec::new();
        let bos = Node::bos(&preceding_edge_costs);

        assert!(bos.key().is_none());
        assert!(bos.value().is_none());
        assert_eq!(bos.index_in_step(), 0);
        assert_eq!(bos.preceding_step(), usize::MAX);
        assert_eq!(bos.preceding_edge_costs(), &preceding_edge_costs);
        assert_eq!(bos.best_preceding_node(), usize::MAX);
        assert_eq!(bos.node_cost(), EntryView::BosEos.cost());
        assert_eq!(bos.path_cost(), 0);
    }

    #[test]
    fn eos() {
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let eos = Node::eos(1, &preceding_edge_costs, 5, 42);

        assert!(eos.key().is_none());
        assert!(eos.value().is_none());
        assert_eq!(eos.index_in_step(), 0);
        assert_eq!(eos.preceding_step(), 1);
        assert_eq!(eos.preceding_edge_costs(), &preceding_edge_costs);
        assert_eq!(eos.best_preceding_node(), 5);
        assert_eq!(eos.node_cost(), EntryView::BosEos.cost());
        assert_eq!(eos.path_cost(), 42);
    }

    #[test]
    fn new() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let _node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);
    }

    #[test]
    fn from() {
        {
            let entry_key = StringInput::new(String::from("mizuho"));
            let entry_value = 42;
            let entry = EntryView::new(&entry_key, &entry_value, 24);
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            let node = Node::from(&entry, 53, 1, &preceding_edge_costs, 5, 2424);

            let node = node.unwrap();
            assert_eq!(
                node.key()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap(),
                &entry_key
            );
            assert_eq!(
                node.value()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<i32>()
                    .unwrap(),
                &42
            );
            assert_eq!(node.index_in_step(), 53);
            assert_eq!(node.preceding_step(), 1);
            assert_eq!(node.preceding_edge_costs(), &preceding_edge_costs);
            assert_eq!(node.best_preceding_node(), 5);
            assert_eq!(node.node_cost(), 24);
            assert_eq!(node.path_cost(), 2424);
        }
        {
            let entry = EntryView::BosEos;
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            let node = Node::from(&entry, 53, 1, &preceding_edge_costs, 5, 2424);

            assert!(node.is_err());
        }
    }

    #[test]
    fn key() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(
            node.key()
                .unwrap()
                .as_any()
                .downcast_ref::<StringInput>()
                .unwrap()
                .value(),
            "mizuho"
        );
    }

    #[test]
    fn value() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(
            node.value()
                .unwrap()
                .as_any()
                .downcast_ref::<i32>()
                .unwrap(),
            &42
        );
    }

    #[test]
    fn index_in_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.index_in_step(), 53);
    }

    #[test]
    fn preceding_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.preceding_step(), 1);
    }

    #[test]
    fn preceding_edge_costs() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.preceding_edge_costs(), &preceding_edge_costs);
    }

    #[test]
    fn best_preceding_node() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.best_preceding_node(), 5);
    }

    #[test]
    fn node_cost() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.node_cost(), 24);
    }

    #[test]
    fn path_cost() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.path_cost(), 2424);
    }

    #[test]
    fn is_bos() {
        {
            let preceding_edge_costs_bos = Vec::new();
            assert!(Node::bos(&preceding_edge_costs_bos).is_bos());
        }
        {
            let preceding_edge_costs_eos = vec![3, 1, 4, 1, 5, 9, 2, 6];
            assert!(!Node::eos(1, &preceding_edge_costs_eos, 5, 42).is_bos());
        }
        {
            let key = StringInput::new(String::from("mizuho"));
            let value = 42;
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            assert!(!Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424).is_bos());
        }
    }

    #[test]
    fn eq() {
        let key = StringInput::new(String::from("mizuho"));

        let preceding_edge_costs_bos = Vec::new();
        let bos = Node::bos(&preceding_edge_costs_bos);

        let preceding_edge_costs_eos = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let eos = Node::eos(1, &preceding_edge_costs_eos, 5, 42);

        let value1 = 42;
        let preceding_edge_costs1 = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node1 = Node::new(&key, &value1, 53, 1, &preceding_edge_costs1, 5, 24, 2424);

        let value2 = 42;
        let preceding_edge_costs2 = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node2 = Node::new(&key, &value2, 53, 1, &preceding_edge_costs2, 5, 24, 2424);

        assert_eq!(bos, bos);
        assert_ne!(bos, eos);
        assert_ne!(bos, node1);
        assert_eq!(node1, node2);
    }
}
