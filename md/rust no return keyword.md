Excellent question — you’re starting to notice one of Rust’s unique features 💡

The short answer is:

> ✅ Rust **does have return values**, but it **usually doesn’t need the `return` keyword**.

Let’s unpack that carefully 👇

---

## 🧩 1. Every block is an expression

In Rust, almost **everything is an expression** — it produces a value.
That means the **last line** of a function (or block) is automatically **returned**, as long as it doesn’t end with a semicolon.

### Example 1: Implicit return

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // 👈 no semicolon → this value is returned
}
```

* `a + b` is an **expression** that produces a value.
* Because there’s **no `;`**, Rust **returns it automatically**.

✅ Works the same as:

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

---

## ⚠️ 2. Semicolon means “don’t return this”

If you end the line with a semicolon, it becomes a **statement**, not an expression.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;  // 🚫 this does NOT return anything
}
```

This would cause an error:

```
error[E0308]: mismatched types
```

because the function expects an `i32`, but the last line doesn’t return a value.

---

## 🪄 3. You can still use `return` early

You can use `return` **explicitly** when you want to exit a function early.

```rust
fn divide(a: i32, b: i32) -> f64 {
    if b == 0 {
        return f64::INFINITY;  // 🛑 early return
    }
    (a as f64) / (b as f64)
}
```

---

## 🧠 4. In your Roman numeral function

In your code:

```rust
total
```

at the end means the function returns the value of `total` **implicitly**.

Same as:

```rust
return total;
```

but more idiomatic in Rust.

---

### ✅ Summary

| Style           | Uses `return`? | Rust style  | Notes                      |
| --------------- | -------------- | ----------- | -------------------------- |
| `return total;` | Yes            | OK          | Explicit return            |
| `total`         | No             | ✅ Preferred | Implicit, clean, idiomatic |

---

Would you like me to show a few examples comparing **functions with and without `return`**, so you can clearly see how Rust treats them differently?
