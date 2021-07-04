
#![allow(dead_code)]

use std::cmp::Ordering;

use crate::node::*;

use Tree::*;

#[derive(Debug)]
pub enum Tree<K, V> 
{
    Empty,
    Filled(Box<Node<K, V>>),
}
impl<K, V> Tree<K, V>
where 
    K: Ord,
    V: Clone,
{
    pub fn new(key: K, value: V) -> Self
    {
        Filled(Box::new(Node::new(key, value)))
    }
    pub fn is_empty(&self) -> bool 
    {
        match self {
            Empty => true,
            _ => false,
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        use Ordering::*;
        let mut ret = None;
        match self {
            Empty => {
                *self = Tree::new(key, value);
            },
            Filled(node) => {
                match key.cmp(node.key()) {
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

                    if bf == 2 && bf_l == 1 {
                        self.rotate_left_left();
                    }
                    else if bf == -2 && bf_r == -1 {
                        self.rotate_right_right();
                    }
                    else if bf == -2 && bf_r == 1 {
                        self.rotate_right_left();
                    }
                    else if bf == 2 && bf_l == -1 {
                        self.rotate_left_right();
                    }
                }
            },
        }
        ret
    }

    pub (crate) fn height(&self) -> isize 
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
    fn weight(&self) -> isize
    {
        match self {
            Filled(node) => node.weight(),
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
            _ => panic!("Cannot borrow mutable reference from an empty tree!"),
        }
    }
    fn right_mut(&mut self) -> &mut Tree<K, V>
    {
        match self {
            Filled(node) => &mut node.right,
            _ => panic!("Cannot borrow mutable reference from an empty tree!"),
        }
    }
    fn rotate_left_left(&mut self)
    {
        let mut p  = self.take();
        let mut tp = p.left_mut().take();
        *p.left_mut()   = tp.right_mut().take();
        *tp.right_mut() = p;
        *self = tp.take();
    }
    fn rotate_right_right(&mut self)
    {
        let mut p  = self.take();
        let mut tp = p.right_mut().take();
        *p.right_mut() = tp.left_mut().take();
        *tp.left_mut() = p;
        *self = tp.take();
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
    }
    fn rotate_left_right(&mut self) -> Tree<K, V>
    {
        Empty
    } 
}
impl<K, V> Default for Tree<K, V>
{
    fn default() -> Self { 
        Empty
    }
}
