use crate::hash_data_structures::{HashLeaf, SECURE_HASH_FUNCTIONS};
use crate::utils::is_power_of_two;
use crate::hash_data_structures::HashNode;

pub struct MerkleTree {
    pub hash_function: String,
    pub leaves: Vec<String>,
    pub nodes: Vec<HashNode>,
    pub root: HashNode,
    pub height: i32,
    pub block_header: String,
}

impl MerkleTree {
    pub fn new(tx_list: Vec<String>, hash_function: &str) -> MerkleTree {
        let hash_function = hash_function.to_lowercase();
        assert!(!tx_list.is_empty(), "No transactions to be hashed");
        assert!(SECURE_HASH_FUNCTIONS.contains(&hash_function.as_str()), "{} is not a valid hash function", hash_function);

        let mut tree = MerkleTree {
            hash_function: hash_function.to_string(),
            leaves: tx_list,
            nodes: Vec::new(),
            root: HashNode::default(),
            height: 0,
            block_header: "".to_string(),
        };

        tree.evaluate();
        tree
    }

    pub fn add_tx(&mut self, tx: &[String]) {
        let mut tx_in = tx.to_vec();
        if let Some(first_tx) = tx_in.first() {
            if first_tx.starts_with('[') && first_tx.ends_with(']') {
                tx_in = tx_in[0]
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        self.leaves.extend_from_slice(&tx_in);
        self.reevaluate();
    }

    pub fn reset_tree(&mut self, hash_function: String) {
        self.hash_function = hash_function.to_lowercase();
        self.nodes.clear();
        self.height = 0;
        self.block_header.clear();
    }

    fn evaluate(&mut self) {
        let mut leaves = self.leaves.clone();
        let len_leaves = leaves.len();

        if !is_power_of_two(len_leaves) || len_leaves < 2 {
            let last_tx = leaves.last().unwrap().clone();
            while !is_power_of_two(leaves.len()) || leaves.len() < 2 {
                leaves.push(last_tx.clone());
            }
        }

        let mut leaf_nodes: Vec<HashLeaf> = Vec::new();

        for tx in (0..leaves.len()).step_by(2) {
            leaf_nodes.push(HashLeaf::new(
                leaves[tx].clone(),
                leaves[tx + 1].clone(),
                self.hash_function.clone(),
            ));
        }

        let mut nodes: Vec<HashNode> = Vec::new();

        while leaf_nodes.len() > 0 {
            let left = leaf_nodes.remove(0);
            let right = leaf_nodes.remove(0);
            let node = HashNode::build_from_leaves(left, right, &self.hash_function);
            nodes.push(node);
        }

        while nodes.len() > 2 {
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            let node = HashNode::new(left, right, self.hash_function.clone());
            nodes.push(node);
        }

        if nodes.len() == 1 {
            self.root = nodes[0].clone();
        } else if nodes.len() == 2 {
            self.root = HashNode::new(nodes[0].clone(), nodes[1].clone(), self.hash_function.clone());
        }

        self.height = self.root.height;
        self.block_header = self.root.data.clone();
    }

    fn reevaluate(&mut self) {
        let hash_function = self.hash_function.clone();
        self.reset_tree(hash_function);
        self.evaluate();
    }

    pub fn hash_function(&self) -> &str {
        self.hash_function.as_str()
    }

    pub fn set_hash_function(&mut self, value: &str) {
        let value = value.to_lowercase();
        assert!(
            SECURE_HASH_FUNCTIONS.contains(&value.as_str()),
            "{} is not a valid hash function", value
        );
        self.hash_function = value.to_string();
        self.reset_tree(self.hash_function.clone());
    }
}