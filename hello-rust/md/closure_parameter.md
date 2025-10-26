Here’s a **simple, clean, and idiomatic Rust** solution for LeetCode:

### ✅ Approach

* Convert the number to a string
* Filter out `'0'` characters
* Parse back to `i64`
* Return the result

This works well because the constraint `1 <= n <= 10^15` fits safely in `i64`.

---

### ✅ Rust Solution

```rust
impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        n.to_string()
            .chars()
            .filter(|&c| c != '0')
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }
}
```

---

### 💡 Notes

* `.chars()` lets us iterate over each digit (character)
* `.filter()` removes only `'0'`
* `.collect::<String>()` makes a new string without zeros
* `.parse()` converts back into `i64`

---

### ✅ Example

`1020030` → `"1020030"`
→ Filter zeros → `"123"`
→ Parse → `123`

---

### ⏱️ Complexity

* **Time:** O(d) where d = number of digits
* **Space:** O(d)

---

If you'd like, I can show:
✅ An alternative pure arithmetic solution (no strings)
✅ A version that works for edge cases like leading zeros
✅ Explanation step-by-step

Just ask!

<br><br>

Great question — this is a common point of confusion when learning Rust iterators and closures.
Let’s break down what `|&c| c != '0'` means.

---

# ✅ First, what is a **closure**?

In Rust, a closure is like an inline, anonymous function.
Its syntax looks like this:

```rust
|parameter| expression
```

For example:

```rust
|x| x + 1
```

---

# ✅ In `filter(|&c| c != '0')`

We're using a closure to decide which characters to **keep**.

## 🔍 Let’s annotate it:

```rust
|&c|        // closure argument (pattern)
c != '0'    // boolean condition
```

---

# ✅ What does `c` represent?

* `.chars()` produces an iterator over **characters**
* Each element is a `char`
* But `filter` gives us `&char` (a reference), not `char`

➡️ So inside the closure, `c` is a `&char`

---

# ✅ Why `&c` in the closure parameters?

`|&c|` is **pattern matching**:

* `&c` means: **take a `&char` and destructure it into a `char`**

In other words, it dereferences automatically.

It's the same idea as writing:

```rust
|c_ref| *c_ref != '0'
```

But shorter.

---

# ✅ You can write it 3 different ways:

### ✅ Using `&c` (pattern match):

```rust
.filter(|&c| c != '0')
```

### ✅ Explicit dereference:

```rust
.filter(|c| *c != '0')
```

### ✅ Use `copied()` to avoid referencing:

```rust
.chars()
.copied()
.filter(|c| c != '0')
```

All are correct and equivalent.

---

# ✅ Why does Rust do this?

Rust tries to avoid copying values automatically, so iterators pass references (`&T`).
We can:

* **pattern match** (`&c`)
* or **dereference** (`*c`)

---

# ✅ What about `c != '0'`?

This expression checks:

* if the current character `c`
* is **not** equal to `'0'`

If true → keep it
If false → filter it out

---

# ✅ Summary Table

| Syntax | Meaning                  |            |                           |
| ------ | ------------------------ | ---------- | ------------------------- |
| `      | &c                       | `          | Destructure &char to char |
| `      | c                        | *c != '0'` | Dereference manually      |
| `'0'`  | char literal, not string |            |                           |

---

# 🧠 Final takeaway

* `|...|` defines closure parameters
* `.chars()` yields `&char`
* `|&c|` pattern-matches the reference away
* `c != '0'` filters out zeros

---

If you'd like, I can:
✅ Show a version without using strings
✅ Explain how `.filter()` works internally
✅ Compare closures vs normal functions in Rust

Just ask!

<br><br>

Absolutely — let’s go deeper and explain `| |` clearly and simply.
This is **one of the most important Rust syntaxes** to understand.

---

# ✅ `| |` is how Rust defines **closure parameters**

In Rust, closures are **anonymous functions** — functions without a name.

The syntax is:

```
|parameters| expression
```

Think of it like a mini inline function.

### Example:

```rust
|x| x + 1
```

This means:

* Take parameter `x`
* Return `x + 1`

---

# ✅ Compare closure vs normal function

Normal function:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

Closure version:

```rust
|x| x + 1
```

Both mean the same thing.

---

# ✅ `filter` expects a closure

When we write:

```rust
.filter(|c| ...)
```

We are saying:

> For each element `c`, run this closure and keep only those returning `true`.

---

# ✅ `|c|` — one parameter

```rust
|c|
```

* `|c|` means the closure takes **one argument** called `c`.

---

# ✅ Multiple parameters: `|a, b|`

Example:

```rust
|a, b| a + b
```

Two parameters: `a`, `b`.

---

# ✅ Zero parameters: `||`

Example:

```rust
|| println!("Hello")
```

No inputs, just do something.

---

# ✅ Closure body

Closures can have:

### Inline expression:

```rust
|x| x * 2
```

Or a block:

```rust
|x| {
    let y = x + 1;
    y * 2
}
```

---

# ✅ Return type

The **last expression** in the closure is returned — no `return` keyword needed.

---

# ✅ Using references: `|&c|`

Here’s our case:

```rust
.filter(|&c| c != '0')
```

* `|&c|` means the closure argument is a `&char`, and we destructure it into `char`.
* Pattern matching.

Equivalent to:

```rust
.filter(|c| *c != '0')
```

---

# ✅ Putting it all together

Full closure:

```rust
|&c| c != '0'
```

Breakdown:

| Piece      | Meaning                   |   |                                                          |
| ---------- | ------------------------- | - | -------------------------------------------------------- |
| `          | &c                        | ` | closure parameter (take `&char` and turn it into `char`) |
| `c != '0'` | return `true` if not zero |   |                                                          |

---

# ✅ Visualization

Imagine `.filter()` feeds characters like this:

```
'1' → closure → keep
'0' → closure → discard
'2' → closure → keep
```

---

# ✅ Why use closures here?

They're great for:

* short logic
* inline functions
* customizing iterator behavior

---

# 🧠 Memory trick

Think of `| |` as parentheses **for arguments** of mini functions.

In normal functions:

```
(x)
```

In closures:

```
|x|
```

---

# ✅ Final analogy

* `fn` defines named functions.
* `| |` defines anonymous mini-functions **inline**.

When iterating, closures are perfect.

---

If you want, I can:
✅ visually diagram how the closure runs on each digit
✅ explain the difference between closures and functions in depth
✅ show more closure examples

Just tell me!
