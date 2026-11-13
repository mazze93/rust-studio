// expect: pass
fn main() {
    let words = [String::from("rust"), String::from("is"), String::from("clear")];
    let long: Vec<&str> = words.iter()
        .map(String::as_str)
        .filter(|word| word.len() > 2)
        .collect();
    assert_eq!(long, ["rust", "clear"]);
    assert_eq!(words.len(), 3); // the array was borrowed
}
