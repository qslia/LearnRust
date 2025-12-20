Here is a **clean, idiomatic Rust implementation** using the **center expansion principle**, matching the logic you just learned.
This version is **LeetCode-ready**, safe, and easy to reason about.

---

## Core Idea Recap (Very Short)

* Every palindrome expands from a **center**
* For each index:

  * expand `(i, i)` → odd length
  * expand `(i, i+1)` → even length
* Keep the longest range

---

## ✅ Rust Implementation (Center Expansion)

```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();

        if n <= 1 {
            return s;
        }

        let mut start = 0usize;
        let mut max_len = 1usize;

        // expand around center [left, right]
        let expand = |mut left: isize, mut right: isize| -> (usize, usize) {
            while left >= 0
                && (right as usize) < n
                && bytes[left as usize] == bytes[right as usize]
            {
                left -= 1;
                right += 1;
            }
            ((left + 1) as usize, (right - 1) as usize)
        };

        for i in 0..n {
            // odd length palindrome
            let (l1, r1) = expand(i as isize, i as isize);
            if r1 - l1 + 1 > max_len {
                start = l1;
                max_len = r1 - l1 + 1;
            }

            // even length palindrome
            let (l2, r2) = expand(i as isize, i as isize + 1);
            if r2 >= l2 && r2 - l2 + 1 > max_len {
                start = l2;
                max_len = r2 - l2 + 1;
            }
        }

        s[start..start + max_len].to_string()
    }
}
```

---

## Why This Rust Code Is Correct

### 1️⃣ `as_bytes()` is safe here

Constraints say:

> s consists of only digits and English letters

So:

* 1 byte = 1 character
* slicing `s[start..end]` is **UTF-8 safe**

---

### 2️⃣ Why `isize` for pointers?

Rust `usize` cannot go negative.
Using `isize` allows safe expansion to the left (`left -= 1`).

---

### 3️⃣ Why closure `expand`?

* Avoids duplicated logic
* Keeps main loop clean
* Common Rust interview pattern

---

## Complexity

* **Time**: `O(n²)`
* **Space**: `O(1)`

Works comfortably for `n ≤ 1000`.

---

## Interview One-Liner (Rust Version)

> “I convert the string to bytes and expand around each possible center, checking both odd and even palindromes, while tracking the longest range.”

---

If you want next:

* 🔥 **Rust Manacher (O(n))**
* 🔍 Step-by-step trace for `"babad"`
* 🧠 How to write this **without using bytes**

Just tell me.
