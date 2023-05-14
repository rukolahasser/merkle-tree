use crate::merkle_tree::MerkleTree;
use crate::node::Node;
use crate::utils::hash_data;

fn merkle_proof(tx: &str, merkle_tree: &MerkleTree, hash_function: &str) -> Vec<Node> {
    merkle_proof_helper(&merkle_tree.leaves, tx, &mut Vec::new(), hash_function)
}

fn merkle_proof_helper(txs: &[String], tx: &str, nodes: &mut Vec<Node>, hash_function: &str) -> Vec<Node> {
    if txs.len() > 1 {
        let mut lst = Vec::new();
        let mut index = String::new();

        for idx in (0..txs.len()).step_by(2) {
            lst.push(hash_data(&(format!("{}{}", txs[idx + 1], txs[idx])), hash_function));

            if txs[idx + 1] == tx {
                nodes.insert(0, Node::new("l".to_string(), txs[idx].clone()));
                index = hash_data(&(format!("{}{}", txs[idx], tx)), hash_function);
            } else if txs[idx] == tx {
                nodes.insert(0, Node::new("r".to_string(), txs[idx + 1].clone()));
                index = hash_data(&(format!("{}{}", tx, &txs[idx + 1])), hash_function);
            }
        }

        merkle_proof_helper(&lst, &index, nodes, hash_function)
    } else {
        nodes.clone()
    }
}

fn verify_proof(tx: &str, merkle_proof: &[Node], hash_function: &str) -> String {
    let mut return_value = tx.to_string();

    for data_value in merkle_proof.iter().rev() {
        match &data_value.direction[..] {
            "r" => {
                return_value = hash_data(&(format!("{}{}", return_value.clone(), &data_value.tx)), hash_function);
            }
            "l" => {
                return_value = hash_data(&(format!("{}{}", data_value.tx.clone(), &return_value)), hash_function);
            }
            _ => continue,
        }
    }

    return_value
}
