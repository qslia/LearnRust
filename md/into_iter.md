`into_iter()` is a Rust method that **consumes** a collection and turns it into an iterator, *moving the elements out* rather than borrowing them.

---

### ✅ Key idea

| Method        | What it does                                                 |
| ------------- | ------------------------------------------------------------ |
| `iter()`      | Iterate by **reference** (`&T`) — does *not* take ownership. |
| `iter_mut()`  | Iterate by **mutable reference** (`&mut T`).                 |
| `into_iter()` | Iterate by **value** (`T`) — *takes ownership* of elements.  |

---

### Example

```rust
let v = vec![1, 2, 3];

for x in v.into_iter() {
    println!("{}", x);
}
```

After this loop, `v` is **moved** and can no longer be used.

---

### In our case:

```rust
let map: HashMap<char, char> = vec![
    ('0', '0'),
    ('1', '1'),
    ('6', '9'),
    ('8', '8'),
    ('9', '6'),
]
.into_iter()       // <-- consumes the Vec of tuples
.collect();        // collects into a HashMap
```

Here:

* `vec![('0','0'), ...]` creates a `Vec<(char, char)>`
* `.into_iter()` turns it into an iterator over `(char, char)` **owned tuples**
* `.collect()` turns that iterator into a `HashMap<char, char>`

---

### 💡 Why use `into_iter()` here?

Because `collect()` needs to consume the iterator to build a new container (a `HashMap`), it requires ownership of each tuple. `into_iter()` gives that ownership.

If we used `.iter()` instead, we'd get `&('0','0')`, which cannot be collected into a `HashMap<char,char>` without copying.

---

### TL;DR

> `into_iter()` = *“take this collection and give me each item by value (move it)”*.

Perfect when you want to build another collection from the original.

---

If you want, I can rewrite the strobogrammatic solution *without* using `HashMap` to avoid `into_iter()` entirely — just ask.

<br>

Short answer:

> **No, `into_iter()` does *not* copy. It *moves* the values (takes ownership).**

---

### Longer Explanation

In Rust:

* `copy` means duplicating the data (requires `Copy` trait).
* `move` means transferring ownership without duplicating memory.

`into_iter()` **moves** each element out of the collection.
It does **not** clone or copy the items.

Example:

```rust
let v = vec![String::from("a"), String::from("b")];

for s in v.into_iter() {
    // s is moved out of v
    println!("{s}");
}

// v can no longer be used here because its contents were moved
```

If `String` were copied, `v` would still be usable — but since it's *moved*, `v` becomes invalid.

---

### Compare with `iter()` vs `into_iter()`

| Method         | What you get      | Ownership         | Copy happens? |
| -------------- | ----------------- | ----------------- | ------------- |
| `.iter()`      | `&T` (reference)  | does **not** move | ❌ no copy     |
| `.iter_mut()`  | `&mut T`          | does **not** move | ❌ no copy     |
| `.into_iter()` | `T` (owned value) | **moves** value   | ❌ no copy     |

---

### Why this matters in the strobogrammatic example

```rust
vec![
    ('0', '0'),
    ('1', '1'),
    ('6', '9'),
    ('8', '8'),
    ('9', '6'),
]
.into_iter() // moves each tuple (char, char)
.collect()
```

* The tuples are **moved into** the iterator,
* Then `collect()` takes ownership of them and builds a `HashMap<char, char>`.

Nothing is copied unless the type implements `Copy` and the compiler decides to implicitly copy it.
`char` happens to be `Copy`, but **`into_iter()` itself does not perform copying**; it performs a move.

---

### TL;DR

* `into_iter()` = move ownership
* Not copying
* After calling it, the original collection cannot be used anymore

---

If you’d like, I can show examples using `Copy` vs non-`Copy` types to make the behavior obvious.

<br>

### ✅ Intuition: "Move" is like *handing over the only copy*

Think of **ownership** as having a *physical item* that only one person can hold.

---

### ✅ Analogy: The backpack

You have **one backpack**, and inside are your items.

```rust
let backpack = vec!["book", "laptop", "water"];
```

If you call:

```rust
for item in backpack.into_iter() {
```

That means:

* You give the **whole backpack** to someone else.
* They take items **out of your backpack** one by one.
* After that, **you no longer have the backpack**, so you can’t use it.

> `into_iter()` = hand your backpack to someone else; you lose it.

---

### ✅ Analogy: Borrowing vs Moving

| Rust action                   | Real-world analogy                                                                     |
| ----------------------------- | -------------------------------------------------------------------------------------- |
| `iter()` (borrow)             | Someone *looks* in your backpack and reads the labels, but does **not** take anything. |
| `iter_mut()` (mutable borrow) | Someone rearranges your backpack items, but gives everything back.                     |
| `into_iter()` (move)          | Someone takes your backpack and keeps the items — now **you don't have them anymore**. |

---

### ✅ Copy vs Move analogy

Imagine you're dealing with:

* A **rock** (cheap to duplicate)
* A **rare diamond** (expensive, one-of-a-kind)

| type                                  | Rust behavior             | Analogy                                                |
| ------------------------------------- | ------------------------- | ------------------------------------------------------ |
| `Copy` type (like `char`, `i32`)      | duplicated on assignment  | You photocopy a rock.                                  |
| Move-only type (like `String`, `Vec`) | transferred on assignment | You give away the diamond — you don’t have it anymore. |

Example:

```rust
let a = String::from("abc");
let b = a;  // move happens
// a can no longer be used here
```

Analogy:

> You handed someone your only diamond (move).
> You can't use it anymore.

---

### 🧠 Why Rust cares about "move"?

Because Rust tracks **who owns memory**, so it knows exactly when the memory can be freed safely — without garbage collectors.

Ownership rules eliminate:

* double free
* dangling pointers
* memory leaks

---

### TL;DR

> **Move = giving away the only copy.**
>
> **Copy = duplicating a cheap item.**
>
> **Borrow = letting someone look without giving ownership.**

---

If you want, I can show you a tiny program that prints when moves happen, so you can *see* moves in action. Just say **"show debugging version"**.
