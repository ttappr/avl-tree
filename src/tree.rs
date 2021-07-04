
use std::cmp::Ordering;
use std::ops::Deref;
use std::ops::DerefMut;

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
    fn balance(&self) -> isize
    {
        self.left.height() - self.right.height()
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

                    if bf >= 2 {
                        if bf_l > 0 {
                            self.rotate_left_left();
                        } 
                        else if bf_l < 0 {
                            self.rotate_left_right();
                        }
                    }
                    else if bf <= -2 {
                        if bf_r < 0 {
                            self.rotate_right_right();
                        } 
                        else if bf_r > 0 {
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
                    
                    if bf >= 2 {
                        if bf_l >= 0 {
                            self.rotate_left_left();
                        }
                        else if bf_l < 0 {
                            self.rotate_left_right();
                        }
                    }
                    else if bf <= -2 {
                        if bf_r <= 0 {
                            self.rotate_right_right();
                        }
                        else if bf_r > 0 {
                            self.rotate_right_left();
                        }
                    }
                }
            }
        }
        ret
    }
    pub fn find_nth(&self, index: usize) -> Option<(&K, &V)>
    {
        use Ordering::*;
        let mut ret  = None;
        let     node = &*self;
        let     wt   = node.weight as usize;
        let     wt_l = node.left.weight;

        match index.cmp(&wt) {
            Less => { 
                ret = self.left.find_nth(index);
            },
            Greater => { 
                ret = self.right.find_nth(index - wt);
            },
            Equal => {
                let node = &*self; 
                ret = Some((&node.key, &node.value));
            },
        }
        ret
    }
    fn height(&self) -> isize
    {
        match self {
            Filled(node) => Self::floor_log2(node.weight * 2 - 1),
            Empty => 0,
        }
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
    fn balance(&self) -> isize 
    {
        match self {
            Filled(node) => node.left.height() - self.right.height(),
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
    fn rotate_left_left(&mut self)
    {
        let mut n = self.take();
        let mut t = n.left.take();
        n.left    = t.right.take();
        t.right   = n;
        *self     = t;
        self.update_weights(2);
    }
    fn rotate_right_right(&mut self)
    {
        let mut n = self.take();
        let mut t = n.right.take();
        n.right   = t.left.take();
        t.left    = n;
        *self     = t;
        self.update_weights(2);
    }
    fn rotate_right_left(&mut self)
    {
        let mut n  = self.take();
        let mut t2 = n.right.left.take();
        let mut t1 = n.right.take();
        n.right    = t2.left.take();
        t1.left    = t2.right.take();
        t2.left    = n;
        t2.right   = t1;
        *self      = t2;
        self.update_weights(2);
    }
    fn rotate_left_right(&mut self)
    {
        let mut n  = self.take();
        let mut t2 = n.left.right.take();
        let mut t1 = n.left.take();
        n.left     = t2.right.take();
        t1.right   = t2.left.take();
        t2.right   = n;
        t2.left    = t1;
        *self      = t2;
        self.update_weights(2);
    } 
    fn update_weights(&mut self, depth: isize) -> isize
    {
        if depth >= 0 {
            let mut wt_l = 0;
            let mut wt_r = 0;
            if self.left.is_filled() {
                wt_l = self.left.update_weights(depth - 1);
            }
            if self.right.is_filled() {
                wt_r = self.right.update_weights(depth - 1);
            }
            self.weight = 1 + wt_l + wt_r;
        }
        self.weight
    }
    fn predecessor(&self) -> (K, V)
    {
        let mut t = self;
        while let Filled(_) = t.right {
            t = &t.right;
        }
        (t.key.clone(), t.value.clone())
    }
    fn successor(&self) -> (K, V)
    {
        let mut t = self;
        while let Filled(_) = t.left {
            t = &t.left;
        }
        (t.key.clone(), t.value.clone())
    }
}
impl<K, V> Default for Tree<K, V>
{
    fn default() -> Self { 
        Empty
    }
}

impl<K, V> Deref for Tree<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Target = Node<K, V>;

    fn deref(&self) -> &Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference an Empty Tree."),
        }
    }
}

impl<K, V> DerefMut for Tree<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference an Empty Tree."),
        }
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
        }/*
        for ch in "qwertyuiopafsa;".chars() {
            tree.remove(&ch);
        }*/
        println!("{:#?}", tree);
    }
}
