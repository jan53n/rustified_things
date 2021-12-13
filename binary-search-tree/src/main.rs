use std::cmp::Ordering;

#[derive(Debug)]
pub struct BSTNode<T: Eq + Ord> {
    value: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T: Eq + Ord> BSTNode<T> {
    pub fn new(t: T) -> Self {
        Self {
            value: t,
            left: None,
            right: None,
        }
    }

    pub fn search(self, needle: T) -> bool {
        match (needle.cmp(&self.value), self.right, self.left) {
            (Ordering::Equal, _, _) => true,
            (Ordering::Greater, Some(r), _) => r.search(needle),
            (Ordering::Less, _, Some(l)) => l.search(needle),
            _ => false,
        }
    }

    pub fn insert(&mut self, needle: T) {
        match (needle.cmp(&self.value), &mut self.right, &mut self.left) {
            (Ordering::Equal, _, _) => {}
            (Ordering::Greater, Some(r), _) => r.insert(needle),
            (Ordering::Greater, None, _) => self.right = Some(Box::new(BSTNode::new(needle))),
            (Ordering::Less, _, Some(l)) => l.insert(needle),
            (Ordering::Less, _, None) => self.left = Some(Box::new(BSTNode::new(needle))),
        };
    }

    pub fn delete(self, _t: T) {
        // code
    }
}

fn main() {
    let mut node = BSTNode::new(123);
    node.insert(1);
    node.insert(2);
    node.insert(3);
    node.insert(4);
    node.insert(5);
    node.insert(6);
    node.insert(7);
    node.insert(8);
    node.insert(9);
    node.insert(10);
    node.insert(11);
    node.insert(12);
    node.insert(13333);

    println!("{:?}", node);
}