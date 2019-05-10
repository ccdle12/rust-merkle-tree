use crate::helper::calculate_tree_size;

/// Index position of the Node in tree.
pub type NodeId = usize;

/// Merkle Tree structure that holds a vector of Nodes.
pub struct MerkleTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> MerkleTree<T> {
    /// Constructor that builds a whole tree given a list of values.
    pub fn new(input: &Vec<T>) -> MerkleTree<&T> {
        // Calculate tree size.
        let tree_size = calculate_tree_size(input.len() as u64);

        // Create root.
        // let root = Node::new();
        let mut nodes = Vec::with_capacity(tree_size as usize);
        nodes.push(Node::new());

        // Create merkle tree and add all leafs.
        let mut merkle_tree = MerkleTree { nodes };

        // Add all the leafs.
        for x in input.iter() {
            merkle_tree.add_leaf(x);
        }
        merkle_tree.build_parents();

        merkle_tree
    }

    /// Constructor that will return a MerkleTree<T> with the root initialised
    /// to a zero value with no nodes and leafs.
    pub fn new_empty() -> MerkleTree<T> {
        // let root = Node::new();
        let nodes = vec![Node::new()];

        MerkleTree { nodes }
    }

    /// Adds a Leaf Node as leaf in the Merkle Tree.
    fn add_leaf(&mut self, val: T) -> NodeId {
        let index = self.nodes.len();
        let mut node = Node::new();

        // Set the value.
        node.value = Some(val);

        // Make sure we avoid assignment to the root node, this is reserved at 0.
        if index > 1 {
            // Assign the left sibling.
            node.sibling_left = Some(index - 1);

            // Assign the new node as the right sibling of the previous node.
            self.nodes[index - 1].sibling_right = Some(index);
        }

        self.nodes.push(node);
        index as NodeId
    }

    /// Returns the right sibling of a node according to the NodeId.
    pub fn get_sibling_right(&self, id: NodeId) -> Option<&Node<T>> {
        match self.nodes[id].sibling_right {
            Some(a) => Some(&self.nodes[a]),
            None => None,
        }
    }

    /// Returns the left sibling of a node according to the NodeId.
    pub fn get_sibling_left(&self, id: NodeId) -> Option<&Node<T>> {
        match self.nodes[id].sibling_left {
            Some(a) => Some(&self.nodes[a]),
            None => None,
        }
    }

    /// Returns the left child of a node according to the NodeId.
    pub fn get_child_left(&self, id: NodeId) -> Option<&Node<T>> {
        match self.nodes[id].child_left {
            Some(a) => Some(&self.nodes[a]),
            None => None,
        }
    }

    /// Returns the right child of a node according to the NodeId.
    pub fn get_child_right(&self, id: NodeId) -> Option<&Node<T>> {
        match self.nodes[id].child_right {
            Some(a) => Some(&self.nodes[a]),
            None => None,
        }
    }

    fn build_parents(&mut self) {
        let mut left_id: NodeId = 1;
        let capacity = self.nodes.capacity();

        loop {
            let right_id = left_id + 1;
            let parent_id: NodeId = self.nodes.len();

            // Check that we are at the head of a new row. The new row will be
            // a parent row, so each parent should not have a left or right sibling
            // assigned yet.
            match (
                self.nodes[left_id].sibling_left,
                self.nodes[left_id].sibling_right,
            ) {
                (None, None) => {
                    self.nodes[left_id].sibling_right = Some(right_id);
                    self.nodes[right_id].sibling_left = Some(left_id);
                    // if (right_id + 1) < self.nodes.len() {
                    // self.nodes[right_id].sibling_right = Some(right_id + 1);
                    // self.nodes[right_id + 1].sibling_left = Some(right_id);
                    // }
                }
                _ => {}
            }

            // if left_id != (self.nodes.len() - 1) {
            // self.nodes[left_id].sibling_right = Some(right_id);
            // self.nodes[right_id].sibling_left = Some(left_id);
            // }
            // TODO: check if the parent can reference a right sibling.
            // if (right_id + 1) < self.nodes.len() && self.nodes[right_id].sibling_right == None {
            //   self.nodes[right_id].sibling_right = Some(right_id + 1)
            // }

            // Create parent and assign child ids to the parent.
            let mut parent = Node::<T>::new();
            parent.child_left = Some(left_id);
            parent.child_right = Some(right_id);

            // Check if we have reached the end of the row.
            // match self.nodes[right_id].sibling_right {
            // Some(_a) => {
            // Increment the row count if there are more siblings.
            // row_count += 2;
            // }
            // None => {}
            // }
            //
            // Check that the parent should be the root.
            if right_id == capacity - 1 {
                // Assign the node id of parent to current nodes.
                self.nodes[left_id].parent = Some(0);
                self.nodes[right_id].parent = Some(0);
                self.nodes[0] = parent;
                break;
            }

            // Assign parents not at root.
            self.nodes[left_id].parent = Some(parent_id);
            self.nodes[right_id].parent = Some(parent_id);
            self.nodes.push(parent);

            // Reset the row count if we are at the end.
            // match self.nodes[right_id].sibling_right {
            // Some(_a) => {}
            // None => {
            // row_count = 2;
            // }
            // }

            // Increment the index.
            left_id += 2;
        }
    }
}

