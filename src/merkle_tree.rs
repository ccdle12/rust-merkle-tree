/// Index position of the Node in tree.
pub type NodeId = usize;

/// Merkle Tree structure that holds a vector of Nodes.
pub struct MerkleTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> MerkleTree<T> {
    /// Constructor that builds a whole tree given a list of values.
    pub fn new(input: &Vec<T>) -> MerkleTree<&T> {
        // Create root.
        let root = Node::new();
        let nodes = vec![root];

        // Create merkle tree and add all leafs.
        let mut merkle_tree = MerkleTree { nodes };

        // TODO: use another iterator that doesn't expect a return?
        input.iter().map(|x| merkle_tree.add_leaf(x));
        merkle_tree.build_parent_nodes();

        merkle_tree
    }

    /// Constructor that will return a MerkleTree<T> with the root initialised
    /// to a zero value with no nodes and leafs.
    pub fn new_empty() -> MerkleTree<T> {
        let root = Node::new();
        let nodes = vec![root];

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

    fn build_parent_nodes(&mut self) {
        // 1. Iterate from root + 1,
        // 2. Create a node add it's left and right node[i] and node[i+1].
        // 3. Update node[i] and node[i+1] to store the parent node index.
        // 4. Push the Node.
        // 5. If node[i+1].right_sibling is None, find the left most node?
        let mut i: NodeId = 1;
        let mut row_count = 2;
        loop {
            let current_id = i + 1;
            let node_id: NodeId = self.nodes.len();

            // Create parent and assign child ids to the parent.
            let mut node = Node::<T>::new();
            node.child_left = Some(i);
            node.child_right = Some(current_id);

            // Check if we have reached the end of the row.
            match self.nodes[current_id].sibling_right {
                Some(_a) => {
                    // Increment the row count if there are more siblings.
                    row_count += 2;
                }
                None => {}
            }

            // Check that the parent should be the root.
            if row_count == 2 {
                // Assign the node id of parent to current nodes.
                self.nodes[i].parent = Some(0);
                self.nodes[current_id].parent = Some(0);
                self.nodes[0] = node;
            } else {
                self.nodes[i].parent = Some(node_id);
                self.nodes[current_id].parent = Some(node_id);
                self.nodes.push(node);
            }

            // Check if we have reached the end of the row.
            match self.nodes[current_id].sibling_right {
                Some(_a) => {}
                None => break,
            }

            // Increment the indexes.
            i += 1;
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

        let mut merkle_tree: MerkleTree<String> = MerkleTree::new_empty();

        let node_id_1 = merkle_tree.add_leaf("hello".to_string());
        let node_id_2 = merkle_tree.add_leaf("world".to_string());

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
            Some(a) => assert!(false),
            None => assert!(true),
        }

        // Left sibling should have the val "hello".
        let sibling_left = merkle_tree.get_sibling_left(node_id_2);

        let val = sibling_left.unwrap().value.as_ref().unwrap();
        assert_eq!(val, "hello");

        // Build the parent nodes.
        merkle_tree.build_parent_nodes();

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

    fn four_leaf_tree_1() {
        // Create four leaf nodes.
        // node_0 will be the parent to node_2, node_3.
        //              node_0
        //           /          \
        //          /            \
        //      node_5          node_6
        //      /    \         /     \
        //     /      \       /       \
        //  node_1   node_2  node_3  node_4

        let input = vec![1, 2, 3, 4];
        let merkle_tree = MerkleTree::new(&input);
    }
}
