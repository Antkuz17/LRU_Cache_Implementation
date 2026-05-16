/// A single node in the doubly linked list.
/// Nodes are stored in an arena (a Vec) and refer to each other by index
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl<K, V> Node<K, V> {
    /// Creates a new node with no connections to other nodes.
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}