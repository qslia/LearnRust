# Code Explanation: 2_is_palindrome.rs

## Overview
This program implements a palindrome checker for integers. A palindrome number reads the same forwards and backwards (e.g., 121, 1221, 12321).

## Code Structure

### Struct Definition
```rust
struct Solution;
```
- Defines a **unit struct** named `Solution` with no fields
- Unit structs are useful for grouping related functions (similar to static classes in other languages)
- Uses zero bytes of memory - it's purely for namespace organization
- Common pattern in LeetCode-style problems

### Implementation Block
```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // ...
    }
}
```
- `impl Solution`: Creates an implementation block for the `Solution` struct
- Contains an associated function (similar to static methods in other languages)
- `pub`: Makes the function public, accessible from outside the module
- **Associated Function**: Called with `Solution::is_palindrome()` rather than on an instance

## Algorithm Breakdown

### Step 1: Handle Negative Numbers
```rust
if x < 0 {
    return false;
}
```
- Negative numbers cannot be palindromes (the minus sign only appears at the start)
- Early return optimization - exits immediately for negative inputs
- Examples: -121 reads as "-121" forward but "121-" backward

### Step 2: Convert to String
```rust
let s = x.to_string();
```
- `to_string()`: Converts the integer to a `String` representation
- Examples: `121` → `"121"`, `10` → `"10"`
- This makes it easy to compare characters

### Step 3: Reverse the String
```rust
let rev: String = s.chars().rev().collect();
```
Breaking this down:
- `s.chars()`: Creates an iterator over Unicode characters
- `.rev()`: Reverses the iterator (returns items in opposite order)
- `.collect()`: Consumes the iterator and collects characters into a `String`
- Type annotation `: String` helps the compiler know what type to collect into

### Step 4: Compare Original and Reversed
```rust
s == rev
```
- Compares the original string with its reverse
- Returns `true` if they're equal (palindrome), `false` otherwise
- This is the return value of the function (no semicolon = expression)

## Main Function - Test Cases

```rust
fn main() {
    let x1 = 121;
    let x2 = -121;
    let x3 = 10;

    println!("{} -> {}", x1, Solution::is_palindrome(x1)); // true
    println!("{} -> {}", x2, Solution::is_palindrome(x2)); // false
    println!("{} -> {}", x3, Solution::is_palindrome(x3)); // false
}
```

### Test Cases Explained:
1. **`x1 = 121`** → `true`
   - Forward: 121
   - Backward: 121
   - ✓ Palindrome

2. **`x2 = -121`** → `false`
   - Negative number
   - Caught by the first condition
   - ✗ Not a palindrome

3. **`x3 = 10`** → `false`
   - Forward: 10
   - Backward: 01
   - ✗ Not a palindrome (leading zeros don't count)

### println! Macro
- `{}`: Placeholder for displaying values
- First `{}` displays the number
- Second `{}` displays the boolean result (true/false)

## Expected Output
```
121 -> true
-121 -> false
10 -> false
```

## Key Rust Concepts

### 1. Unit Structs
- `struct Solution;` creates a struct with no fields
- Used for namespace organization without data storage

### 2. Associated Functions
- Functions called on the type itself: `Solution::is_palindrome()`
- No `self` parameter (unlike methods)

### 3. Implicit Returns
- `s == rev` without semicolon returns the boolean value
- Last expression in a function is the return value

### 4. String Conversion
- `to_string()` trait method for converting types to strings
- Works on any type implementing the `Display` trait

### 5. Iterator Methods
- `.chars()`: String → character iterator
- `.rev()`: Reverses any iterator
- `.collect()`: Gathers iterator items into a collection

### 6. Type Annotations
- `: String` helps Rust infer the correct collection type
- Could be omitted in some cases but improves clarity

## Time & Space Complexity

- **Time Complexity**: O(n) where n is the number of digits
  - Converting to string: O(n)
  - Reversing: O(n)
  - Comparing: O(n)
  
- **Space Complexity**: O(n)
  - Stores both original and reversed strings

## Alternative Approach (Mathematical)
Note: This implementation uses string conversion for simplicity. An alternative approach would reverse the number mathematically without allocating strings:

```rust
pub fn is_palindrome_math(x: i32) -> bool {
    if x < 0 { return false; }
    
    let mut original = x;
    let mut reversed = 0;
    
    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    
    reversed == x
}
```

This would have O(1) space complexity but similar O(n) time complexity.

