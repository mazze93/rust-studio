fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
fn main() {
    let left = String::from("signal");
    let right = String::from("ok");
    assert_eq!(longer(&left, &right), "signal");
}
