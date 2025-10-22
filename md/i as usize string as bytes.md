Excellent question — this gets into how **Rust handles string indexing** and **types**. Let’s break it down carefully 👇

---

### 🧩 1. Why `s.as_bytes()`

In Rust, **you cannot index a `String` directly** like `s[i]`.
That’s because a `String` is UTF-8 encoded — one character (a “grapheme”) can take **more than one byte** in memory.

Example:

```rust
let s = String::from("你好");
println!("{}", s.len()); // 6 bytes, even though there are 2 characters
```

So, `s[0]` doesn’t make sense — the compiler can’t tell which bytes belong to the first character safely.

➡️ But for this problem, we only need to check for **ASCII spaces (' ')**, which are always **1 byte**.
So it’s safe (and faster) to convert to bytes:

```rust
let bytes = s.as_bytes();
```

Now `bytes[i]` is legal — it gives you the `u8` value of the character at position `i`.

---

### 🔢 2. Why `i as usize`

In Rust:

* `String::len()` returns a `usize` (unsigned integer type for indexing).
* But we use `i32` in our loop because we want to **go backward** (`i -= 1`), and `usize` can’t be negative.

So we do this:

```rust
let mut i = (bytes.len() as i32) - 1;
```

That lets us loop backward safely:

```rust
while i >= 0 && bytes[i as usize] == b' ' {
    i -= 1;
}
```

We must cast `i` back to `usize` (`i as usize`) when indexing, because arrays, slices, and strings in Rust only accept `usize` indices.

---

### 🧠 Summary

| Expression     | Purpose                                                                                |
| -------------- | -------------------------------------------------------------------------------------- |
| `s.as_bytes()` | Convert the UTF-8 string into a byte slice (`&[u8]`) so we can safely index characters |
| `i as usize`   | Convert our signed integer loop index back to unsigned for indexing                    |

---

Would you like me to show how to do the **same logic without using `as_bytes()`**, using `.chars().rev()` instead (pure Unicode-safe version)?
