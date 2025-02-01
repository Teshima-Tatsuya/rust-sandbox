use std::collections::HashSet;
use std::hash::Hash;

fn has_duplicates<T: Eq + Hash>(input: &[T]) -> bool {
    let hs: HashSet<_> = input.iter().collect();
    hs.len() != input.len()
}

fn main() {
    let input = vec!["r", "u", "s", "t", "r"];

    if has_duplicates(&input) {
        println!("重複あり");
    } else {
        println!("重複なし")
    }
}
