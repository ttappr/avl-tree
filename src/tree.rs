
use std::cmp::Ordering;

use Tree::*;

#[derive(Debug)]
pub struct Node<K, V>
{
    key     : K,
    value   : V,
    weight  : isize,
    left    : Tree<K, V>,
    right   : Tree<K, V>,
}

impl<K, V> Node<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn new(key: K, value: V) -> Self
    {
        Node { key, value, weight: 1, left: Empty, right: Empty }
    }
    fn height(&self) -> isize
    {
        // Adjust to get the ceiling.
        Self::floor_log2(self.weight * 2 - 1)
    }
    fn balance(&self) -> isize
    {
        self.left.height() - self.right.height()
    }
    fn floor_log2(mut n: isize) -> isize
    {
        if n != 0 {
            let mut c = 0;
            while n != 0 {
                n >>= 1;
                c  += 1;
            }
            c - 1
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub enum Tree<K, V> 
{
    Empty,
    Filled(Box<Node<K, V>>),
}
impl<K, V> Tree<K, V>
where 
    K: Clone + Ord,
    V: Clone,
{
    pub fn new_and_insert(key: K, value: V) -> Self
    {
        Filled(Box::new(Node::new(key, value)))
    }
    pub fn new() -> Self
    {
        Empty
    }
    pub fn is_empty(&self) -> bool 
    {
        matches!(self, Empty)
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        use Ordering::*;
        let mut ret = None;
        match self {
            Empty => {
                *self = Tree::new_and_insert(key, value);
            },
            Filled(node) => {
                match key.cmp(&node.key) {
                    Less => {
                        ret = node.left.insert(key, value);
                    },
                    Greater => {
                        ret = node.right.insert(key, value);
                    },
                    Equal => {
                        ret = Some(node.value.clone());
                        node.value = value;
                    },
                }
                // If ret.is_none() == true, tree changed size.
                if ret.is_none() {
                    node.weight += 1;

                    let bf   = node.balance();
                    let bf_r = node.right.balance();
                    let bf_l = node.left.balance();

                    if bf == 2 {
                        if bf_l == 1 {
                            self.rotate_left_left();
                        } 
                        else if bf_l == -1 {
                            self.rotate_left_right();
                        }
                    }
                    else if bf == -2 {
                        if bf_r == -1 {
                            self.rotate_right_right();
                        } 
                        else if bf_r == 1 {
                            self.rotate_right_left();
                        }
                    }
                }
            },
        }
        ret
    }
    pub fn remove(&mut self, key: &K) -> Option<V>
    {
        let mut ret = None;

        if let Filled(node) = self {
            if key == &node.key {
                ret = Some(node.value.clone());
                if node.left.is_empty() && node.right.is_empty() {
                    *self = Empty;
                }
                else if node.left.is_filled() {
                    let (k, v)   = node.left.predecessor();
                    node.key     = k.clone();
                    node.value   = v;
                    node.weight -= 1;
                    node.left.remove(&k);
                } 
                else {
                    let (k, v)   = node.right.successor();
                    node.key     = k.clone();
                    node.value   = v;
                    node.weight -= 1;
                    node.right.remove(&k);
                }                
            } else {
                if key < &node.key {
                    ret = node.left.remove(key);
                }
                else if key > &node.key {
                    ret = node.right.remove(key);
                }
                if ret.is_some() {
                    node.weight -= 1;
                    
                    let bf   = node.balance();
                    let bf_r = node.right.balance();
                    let bf_l = node.left.balance();
                    
                    if bf == 2 {
                        if bf_l == 1 || bf_l == 0 {
                            self.rotate_left_left();
                        }
                        else if bf_l == -1 {
                            self.rotate_left_right();
                        }
                    }
                    else if bf == -2 {
                        if bf_r == -1 || bf_r == 0 {
                            self.rotate_right_right();
                        }
                        else if bf_r == 1 {
                            self.rotate_right_left();
                        }
                    }
                }
            }
        }
        ret
    }
    fn height(&self) -> isize 
    {
        match self {
            Filled(node) => node.height(),
            Empty => 0,
        }
    }
    fn take(&mut self) -> Tree<K, V>
    {
        std::mem::take(self)
    }
    fn is_filled(&self) -> bool
    {
        !self.is_empty()
    }
    fn key(&self) -> &K
    {
        match self {
            Filled(node) => &node.key,
            Empty => panic!("Node is Empty."),
        }
    }
    fn value(&self) -> &V
    {
        match self {
            Filled(node) => &node.value,
            Empty => panic!("Node is Empty."),
        }
    }
    fn weight(&self) -> isize
    {
        match self {
            Filled(node) => node.weight,
            Empty => 0,
        }
    }
    fn balance(&self) -> isize
    {
        match self {
            Filled(node) => node.balance(),
            Empty => 0,
        }
    }
    fn left(&self) -> &Tree<K, V>
    {
        match self {
            Filled(node) => &node.left,
            Empty => &Empty,
        }
    }
    fn right(&self) -> &Tree<K, V>
    {
        match self {
            Filled(node) => &node.right,
            Empty => &Empty,
        }
    }
    fn left_mut(&mut self) -> &mut Tree<K, V>
    {
        match self {
            Filled(node) => &mut node.left,
            _ => panic!("Node is Empty."),
        }
    }
    fn right_mut(&mut self) -> &mut Tree<K, V>
    {
        match self {
            Filled(node) => &mut node.right,
            _ => panic!("Node is Empty."),
        }
    }
    fn node_mut(&mut self) -> &mut Node<K, V>
    {
        match self {
            Filled(node) => node,
            _ => panic!("Node is Empty."),
        }
    }
    fn node(&self) -> &Node<K, V>
    {
        match self {
            Filled(node) => node,
            _ => panic!("Node is Empty."),
        }
    }
    fn rotate_left_left(&mut self)
    {
        let mut p  = self.take();
        let mut tp = p.left_mut().take();
        *p.left_mut()   = tp.right_mut().take();
        *tp.right_mut() = p;
        *self = tp.take();
        self.update_weights(2);
    }
    fn rotate_right_right(&mut self)
    {
        let mut p  = self.take();
        let mut tp = p.right_mut().take();
        *p.right_mut() = tp.left_mut().take();
        *tp.left_mut() = p;
        *self = tp.take();
        self.update_weights(2);
    }
    fn rotate_right_left(&mut self)
    {
        let mut p   = self.take();
        let mut tp2 = p.right_mut().left_mut().take();
        let mut tp  = p.right_mut().take();
        *p.right_mut()   = tp2.left_mut().take();
        *tp.left_mut()   = tp2.right_mut().take();
        *tp2.left_mut()  = p.take();
        *tp2.right_mut() = tp.take();
        *self = tp2.take();
        self.update_weights(2);
    }
    fn rotate_left_right(&mut self)
    {
        let mut p   = self.take();
        let mut tp2 = p.left_mut().right_mut().take();
        let mut tp  = p.left_mut().take();
        *p.left_mut()    = tp2.right_mut().take();
        *tp.right_mut()  = tp2.left_mut().take();
        *tp2.right_mut() = p.take();
        *tp2.left_mut()  = tp.take();
        *self = tp2.take();
        self.update_weights(2);
    } 
    fn update_weights(&mut self, depth: isize) -> isize
    {
        if depth >= 0 {
            let mut wt_l = 0;
            let mut wt_r = 0;
            if self.left().is_filled() {
                wt_l = self.left_mut().update_weights(depth - 1);
            }
            if self.right().is_filled() {
                wt_r = self.right_mut().update_weights(depth - 1);
            }
            self.node_mut().weight = 1 + wt_l + wt_r;
        }
        self.node().weight
    }
    fn predecessor(&self) -> (K, V)
    {
        let mut t = self;
        while t.right().is_filled() {
            t = t.right();
        }
        (t.key().clone(), t.value().clone())
    }
    fn successor(&self) -> (K, V)
    {
        let mut t = self;
        while t.left().is_filled() {
            t = t.left();
        }
        (t.key().clone(), t.value().clone())
    }
}
impl<K, V> Default for Tree<K, V>
{
    fn default() -> Self { 
        Empty
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::*;
    #[test]
    fn it_works() {
        let mut tree = Tree::new();
        for ch in "qwertyuiopasdfghjklzxcvbnmklasjfal;jasjfsa;".chars() {
            tree.insert(ch, 5);
        }
        for ch in "qwertyuiopafsa;".chars() {
            tree.remove(&ch);
        }
        println!("{:#?}", tree);
    }
}
