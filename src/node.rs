#![allow(dead_code)]

use std::cmp::Ordering;

use crate::tree::*;
use Tree::*;

#[derive(Debug)]
pub struct Node<K, V>
{
    pub (crate) key     : K,
    pub (crate) value   : V,
    pub (crate) weight  : isize,
    pub (crate) left    : Tree<K, V>,
    pub (crate) right   : Tree<K, V>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
    V: Clone,
{
    pub (crate) fn new(key: K, value: V) -> Self
    {
        Node { key, value, weight: 1, left: Empty, right: Empty }
    }
    pub (crate) fn key(&self) -> &K
    {
        &self.key
    }
    pub (crate) fn weight(&self) -> isize
    {
        self.weight
    }
    pub (crate) fn height(&self) -> isize
    {
        Self::floor_log2(self.weight)
    }
    pub (crate) fn balance(&self) -> isize
    {
        self.left.height() - self.right.height()
    }
    fn floor_log2(mut n: isize) -> isize
    {
        let mut c = 0;
        while n != 0 {
            n >>= 1;
            c  += 1;
        }
        c - 1
    }
}
