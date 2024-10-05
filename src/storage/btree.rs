
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

const B: usize = 6; // B-tree order

#[derive(Debug)]
struct Node<K: Ord + Debug, V: Debug> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> Node<K, V> {
    fn new() -> Self {
        Node {
            keys: Vec::with_capacity(2 * B - 1),
            values: Vec::with_capacity(2 * B - 1),
            children: Vec::with_capacity(2 * B),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug)]
pub struct BTree<K: Ord + Debug, V: Debug> {
    root: Box<Node<K, V>>,
}

impl<K: Ord + Debug, V: Debug> BTree<K, V> {
    pub fn new() -> Self {
        BTree {
            root: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let root = mem::replace(&mut self.root, Box::new(Node::new()));
        let (new_root, _) = self.insert_non_full(root, key, value);
        self.root = new_root;
    }

    fn insert_non_full(
        &mut self,
        mut node: Box<Node<K, V>>,
        key: K,
        value: V,
    ) -> (Box<Node<K, V>>, Option<(K, V, Box<Node<K, V>>)>) {
        let mut i = node.keys.len();
        if node.is_leaf() {
            node.keys.push(key);
            node.values.push(value);
            node.keys.sort_unstable();
            node.values.sort_by(|_, _| node.keys.iter().cmp(node.keys.iter()));
            
            if node.keys.len() > 2 * B - 1 {
                let mid = B - 1;
                let right_node = Box::new(Node {
                    keys: node.keys.split_off(mid + 1),
                    values: node.values.split_off(mid + 1),
                    children: Vec::new(),
                });
                let median_key = node.keys.pop().unwrap();
                let median_value = node.values.pop().unwrap();
                (node, Some((median_key, median_value, right_node)))
            } else {
                (node, None)
            }
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            let (child, split_info) = self.insert_non_full(mem::replace(&mut node.children[i], Box::new(Node::new())), key, value);
            node.children[i] = child;

            if let Some((median_key, median_value, right_child)) = split_info {
                node.keys.insert(i, median_key);
                node.values.insert(i, median_value);
                node.children.insert(i + 1, right_child);

                if node.keys.len() > 2 * B - 1 {
                    let mid = B - 1;
                    let mut right_node = Box::new(Node {
                        keys: node.keys.split_off(mid + 1),
                        values: node.values.split_off(mid + 1),
                        children: node.children.split_off(mid + 1),
                    });
                    let median_key = node.keys.pop().unwrap();
                    let median_value = node.values.pop().unwrap();
                    (node, Some((median_key, median_value, right_node)))
                } else {
                    (node, None)
                }
            } else {
                (node, None)
            }
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        self.search_node(&self.root, key)
    }

    fn search_node(&self, node: &Node<K, V>, key: &K) -> Option<&V> {
        let mut i = 0;
        while i < node.keys.len() && key > &node.keys[i] {
            i += 1;
        }

        if i < node.keys.len() && key == &node.keys[i] {
            Some(&node.values[i])
        } else if node.is_leaf() {
            None
        } else {
            self.search_node(&node.children[i], key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut btree = BTree::new();
        btree.insert(3, "three");
        btree.insert(1, "one");
        btree.insert(5, "five");
        btree.insert(2, "two");
        btree.insert(4, "four");

        assert_eq!(btree.search(&1), Some(&"one"));
        assert_eq!(btree.search(&2), Some(&"two"));
        assert_eq!(btree.search(&3), Some(&"three"));
        assert_eq!(btree.search(&4), Some(&"four"));
        assert_eq!(btree.search(&5), Some(&"five"));
        assert_eq!(btree.search(&6), None);
    }
}
