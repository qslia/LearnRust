struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers are never palindrome
        if x < 0 {
            return false;
        }

        let s = x.to_string(); // convert number to string
        let rev: String = s.chars().rev().collect(); // reverse the string

        s == rev
    }
}

fn main() {
    let x1 = 121;
    let x2 = -121;
    let x3 = 10;

    println!("{} -> {}", x1, Solution::is_palindrome(x1)); // true
    println!("{} -> {}", x2, Solution::is_palindrome(x2)); // false
    println!("{} -> {}", x3, Solution::is_palindrome(x3)); // false
}
