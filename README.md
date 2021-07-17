# avl-tree

An AVL Tree implementation in Rust that provides a dictionary-like 
interface. Keys are used as the data for comparison when inserting into the
tree. The keys can be associated with values, which can be retrieved using
the keys with `O(log n)` time-complexity. Insertions, deletions, lookups,
etc. are all `O(log n)` operations.

```rust
use avl_tree::*;

fn main() 
{
    let mut tree = Tree::new();
  
    for (i, ch) in "qwertyuiopasdfghjklzxcvbnm".chars().enumerate() {
        tree.insert(ch, i);
    }
    
    assert_eq!( tree[&'a'], 10 );
    
    // Items in the tree can also be accessed by their ordinal position in the
    // tree. This is an `O(log n)` operation.
    assert_eq!( tree.get_nth(25), Some((&'z', &19)) );
}
```
