Short answer: **No**, you cannot do `nums.sum()`.

---

# ❌ `nums.sum()` does NOT work

`sum()` is a method on **iterators**, not on `Vec`.

So this is **invalid**:

```rust
nums.sum(); // ❌ won't compile
```

You'll get an error like:

```
error[E0599]: no method named `sum` found for struct `Vec<i32>`
```

---

# ✅ Correct ways

### 1. Use `.iter().sum()`

```rust
let s: i32 = nums.iter().sum();
```

### 2. Use `.into_iter().sum()` (consumes the vector)

```rust
let s: i32 = nums.into_iter().sum();
```

Both are valid because `sum()` is defined on anything that implements `Iterator`.

---

# ⭐ Why `nums.sum()` cannot work

`Vec<T>` is **not** an iterator.
It only *produces* iterators through `.iter()` or `.into_iter()`.

So you must call `.iter()` (borrow) or `.into_iter()` (move).

---

If you want, I can show how to implement your own trait so that `nums.sum()` *does* work for Vec, but that’s custom code—not standard Rust.
