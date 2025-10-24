# Same Tree

## Problem
Given the roots of two binary trees `p` and `q`, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

## Examples

### Example 1:
```
Input: p = [1,2,3], q = [1,2,3]
Output: true
```

### Example 2:
```
Input: p = [1,2], q = [1,null,2]
Output: false
```

### Example 3:
```
Input: p = [1,2,1], q = [1,1,2]
Output: false
```

## Constraints
- The number of nodes in both trees is in the range [0, 100]
- -10^4 <= Node.val <= 10^4

## Solution Approach

The solution uses **recursive depth-first comparison**:

1. **Base Cases**:
   - If both trees are `None` (empty), they are the same → return `true`
   - If only one tree is `None`, they are different → return `false`

2. **Recursive Case**:
   - If both trees have nodes:
     - Check if the current node values match
     - Recursively check if left subtrees are the same
     - Recursively check if right subtrees are the same
   - Return `true` only if all three conditions are met

## Rust-Specific Details

- Uses `Option<Rc<RefCell<TreeNode>>>` for nullable tree nodes
- `Rc` (Reference Counted) allows multiple ownership
- `RefCell` allows interior mutability with runtime borrow checking
- Pattern matching with `match` elegantly handles all cases
- `.borrow()` is used to access the node's data
- `.clone()` is used to clone the `Rc` pointers (not the entire tree) for recursion

## Time Complexity
- **O(min(n, m))** where n and m are the number of nodes in trees p and q
- We visit each node once until we find a difference or traverse both trees

## Space Complexity
- **O(min(h1, h2))** where h1 and h2 are the heights of the trees
- Space is used by the recursion call stack
- In the worst case (skewed tree), this could be O(n)

