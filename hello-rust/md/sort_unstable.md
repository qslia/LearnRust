`sort_unstable` is a sorting method for slices (`Vec`, arrays) in Rust.

### ✅ Definition

```rust
nums.sort_unstable();
```

* Sorts the vector **in place** (modifies the original vector)
* Uses an **unstable sorting algorithm** (typically quicksort–style)
* Faster than the default `sort()` because it does **not** preserve the relative order of equal elements

---

### Stable vs Unstable sorting

| Method            | Preserves order of equal elements? | Potentially faster? |
| ----------------- | ---------------------------------- | ------------------- |
| `sort()`          | ✅ stable                           | slightly slower     |
| `sort_unstable()` | ❌ unstable                         | ✅ faster            |

Example:

```rust
let mut v = vec![("a", 2), ("b", 2), ("c", 1)];
v.sort(); // stable sort
```

Result:

```
[("c", 1), ("a", 2), ("b", 2)]
             ↑       ↑
      equal keys keep original order (a before b)
```

Now with `sort_unstable()`:

```rust
let mut v = vec![("a", 2), ("b", 2), ("c", 1)];
v.sort_unstable();
```

Result might be:

```
[("c", 1), ("b", 2), ("a", 2)]   // order of ("a",2) and ("b",2) can flip
```

Equal elements are **not guaranteed** to keep their previous order.

---

### When should I use it?

Use `sort_unstable` when:

* You **don't care** about maintaining order for equal elements (e.g., integers)
* You want the **fastest sorting** possible

In your missing numbers problem:

```rust
nums.sort_unstable();
```

is perfect because:

* `nums` contains **unique integers**, so "equal elements" won't happen
* Faster than `sort()`

---

### TL;DR

`sort_unstable` = fastest sort, doesn't maintain order of equal items
`sort` = stable sort, preserves equal-item order but slightly slower

Since your integers are unique, **there is no downside** to using `sort_unstable`.

---

If you want, I can show the compiled assembly difference or benchmark results.
