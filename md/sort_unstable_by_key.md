Let's break it down step by step:

---

### Code

```rust
ints.sort_unstable_by_key(|&(s, _)| s)
```

Assume `ints` is:

```rust
let mut ints = vec![(0, 30), (5, 10), (15, 20)];
```

---

### What `sort_unstable_by_key` does

* Sorts a vector **in place**.
* You provide a closure that extracts a *key* to sort by.

So we sort the vector by the **start time** (`s`).

Equivalent description:

> "Sort the intervals by their start field."

---

### Breaking down the closure: `|&(s, _)| s`

#### Part 1: `| ... |`

This is a closure (anonymous function).

#### Part 2: `&(s, _)`

This is **pattern matching** + destructuring.

* The vector contains **tuples**: `(i32, i32)`
* Inside the closure, Rust hands you a **reference** to each tuple, like `&(start, end)`

So we write `&(s, _)` to:

1. Match the tuple structure `(start, end)`
2. Bind the first element to variable `s`
3. Ignore the second element (`_`)
4. Dereference the reference (`&...`) in the pattern

In other words, Rust passes:

```
&(0, 30)  →  s = 0
&(5, 10)  →  s = 5
&(15, 20) →  s = 15
```

#### Part 3: `s`

This is the **key** returned by the closure — what we compare when sorting.

---

### Why `&` is needed

Because `.sort_unstable_by_key()` gives the closure a reference to each element:

```rust
|x| { /* x is &(i32, i32) */ }
```

So instead of writing:

```rust
|x| x.0   // get the first field manually
```

We destructure it directly:

```rust
|&(s, _)| s
```

This is elegant and avoids `x.0`.

---

### Visualization

Before sorting:

```
ints = [(15, 20), (0, 30), (5, 10)]
```

After sorting by start time:

```
ints = [(0, 30), (5, 10), (15, 20)]
```

---

### TL;DR

| Piece of code          | Meaning                                                     |   |                                                    |
| ---------------------- | ----------------------------------------------------------- | - | -------------------------------------------------- |
| `sort_unstable_by_key` | Sort optimized for speed, does not preserve equal-key order |   |                                                    |
| `                      | &(s, _)                                                     | ` | Closure receives `&(i32, i32)` and destructures it |
| `s`                    | Use the first element (start time) as sort key              |   |                                                    |

---

If you'd like, I can rewrite it without destructuring to make it clearer:

```rust
ints.sort_unstable_by_key(|v| v.0);
```

Both are correct — the destructuring version just looks more Rust-idiomatic.

Want an explanation of `sort()` vs `sort_unstable()` next?
