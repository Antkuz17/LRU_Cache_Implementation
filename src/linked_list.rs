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


/// A doubly linked list backed by an arena (a Vec of slots).
pub struct DoublyLinkedList<K, V> {
    slots: Vec<Option<Node<K, V>>>,
    head: Option<usize>,
    tail: Option<usize>,
    free_list: Vec<usize>,
}

impl<K, V> DoublyLinkedList<K, V> {
    /// Creates a new, empty doubly linked list.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            head: None,
            tail: None,
            free_list: Vec::new(),
        }
    }

    /// Returns the number of nodes currently in the list.
    pub fn len(&self) -> usize {
        self.slots.len() - self.free_list.len()
    }

    /// Returns true if the list contains no nodes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }


    /// Inserts a node into the arena and returns its index
    fn allocate_node(&mut self, node: Node<K, V>) -> usize {
        if let Some(index) = self.free_list.pop() {
            self.slots[index] = Some(node);
            index
        } else {
            self.slots.push(Some(node));
            self.slots.len() - 1
        }
    }

    /// Adds a new node at the head of the list and returns its index
    pub fn push_front(&mut self, key: K, value: V) -> usize {
        let new_index = self.allocate_node(Node::new(key, value));

        match self.head {
            None => {
                self.head = Some(new_index);
                self.tail = Some(new_index);
            }
            Some(old_head_index) => {
                self.slots[new_index].as_mut().unwrap().next = Some(old_head_index);
                self.slots[old_head_index].as_mut().unwrap().prev = Some(new_index);
                self.head = Some(new_index);
            }
        }

        new_index
    }

    /// Removes the node at the tail of the list (least recently used).
    pub fn pop_back(&mut self) -> Option<(K, V)> {
        let tail_index = self.tail?;

        // Take the node out of its slot. The slot now holds `None`.
        let node = self.slots[tail_index].take().unwrap();

        match node.prev {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(new_tail_index) => {
                self.slots[new_tail_index].as_mut().unwrap().next = None;
                self.tail = Some(new_tail_index);
            }
        }

        self.free_list.push(tail_index);

        Some((node.key, node.value))
    }

    /// Moves the node at the given index to the head of the list
    pub fn move_to_front(&mut self, index: usize) {
        if self.head == Some(index) {
            return;
        }

        let (prev, next) = {
            let node = self.slots[index].as_ref().unwrap();
            (node.prev, node.next)
        };

        match prev {
            Some(prev_index) => {
                self.slots[prev_index].as_mut().unwrap().next = next;
            }
            None => {
                // No prev means this node was the head
            }
        }

        // Unlink from the next neighbor.
        match next {
            Some(next_index) => {
                self.slots[next_index].as_mut().unwrap().prev = prev;
            }
            None => {
                // No next means this node was the tail
                self.tail = prev;
            }
        }

        // Re-insert at the head.
        let old_head = self.head;
        let node = self.slots[index].as_mut().unwrap();
        node.prev = None;
        node.next = old_head;

        if let Some(old_head_index) = old_head {
            self.slots[old_head_index].as_mut().unwrap().prev = Some(index);
        }

        self.head = Some(index);
    }
}