// Function to check if a given string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// Function to find the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to return the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

// Function to check if a given number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to return the median of a sorted array of integers
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first_string = &strings[0];
    let mut prefix = String::new();
    for (i, c) in first_string.chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix
}

// Function to return the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to find the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
    }
}

// Function to reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Function to check if a number is prime
fn is_prime_rust(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// Function to find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for &num in arr {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Testing the functions
    println!("{}", is_palindrome("racecar")); // true
    println!("{:?}", first_occurrence(&[1, 2, 3, 4, 5], 3)); // Some(2)
    println!("{:?}", shortest_word("the quick brown fox")); // Some("the")
    println!("{}", is_prime(13)); // true
    println!("{}", median(&[1, 2, 3, 4, 5])); // 3
    println!("{}", longest_common_prefix(&vec!["flower".to_string(), "flow".to_string(), "flight".to_string()])); // "fl"
    println!("{:?}", kth_smallest(&[4, 3, 1, 5, 2], 3)); // Some(3)
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("{}", max_depth(root)); // 3
    println!("{}", reverse_string("hello")); // "olleh"
    println!("{}", is_prime_rust(17)); // true
    println!("{:?}", merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6])); // [1, 2, 3, 4, 5, 6]
    println!("{}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
}
