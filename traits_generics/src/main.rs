// ═══════════════════════════════════════════════════════════════════════════
//          RUST TRAITS & GENERICS - PRACTICE CHALLENGES
// ═══════════════════════════════════════════════════════════════════════════

use std::{
    fmt::{Debug, Display},
    iter,
    process::Output,
};

// ═══════════════════════════════════════════════════════════════════════════
//                      CHALLENGE 1: Basic Display ⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Fix this function to accept anything that can be displayed
// and print it twice
pub fn print_twice<T: Display>(item: T) {
    println!("{}", item);
    println!("{}", item);
}

#[test]
fn test_challenge1() {
    print_twice(42);
    print_twice("hello");
    print_twice(3.14);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 2: Return with Traits ⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that returns a value that implements Display
// Return any value you want (number, string, etc.)
pub fn create_displayable() -> impl Display {
    43
}

#[test]
fn test_challenge2() {
    let result = create_displayable();
    println!("Result: {}", result);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 3: Multiple Traits ⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that accepts a parameter with BOTH Display and Debug
// Print it using both formatting styles
pub fn show_both<T: Display + Debug>(value: T) {
    println!("{}", value);
    println!("{:?}", value);
}

#[test]
fn test_challenge3() {
    show_both(42);
    show_both("test");
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 4: Comparison ⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that finds the larger of two values
// Requirements: values must be comparable and displayable
pub fn find_larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[test]
fn test_challenge4() {
    assert_eq!(find_larger(10, 20), 20);
    assert_eq!(find_larger(5.5, 2.2), 5.5);
    assert_eq!(find_larger('a', 'z'), 'z');
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 5: Clone and Transform ⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that clones a value and returns both original and clone
// Return a tuple: (original, clone)
pub fn clone_pair<T: Clone>(item: T) -> (T, T) {
    let clone = item.clone();
    (item, clone)
}

#[test]
fn test_challenge5() {
    let (orig, cloned) = clone_pair(vec![1, 2, 3]);
    assert_eq!(orig, vec![1, 2, 3]);
    assert_eq!(cloned, vec![1, 2, 3]);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 6: Math Operations ⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that adds two numbers and displays the result
// Hint: You need Add trait for addition, Display for printing, Copy for ease
pub fn add_and_show<T: std::ops::Add<Output = T> + Display + Copy>(a: T, b: T) -> T {
    let sum = a + b;
    println!("Sum: {}", sum);
    sum
}

#[test]
fn test_challenge6() {
    assert_eq!(add_and_show(5, 10), 15);
    assert_eq!(add_and_show(1.5, 2.5), 4.0);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 7: Sum a Vector ⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that sums all numbers in a vector
// Hint: You need Add, Default (for initial value), and Copy
use std::ops::Add;
pub fn sum_vector<T>(v: Vec<T>) -> T
where
    T: Add<Output = T> + Default + Copy,
{
    let mut sum = T::default();
    for x in v {
        sum = sum + x;
    }
    sum
}

#[test]
fn test_challenge7() {
    assert_eq!(sum_vector(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(sum_vector(vec![10, 20, 30]), 60);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 8: Generic Struct ⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a generic Pair struct that holds two values of the same type
// Add a method `new` to create it and `swap` to swap the values

pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Copy> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    pub fn swap(&self) -> Self {
        Self {
            first: self.second,
            second: self.first,
        }
    }
}

#[test]
fn test_challenge8() {
    let pair = Pair::new(5, 10);
    assert_eq!(pair.first, 5);
    assert_eq!(pair.second, 10);

    let swapped = pair.swap();
    assert_eq!(swapped.first, 10);
    assert_eq!(swapped.second, 5);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 9: Flexible Return ⭐⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that can return different types based on a flag
// Use Box<dyn Display> to return either a number or a string
pub fn flexible_return(flag: bool) -> Box<dyn Display> {
    if flag {
        Box::new(2)
    } else {
        Box::new("hello world")
    }
}

#[test]
fn test_challenge9() {
    let num = flexible_return(true);
    println!("Number: {}", num);

    let text = flexible_return(false);
    println!("Text: {}", text);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  CHALLENGE 10: DeFi Token Transfer ⭐⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a generic transfer function for DeFi
// Requirements:
// - Amount must be comparable (to check > 0)
// - Amount must be displayable (to print)
// - Amount must be copyable
// - Return true if transfer successful, false otherwise

pub fn transfer<T: PartialOrd + Default + Display + Copy>(
    from: &str,
    to: &str,
    amount: T,
    _token: &str,
) -> bool {
    if amount > T::default() {
        println!("Transferring {} tokens from {} to {}", amount, from, to);
        true
    } else {
        println!("Transfer failed: amount {} is not greater than 0", amount);
        false
    }
}

#[test]
fn test_challenge10() {
    assert_eq!(transfer("Alice", "Bob", 100_u64, "USDC"), true);
    assert_eq!(transfer("Bob", "Charlie", 0_u64, "ETH"), false);
    assert_eq!(transfer("Charlie", "Alice", 50.5_f64, "BTC"), true);
}

// ═══════════════════════════════════════════════════════════════════════════
//                  BONUS CHALLENGE 11: Multiple Parameters ⭐⭐⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that works with TWO different generic types
// First parameter: must be Display
// Second parameter: must be Debug
// Return: nothing
// Print both parameters appropriately
pub fn process_two_types<A: Display, B: Debug>(a: A, b: B) {
    println!("Display: {}", a);
    println!("Display: {:?}", b);
}

#[test]
fn test_challenge11() {
    process_two_types(42, vec![1, 2, 3]);
    process_two_types("hello", Some(100));
}

// ═══════════════════════════════════════════════════════════════════════════
//                  BONUS CHALLENGE 12: Iterator with Traits ⭐⭐⭐⭐⭐
// ═══════════════════════════════════════════════════════════════════════════

// TODO: Create a function that takes any collection that can be iterated
// and prints all items that implement Display
// Hint: Use IntoIterator trait

pub fn print_all<T>(collection: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for item in collection {
        println!("{}", item);
    }
}

#[test]
fn test_challenge12() {
    print_all(vec![1, 2, 3, 4, 5]);
    print_all([10, 20, 30]);
    print_all(vec!["a", "b", "c"]);
}

fn main() {
    println!("Hello, world!");
}

