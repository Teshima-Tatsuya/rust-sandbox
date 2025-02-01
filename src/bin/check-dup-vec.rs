use std::collections::HashSet;

fn main() {
    let input = vec!["r", "u", "s", "t", "r"];

    let hs: HashSet<_> = input.iter().collect();

    if hs.len() != input.len() {
        println!("重複あり");
    } else {
        println!("重複なし")
    }
}