/// Node represents each node/leaf in the MerkleTree. This can be a parent
/// or child.
#[derive(Hash)]
pub struct Node<T> {
    parent: Option<NodeId>,
    sibling_left: Option<NodeId>,
    sibling_right: Option<NodeId>,
    child_left: Option<NodeId>,
    child_right: Option<NodeId>,
    value: Option<T>,
    hash: Option<u64>,
}

impl<T> Node<T> {
    pub fn new() -> Node<T> {
        Node {
            parent: None,
            sibling_left: None,
            sibling_right: None,
            child_left: None,
            child_right: None,
            value: None,
            hash: None,
        }
    }
}

#[cfg(test)]
mod merkle_tree {
    use super::*;

    #[test]
    fn simple_tree_1() {
        // Create three nodes.
        // node_0 will be the parent to node_2, node_3.
        //      node_0
        //      /    \
        //     /      \
        //  node_1   node_2

        // let mut merkle_tree: MerkleTree<String> = MerkleTree::new_empty();
        //

        // let node_id_1 = merkle_tree.add_leaf("hello".to_string());
        // let node_id_2 = merkle_tree.add_leaf("world".to_string());
        let input = vec!["hello", "world", "!"];
        let merkle_tree = MerkleTree::new(&input);
        assert_eq!(merkle_tree.nodes.len(), 3);

        // Node Id 1 should have no left sibling since it is the far left most
        // leaf.
        assert!(merkle_tree.nodes[node_id_1].sibling_left == None);
        assert!(merkle_tree.nodes[node_id_1].sibling_right != None);

        // Check the right sibling of node_1.
        assert_eq!(merkle_tree.nodes[node_id_1].sibling_right.unwrap(), 2);

        // Get the right sibling of node_id_1, and assert its value.
        let sibling_right = merkle_tree.get_sibling_right(node_id_1);

        // as_ref() Converts an Option<T> to a Option<&T> safely.
        let val = sibling_right.unwrap().value.as_ref().unwrap();
        assert_eq!(val, "world");

        // Assert the left and right sibling for node_id_2.
        // Right sibling should be None.
        match merkle_tree.get_sibling_right(node_id_2) {
            Some(_a) => assert!(false),
            None => assert!(true),
        }

        // Left sibling should have the val "hello".
        let sibling_left = merkle_tree.get_sibling_left(node_id_2);

        let val = sibling_left.unwrap().value.as_ref().unwrap();
        assert_eq!(val, "hello");

        // Build the parent nodes.
        merkle_tree.build_parents();

        // Check that both nodes are pointing to the root as their parent.
        assert!(merkle_tree.nodes[1].parent != None);
        assert!(merkle_tree.nodes[1].parent.unwrap() == 0);
        assert!(merkle_tree.nodes[0].child_left.unwrap() == 1);
        assert!(merkle_tree.nodes[0].child_right.unwrap() == 2);

        let child_left = merkle_tree.get_child_left(0);
        assert!(child_left.unwrap().value.as_ref().unwrap() == "hello");

        let child_right = merkle_tree.get_child_right(0);
        assert!(child_right.unwrap().value.as_ref().unwrap() == "world");
    }

    #[test]
    fn four_leaf_tree_1() {
        // Create four leaf nodes.
        //              node_0
        //           /          \
        //          /            \
        //      node_5          node_6
        //      /    \         /     \
        //     /      \       /       \
        //  node_1   node_2  node_3  node_4

        let input = vec![1, 2, 3, 4];
        let merkle_tree = MerkleTree::new(&input);

        // Assert the tree size.
        assert_eq!(merkle_tree.nodes.len(), 7);

        // Assert the leaf siblings.
        assert_eq!(merkle_tree.nodes[1].sibling_left, None);
        assert_eq!(merkle_tree.nodes[1].sibling_right.unwrap(), 2);
        assert_eq!(merkle_tree.nodes[2].sibling_left.unwrap(), 1);
        assert_eq!(merkle_tree.nodes[2].sibling_right.unwrap(), 3);
        assert_eq!(merkle_tree.nodes[3].sibling_left.unwrap(), 2);
        assert_eq!(merkle_tree.nodes[3].sibling_right.unwrap(), 4);
        assert_eq!(merkle_tree.nodes[4].sibling_right, None);
        assert_eq!(merkle_tree.nodes[4].sibling_left.unwrap(), 3);

        // Assert the parents.
        assert_eq!(merkle_tree.nodes[1].parent.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[2].parent.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[3].parent.unwrap(), 6);
        assert_eq!(merkle_tree.nodes[4].parent.unwrap(), 6);

        assert_eq!(merkle_tree.nodes[5].sibling_left, None);
        assert_eq!(merkle_tree.nodes[5].sibling_right.unwrap(), 6);
        assert_eq!(merkle_tree.nodes[5].child_left.unwrap(), 1);
        assert_eq!(merkle_tree.nodes[5].child_right.unwrap(), 2);
        assert_eq!(merkle_tree.nodes[5].parent.unwrap(), 0);

        assert_eq!(merkle_tree.nodes[6].sibling_left.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[6].sibling_right, None);
        assert_eq!(merkle_tree.nodes[6].child_left.unwrap(), 3);
        assert_eq!(merkle_tree.nodes[6].child_right.unwrap(), 4);
        assert_eq!(merkle_tree.nodes[6].parent.unwrap(), 0);

        assert_eq!(merkle_tree.nodes[0].child_left.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[0].child_right.unwrap(), 6);
    }

