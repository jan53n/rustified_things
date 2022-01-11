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

    pub fn lookup<'a>(&'a self, needle: T) -> Option<&'a Self> {
        match (needle.cmp(&self.value), &self.right, &self.left) {
            (Ordering::Equal, _, _) => Some(&self),
            (Ordering::Greater, Some(r), _) => r.lookup(needle),
            (Ordering::Less, _, Some(l)) => l.lookup(needle),
            _ => None,
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

    #[allow(dead_code)]
    fn maximum(&self, node: Self) -> &Self {
        match node.right {
            Some(n) => self.maximum(*n),
            _ => self,
        }
    }

    #[allow(dead_code)]
    fn minimum(&self, node: Self) -> &Self {
        match node.left {
            Some(n) => self.maximum(*n),
            _ => self,
        }
    }
}

fn main() {
    //
}
