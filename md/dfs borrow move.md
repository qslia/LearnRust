Great question — this goes straight to **ownership vs borrowing** with `Rc<RefCell<_>>`.

---

## ✅ Why does DFS take `&Rc<RefCell<TreeNode>>`?

Because we **do not want to take ownership** of the node when traversing the tree — we only want to *borrow* it.

Here is the DFS signature again:

```rust
fn dfs(node: &Rc<RefCell<TreeNode>>, path: &mut Vec<i32>, out: &mut Vec<String>)
```

### `node: &Rc<RefCell<TreeNode>>` means:

* `Rc<...>` is **shared ownership pointer**
* `&` means we are only **borrowing** the `Rc`, not moving it

Why?
Because if DFS took ownership:

```rust
fn dfs(node: Rc<RefCell<TreeNode>>, ...)
```

then when you call `dfs(node_b.left.clone())` deeper in the tree, you have to **clone the Rc every time**, increasing reference counts unnecessarily.

Even worse, if DFS took plain `Rc` (no reference passed), you could only use `node` once — moving ownership of `Rc` would *invalidate* the caller's reference.

---

## ✅ Key ownership rule

> Passing `Rc` by value moves ownership.

But:

> Passing `&Rc` does NOT move — it only borrows.

DFS does not need to *own* the node, just read it and explore children.

---

## Example showing the difference

### If DFS expected `Rc<RefCell<TreeNode>>` (ownership):

```rust
dfs(root.unwrap(), ...);        // moves Rc OUT of root
dfs(node_b.left.clone(), ...);  // must clone Rc every time!!!
```

This costs more (extra refcount increments).

### With `&Rc<RefCell<TreeNode>>`:

```rust
let node = root.as_ref().unwrap();
dfs(node, ...);                 // no move, just borrow
dfs(left_child, ...);           // borrow again, no clone needed
```

No extra cloning, no ownership transfer, cheap.

---

## ✅ Why not `&RefCell<TreeNode>`?

Because the real tree pointer is the `Rc`.

* `Rc<...>` ensures shared ownership (multiple parents can point to a child)
* `RefCell<...>` gives interior mutability (borrow rules checked at runtime)

We need to borrow **the Rc itself**, not just borrow the inside value.

---

## 🧠 TL;DR

| Type passed into DFS       | Meaning               | Drawback                           |
| -------------------------- | --------------------- | ---------------------------------- |
| `Rc<RefCell<TreeNode>>`    | move ownership        | can't reuse node without `clone()` |
| `&Rc<RefCell<TreeNode>>` ✅ | borrow shared pointer | zero overhead                      |

DFS should **borrow**, not take ownership.

---

## If we didn’t use `&Rc`

Rust would error: *"value moved here and used again"* when recursing.

---

If you'd like, I can show a version where DFS takes `node: Option<Rc<RefCell<TreeNode>>>` instead, or a cleaner version avoiding `unwrap()`. Just say **"give cleaner version"**.