    #[test]
    fn eight_leaf_tree_1() {
        // Create eight leaf nodes.
        //                               node_0
        //                    /                        \
        //                   /                          \
        //                  /                            \
        //                 /                              \
        //              node_13                         node_14
        //           /          \                  /             \
        //          /            \                /               \
        //      node_9          node_10         node_11         node_12
        //      /    \         /     \         /      \       /      \
        //     /      \       /       \       /        \     /        \
        //  node_1   node_2  node_3  node_4  node_5  node_6 node_7  node_8

        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let merkle_tree = MerkleTree::new(&input);

        // Assert the leaf siblings.
        assert_eq!(merkle_tree.nodes[1].sibling_left, None);
        assert_eq!(merkle_tree.nodes[1].sibling_right.unwrap(), 2);
        assert_eq!(merkle_tree.nodes[2].sibling_left.unwrap(), 1);
        assert_eq!(merkle_tree.nodes[2].sibling_right.unwrap(), 3);
        assert_eq!(merkle_tree.nodes[3].sibling_left.unwrap(), 2);
        assert_eq!(merkle_tree.nodes[3].sibling_right.unwrap(), 4);
        assert_eq!(merkle_tree.nodes[4].sibling_right.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[5].sibling_left.unwrap(), 4);
        assert_eq!(merkle_tree.nodes[5].sibling_right.unwrap(), 6);
        assert_eq!(merkle_tree.nodes[6].sibling_left.unwrap(), 5);
        assert_eq!(merkle_tree.nodes[6].sibling_right.unwrap(), 7);
        assert_eq!(merkle_tree.nodes[7].sibling_left.unwrap(), 6);
        assert_eq!(merkle_tree.nodes[7].sibling_right.unwrap(), 8);
        assert_eq!(merkle_tree.nodes[8].sibling_left.unwrap(), 7);
        assert_eq!(merkle_tree.nodes[8].sibling_right, None);

        // Assert the parents.
        assert_eq!(merkle_tree.nodes[1].parent.unwrap(), 9);
        assert_eq!(merkle_tree.nodes[2].parent.unwrap(), 9);
        assert_eq!(merkle_tree.nodes[3].parent.unwrap(), 10);
        assert_eq!(merkle_tree.nodes[4].parent.unwrap(), 10);
        assert_eq!(merkle_tree.nodes[5].parent.unwrap(), 11);
        assert_eq!(merkle_tree.nodes[6].parent.unwrap(), 11);
        assert_eq!(merkle_tree.nodes[7].parent.unwrap(), 12);
        assert_eq!(merkle_tree.nodes[8].parent.unwrap(), 12);

        assert_eq!(merkle_tree.nodes[9].sibling_left, None);
        assert_eq!(merkle_tree.nodes[9].parent.unwrap(), 13);
        // assert_eq!(merkle_tree.nodes[9].sibling_right.unwrap(), 10);
        // assert_eq!(merkle_tree.nodes[10].sibling_left.unwrap(), 9);
        // assert_eq!(merkle_tree.nodes[10].sibling_right.unwrap(), 11);
        // assert_eq!(merkle_tree.nodes[11].sibling_left.unwrap(), 10);
        // assert_eq!(merkle_tree.nodes[11].sibling_right.unwrap(), 12);
        // assert_eq!(merkle_tree.nodes[5].sibling_right.unwrap(), 6);
        // assert_eq!(merkle_tree.nodes[5].child_left.unwrap(), 1);
        // assert_eq!(merkle_tree.nodes[5].child_right.unwrap(), 2);
        // assert_eq!(merkle_tree.nodes[5].parent.unwrap(), 0);
        //
        // assert_eq!(merkle_tree.nodes[6].sibling_left.unwrap(), 5);
        // assert_eq!(merkle_tree.nodes[6].sibling_right, None);
        // assert_eq!(merkle_tree.nodes[6].child_left.unwrap(), 3);
        // assert_eq!(merkle_tree.nodes[6].child_right.unwrap(), 4);
        // assert_eq!(merkle_tree.nodes[6].parent.unwrap(), 0);
        //
        // assert_eq!(merkle_tree.nodes[0].child_left.unwrap(), 5);
        // assert_eq!(merkle_tree.nodes[0].child_right.unwrap(), 6);
    }
}
