#[derive(Clone)]
pub struct Node {
    pub direction: String,
    pub tx: String,
}

impl Node {
    pub fn new(direction: String, tx: String) -> Node {
        Node {
            direction,
            tx,
        }
    }
}
