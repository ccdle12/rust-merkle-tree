# TODO

- [ ] Merkle Tree Constructor
- [ ] Decide on Hashing Function (SHA-256?)
- [ ] Create Algorithm to create the next level above until root
- [ ] How to store it?
  - On the heap with Smart Pointers on each leaf?
  - In an array?


## Idea

```
pub struct MerkleTree {
    root Hash
    left Box<Option<MerkleNode>>,
    right Box<Option<MerkleNode>>,
    size?
    depth?
}
```

```
struct MerkleNode {
  parent Box<MerkleNode>,
  hash Hash,
  left Box<Option<MerkleNode>>,
  right Box<Option<MerkleNode>>,
  value Option<T>,
}
```

