use crate::utils::hash_data;

pub const SECURE_HASH_FUNCTIONS: [&str; 5] = ["sha1", "sha224", "sha256", "sha384", "sha512"];

pub struct HashLeaf {
    pub hash_function: String,
    pub left: String,
    pub right: String,
    pub data: String,
    pub height: i32,
}

impl HashLeaf {
    pub fn new(left: String, right: String, hash_function: String) -> HashLeaf {
        let data = HashLeaf::evaluate(&left, &right, &hash_function);
        let height = 1;

        HashLeaf {
            hash_function,
            left,
            right,
            data,
            height,
        }
    }

    fn evaluate(left: &str, right: &str, hash_function: &str) -> String {
        let concatenated_data = format!("{}{}", left, right);
        hash_data(&concatenated_data, hash_function)
    }
}

#[derive(Clone)]
pub struct HashNode {
    pub left: Option<Box<HashNode>>,
    pub right: Option<Box<HashNode>>,
    pub hash_function: String,
    pub data: String,
    pub height: i32,
}

impl HashNode {
    pub fn new(left: HashNode, right: HashNode, hash_function: String) -> HashNode {
        assert_eq!(left.hash_function, hash_function, "Hash functions incompatible");
        assert_eq!(right.hash_function, hash_function, "Hash functions incompatible");

        let data = HashNode::evaluate(&left, &right, &hash_function);
        let height = left.height + 1;

        HashNode {
            left: Some(Box::from(left)),
            right: Some(Box::from(right)),
            hash_function,
            data,
            height,
        }
    }

    pub fn build_from_leaves(left: HashLeaf, right: HashLeaf, hash_function: &str) -> HashNode {
        let concatenated_data = format!("{}{}", left.data, right.data);
        let data = hash_data(&concatenated_data, hash_function);

        HashNode {
            left: None,
            right: None,
            hash_function: hash_function.to_string(),
            data,
            height: 1,
        }
    }

    fn evaluate(left: &HashNode, right: &HashNode, hash_function: &str) -> String {
        assert_eq!(left.height, right.height, "Left and right branch not balanced");
        let concatenated_data = format!("{}{}", left.data, right.data);
        hash_data(&concatenated_data, hash_function)
    }
}

impl Default for HashNode {
    fn default() -> HashNode {
        HashNode {
            left: None,
            right: None,
            hash_function: "sha256".to_string(),
            data: "".to_string(),
            height: 0,
        }
    }
}
