struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node. 
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Check if the tree is a Binary Search Tree
    pub fn is_binary(&self) -> bool {
        self.rec_is_binary(Some(0), u32::MIN, u32::MAX)
    }

    /// A private recursive function that computes whether the
    /// subtree rooted at `node_id` is a Binary Search Tree
    fn rec_is_binary(&self, node_id: Option<usize>, left_limit: u32, right_limit: u32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if !(left_limit <= node.key && node.key <= right_limit) {
                return false;
            }

            let res_left = self.rec_is_binary(node.id_left, left_limit, node.key);
            let res_right = self.rec_is_binary(node.id_right, node.key, right_limit);

            return res_left && res_right;
        }

        true
    }

    // Return the sum of the maximum simple path connecting two leaves
    pub fn max_path(&self) -> u32 {
        self.rec_max_path(Some(0)).0
    }

    fn rec_max_path(&self, node_id: Option<usize>) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if node.id_left.is_some() && node.id_right.is_some() {
                let res_left = self.rec_max_path(node.id_left);
                let res_right = self.rec_max_path(node.id_right);

                let max_node_to_leaf = node.key + res_left.1.max(res_right.1);
                let res = (node.key + res_left.1 + res_right.1).max(res_left.0).max(res_right.0);

                return (res, max_node_to_leaf);
            } else if node.id_left.is_some() {

                let res_left = self.rec_max_path(node.id_left);
                let res = (node.key + res_left.1).max(res_left.0);

                return (res, res_left.1 + node.key);
            } else if node.id_right.is_some() {

                let res_right = self.rec_max_path(node.id_right);
                let res = (node.key + res_right.1).max(res_right.0);

                return (res, res_right.1 + node.key);
            } else {
                return (node.key, node.key);
            }
        }

        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1

        assert_eq!(tree.is_binary(), true);
        assert_eq!(tree.max_path(), 15);

        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.is_binary(), true);
        assert_eq!(tree.max_path(), 37);
        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4
    
        assert_eq!(tree.sum(), 64);
        assert_eq!(tree.max_path(), 64);
    }
}
