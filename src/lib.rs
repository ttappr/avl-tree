
//! An AVL Tree implementation in Rust that provides a dictionary-like 
//! interface. Keys are used as the data for comparison when inserting into the
//! tree. The keys can be associated with values, which can be retrieved using
//! the keys with `O(log n)` time-complexity. Insertions, deletions, lookups,
//! etc. are all `O(log n)` operations.
//! 


use std::cmp::Ordering;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;

use Tree::*;

/// Represents a node in the binary tree, that holds a key and value and 
/// slots for the right and left sub-trees.
/// 
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
    /// Private constructor for `Node`. Takes a key and value.
    /// 
    fn new(key: K, value: V) -> Self
    {
        Node { key, value, weight: 1, left: Empty, right: Empty }
    }

    /// Returns a value indicating the difference in height between its left
    /// and right sub-trees (`left.height() - right.height()`).
    /// 
    fn balance(&self) -> isize
    {
        self.left.height() - self.right.height()
    }
}

/// Represents the whole AVL binary tree externally. Internally, it's also the
/// the implementation for the sub-trees that fill each node's right and left
/// slot. It also implements `Deref` and `DerefMut` to render a reference to the
/// `Node` it holds in it's `Filled` variant. A `Tree` is either occupied by
/// a `Node` as indicated by the `Filled` variant, or it's `Empty`.
/// 
/// # Variants
/// 
/// * `Empty`   - Doesn't hold a node.
/// * `Filled`  - Holds a `Node`, which in turn may hold other `Tree`s.
/// 
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
    /// Creates a new `Tree` populated with a `Node` holding the given key and
    /// value.
    /// 
    pub fn new_with_insert(key: K, value: V) -> Self
    {
        Filled(Box::new(Node::new(key, value)))
    }

    /// Creates a new empty `Tree` - the `Tree::Empty` variant.
    /// 
    pub fn new() -> Self
    {
        Empty
    }

    /// Indicates whether the `Tree` is populated or entirely empty.
    /// 
    pub fn is_empty(&self) -> bool 
    {
        matches!(self, Empty)
    }

    /// Retrieves the value associated with the given key. If the key exists in
    /// the tree, `Some(&V)` is returned; `None` otherwise. If invoked on an
    /// empty tree, returns `None`.
    /// 
    pub fn get(&self, key: &K) -> Option<&V>
    {
        match self {
            Filled(_) => self.get_internal(key),
            _ => None,
        }
    }

    /// Returns a mutable reference to the value associated with the given key.
    /// If a value exists at `key`, then `Some(&mut V)` is returned; `None`
    /// otherwise. If invoked on an empty tree, returns `None`.
    /// 
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        match self {
            Filled(_) => self.get_mut_internal(key),
            _ => None,
        }
    }
    
    /// Inserts the given key and value into the binary tree. If the key was
    /// already present, then `Some(V)` is returned holding the former value
    /// of the key. If the key wasn't already present, `None` is returned.
    /// 
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        use Ordering::*;
        let mut ret = None;
        match self {
            Empty => {
                *self = Tree::new_with_insert(key, value);
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

    /// Removes the provided key from the binary tree. If the key was present
    /// in the tree, `Some(V)` is returned holding the former value; otherwise,
    /// `None` is returned.
    /// 
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

    /// Returns the key and value in the tree at the ordinal 0-based position 
    /// given by `index`. If the index was within range of the items in the 
    /// tree, the `index`-th item is returned as `Some((&K, &V))` holding both 
    /// the key and the value. If `index` was out of range, `None` is returned.
    /// This operation has `O(log n)` time-complexity.
    /// 
    pub fn get_nth(&self, index: usize) -> Option<(&K, &V)>
    {
        match self {
            Filled(_) => self.get_nth_internal(index as isize),
            _ => None,
        }
    }

    /// Internal implementation for `.find_nth()`. The public facing version
    /// prohibits passing negative values as indices, while the internal version
    /// needs flexibility to avoid unsigned overflows.
    /// 
    fn get_nth_internal(&self, index: isize) -> Option<(&K, &V)>
    {
        let mut ret  = None;
        let     wt_l = if self.left.is_filled() 
                            { self.left.weight } 
                    else { 0                };
        let idx_adj = index - wt_l;
        if idx_adj == 0 {
            ret = Some((&self.key, &self.value))
        }
        else if idx_adj > 0 && self.right.is_filled() {
            ret = self.right.get_nth_internal(idx_adj - 1);
        }
        else if self.left.is_filled() {
            ret = self.left.get_nth_internal(index);
        }
        ret
    }

    /// Internal implementation for `.get()`. Returns the value corresponding
    /// to the given key. Doesn't check whether tree is empty.
    ///
    fn get_internal(&self, key: &K) -> Option<&V>
    {
        use Ordering::*;
        let mut ret = None;
        match key.cmp(&self.key) {
            Less => {
                if self.left.is_filled() {
                    ret = self.left.get_internal(key);
                }
            },
            Greater => {
                if self.right.is_filled() {
                    ret = self.right.get_internal(key);
                }
            },
            Equal => {
                ret = Some(&self.value)
            },
        }
        ret
    }

    /// Internal implementation for `.get_mut()`. Returns a mutable reference
    /// to the corresponding value of the key. Doesn't check whether tree is
    /// empty or not before executing search.
    ///
    fn get_mut_internal(&mut self, key: &K) -> Option<&mut V>
    {
        use Ordering::*;
        let mut ret = None;
        match key.cmp(&self.key) {
            Less => {
                if self.left.is_filled() {
                    ret = self.left.get_mut_internal(key);
                }
            },
            Greater => {
                if self.right.is_filled() {
                    ret = self.right.get_mut_internal(key);
                }
            },
            Equal => {
                ret = Some(&mut self.value);
            }
        }
        ret
    }

    /// Returns the height of the tree, which is the log2 of the number of nodes
    /// and sub-nodes in the current `Tree`.
    /// 
    fn height(&self) -> isize
    {
        match self {
            Filled(node) => Self::floor_log2(node.weight),
            Empty => 0,
        }
    }

    /// A simple, but quick, calculation for `floor(log2(n))`.
    /// 
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

    /// Returns a value indicating whether the tree is balanced or not, with
    /// negative values indicating the tree is heavy on the right, and
    /// positive values indicating the tree is heavy on the left. The value 0
    /// indicates a perfectly balanced tree/sub-tree.
    /// 
    fn balance(&self) -> isize 
    {
        match self {
            Filled(node) => node.left.height() - node.right.height(),
            Empty => 0,
        }
    }

    /// Moves the tree from it's former location, replacing it with `Empty` and
    /// returns the moved value to the caller giving it ownership.
    ///
    /// The Rust version on HackerRank is old and won't support the 
    /// implementation below. To get avl-tree working on that site, this 
    /// following implementation has to be used instead:
    ///
    /// ```ignore
    /// // Only use this if you have to. It will recursively clone every subtree
    /// // in `right` and `left`. The uncommented version below is preferred
    /// // because it can move the node and its `Box` without cloning.
    ///
    /// fn take(&mut self) -> Tree<K, V>
    /// {
    ///    let t = self.clone();
    ///    *self = Empty;
    ///    t
    /// }
    /// ```
    /// 
    fn take(&mut self) -> Tree<K, V>
    {
        std::mem::take(self)
    }

    /// Indicates whether a `Tree` has nodes (`true`), or is `Empty` (`false`). 
    /// 
    fn is_filled(&self) -> bool
    {
        matches!(self, Filled(_))
    }

    /// Performs a left-left rotation on the current `Tree`. These methods are
    /// used to keep the tree in balance, so both left and right sub-trees 
    /// grow or shrink at nearly the same rate. The name of the method can
    /// be read as, "a left rotation is performed on the left branch." The
    /// node within the current tree will be updated to hold the former left
    /// node.
    /// 
    fn rotate_left_left(&mut self)
    {
        let mut n = self.take();
        let mut t = n.left.take();
        n.left    = t.right.take();
        t.right   = n;
        *self     = t;
        self.update_weights(2);
    }

    /// Performs a right-right rotation on the current `Tree`. The `Tree`'s
    /// node will be updated to hold the former right node.
    /// 
    fn rotate_right_right(&mut self)
    {
        let mut n = self.take();
        let mut t = n.right.take();
        n.right   = t.left.take();
        t.left    = n;
        *self     = t;
        self.update_weights(2);
    }

    /// Performs a right-left rotation on the current `Tree`.
    /// 
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

    /// Performs a left-right rotation on the current `Tree`.
    /// 
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

    /// Updates the weights of a sub-tree by descending `depth` levels in the
    /// tree to find valid values, which are then used to update the nodes
    /// in the higher ranks. This is invoked after rotations.
    /// 
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

    /// Returns the key and value of the rightmost node in the current `Tree`.
    /// This is invoked as part of the `.remove()` method.
    /// 
    fn predecessor(&self) -> (K, V)
    {
        let mut t = self;
        while let Filled(_) = t.right {
            t = &t.right;
        }
        (t.key.clone(), t.value.clone())
    }

    /// Returns the key and value of the leftmost node in the current `Tree`.
    /// Invoked by `.remove()`.
    /// 
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
    /// Implements the default value for `Tree`. This is needed as part of the
    /// `.take()` feature.
    /// 
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

    /// Implements `Deref` for the `Tree`. This makes the fields of the `Node`
    /// contained in the `Filled` variant accessible with minimal syntax.
    /// 
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
    /// Complements the implementation of `Deref` by giving access to mutable
    /// `Node` fields with minimal syntax.
    /// 
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference an Empty Tree."),
        }
    }
}

impl<K, V> Index<&K> for Tree<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Output = V;

    /// Gives the tree the square bracket indexing feature. The tree keys are
    /// used to index their related values.
    ///
    fn index(&self, key: &K) -> &Self::Output
    {
        match self.get(key) {
            Some(v) => v,
            None => panic!("Attempt to read non-existent key."),
        }
    }
}

impl<K, V> IndexMut<&K> for Tree<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    /// Gives the tree the indexing feature so it behaves like a dictionary
    /// which supports square bracket indexing.
    /// 
    fn index_mut(&mut self, key: &K) -> &mut Self::Output
    {
        match self.get_mut(key) {
            Some(v) => v,
            None => panic!("Key is not in the tree."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut tree = Tree::new();
        for ch in "qwertyuiopasdfghjklzxcvbnmklasjfal;jasjfsa;".chars() {
            tree.insert(ch, 5);
        }
        println!("{:#?}", tree);
    }
    
    #[test]
    fn update_or_insert_and_update() {
        let mut tree = Tree::new();

        match tree.get_mut(&'b') {
            Some(value) => *value += 7,
            None => {
                tree.insert('b', 7);
            }
        }
    }
}
