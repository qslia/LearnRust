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

<br><br><br>
This line is a **derive attribute** in Rust that automatically implements three traits for the `TreeNode` struct:

```2:2:hello-rust/src/bin/3_is_same_tree.rs
#[derive(Debug, PartialEq, Eq)]
```

Let me show you the context:

```2:11:hello-rust/src/bin/3_is_same_tree.rs
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
```

## Breakdown:

- **`#[derive(...)]`** - Tells the Rust compiler to automatically generate trait implementations
- **`Debug`** - Enables printing with `{:?}` and `{:#?}`
- **`PartialEq`** - Enables equality comparisons with `==` and `!=`
- **`Eq`** - Indicates the type has full equivalence (reflexive, symmetric, transitive)

## What Each Trait Does:

### 1. **`Debug`**
Allows you to print the struct for debugging:
```rust
let node = TreeNode::new(5);
println!("{:?}", node); // Works because of Debug
```

### 2. **`PartialEq`**
Enables comparison operators:
```rust
let node1 = TreeNode::new(5);
let node2 = TreeNode::new(5);
if node1 == node2 { // Works because of PartialEq
    println!("Equal!");
}
```

### 3. **`Eq`**
A marker trait that says "this type has full equality":
- **`PartialEq`** = can compare, but might have weird cases (like `NaN != NaN` for floats)
- **`Eq`** = all values can be compared reliably

## Why Not Write These Manually?

You *could* write them manually, but `derive` saves time:

**Without derive:**
```rust
impl Debug for TreeNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // 10+ lines of formatting code
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && 
        self.left == other.left && 
        self.right == other.right
    }
}
```

**With derive:**
```rust
#[derive(Debug, PartialEq, Eq)]
```

Much simpler! The compiler generates all the boilerplate for you. 🎉

<br><br><br>
[2 tools called]

This is the **`#[inline]` attribute** in Rust, which is a **compiler hint** for optimization.

## What it does:

`#[inline]` suggests to the compiler that it should try to **inline** the function at the call site instead of making a regular function call.

### Without inlining:
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
let result = add(5, 3); // Calls the function
```
The CPU jumps to the function, executes it, and jumps back.

### With inlining:
```rust
#[inline]
fn add(a: i32, b: i32) -> i32 { a + b }
let result = add(5, 3); // Compiler replaces this with: let result = 5 + 3;
```
The function code is copied directly into the caller, avoiding the function call overhead.

## Context in your code:

Based on the earlier version, it's likely applied to the `new` function:

```rust
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
```

## When to use `#[inline]`:

✅ **Good for:**
- Small, frequently-called functions
- Simple constructors (like `TreeNode::new`)
- Getters/setters
- Performance-critical hot paths

❌ **Avoid for:**
- Large functions (can bloat binary size)
- Rarely-called functions

## Types of inline:

| Attribute | Meaning |
|-----------|---------|
| `#[inline]` | "Please try to inline this" (compiler may ignore) |
| `#[inline(always)]` | "Always inline this" (forces inlining) |
| `#[inline(never)]` | "Never inline this" |

In most cases, the compiler is smart enough without hints, but `#[inline]` can help for small, hot functions like constructors.
