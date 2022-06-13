use std::fs;
use regex::Regex;

fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 { return true; }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) { return true; }
        haystack = &haystack[1..];
    }
    false
}

fn main() {
    // Bai tap 1
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];

    println!("{}",is_sub(&org_arr, &sub_arr));

    // Bai tap 2
    let str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    println!("Enter some string:");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("The number of string is {}", str.matches(&line.trim()).count());

    // Bai tap 2 nang cao
    let contents = fs::read_to_string("1-s2.0-S0960982203005347-mmc6.txt").unwrap();

    println!("Enter some string:");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let re = format!(r"{}",line);
    let re = Regex::new(&re).unwrap();
    println!("The number of string is {}", re.captures_iter(&contents).count());

}
