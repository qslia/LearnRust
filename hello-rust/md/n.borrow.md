`n.borrow()` is used because the tree nodes are stored inside:

```rust
Rc<RefCell<TreeNode>>
```

So to understand `n.borrow()`, we need to break down `Rc` and `RefCell`.

---

## 1. Why `Rc<RefCell<T>>` ?

| Wrapper      | Purpose                                                        |
| ------------ | -------------------------------------------------------------- |
| `Rc<T>`      | Allows **multiple owners** of the same node (shared pointers). |
| `RefCell<T>` | Allows **mutable access at runtime** (interior mutability).    |

A binary tree structure needs:

* Multiple references (parent pointing to left and right child), so we use `Rc`
* Ability to mutate child pointers later, so we use `RefCell`

Thus each node is:

```rust
Rc<RefCell<TreeNode>>
```

---

## 2. What `RefCell` does

`RefCell<T>` stores the value but controls mutations at **runtime** (not compile time).
To access the stored `TreeNode`, we call:

```rust
n.borrow()
```

This returns a **temporary immutable reference** to the inner value.

| Method         | Meaning                                             |
| -------------- | --------------------------------------------------- |
| `borrow()`     | Get **immutable** access → returns `Ref<TreeNode>`  |
| `borrow_mut()` | Get **mutable** access → returns `RefMut<TreeNode>` |

---

## 3. Example

```rust
let node: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(1)));

let borrowed = node.borrow();        // immutable borrow
println!("{}", borrowed.val);        // OK

let mut borrowed_mut = node.borrow_mut(); // mutable borrow
borrowed_mut.val = 10;              // OK
```

Important rules:

* You can have many `borrow()` active at the same time ✔
* You can only have **one** `borrow_mut()` active at a time ✔
* `borrow()` + `borrow_mut()` at the same time → ❌ panic

This is checked **at runtime**.

---

## 4. Returning to your code line:

```rust
while let Some(n) = node {
    n.borrow().left.clone()   // <- uses borrow()
}
```

Here’s what happens:

* `n` is an `Rc<RefCell<TreeNode>>`
* `n.borrow()` gives access to the actual `TreeNode`
* `.left` accesses the left child
* `.clone()` clones the `Rc` pointer (not the whole subtree)

So:

* **borrow()** = "Give me read access to the node so I can inspect it."

---

## TL;DR

`n.borrow()` means:

> "Get an immutable reference to the TreeNode stored inside the RefCell so we can read its fields."

It does **not** clone the node, does **not** copy data, and is **safe** as long as no mutable borrow is active.

---

If you'd like, I can now draw the memory layout or rewrite the function using more beginner-friendly variable names.
