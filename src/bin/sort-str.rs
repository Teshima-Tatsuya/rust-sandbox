use std::collections::HashSet;
use std::hash::Hash;

fn sort_str(str: &str) -> String {
    let mut strvec: Vec<char> = str.chars().collect();
    strvec.sort_unstable();
    strvec.into_iter().collect()
}

fn main() {
    let input = "testtosort".to_string();

    println!("{}", sort_str(&input))
}
