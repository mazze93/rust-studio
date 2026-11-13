fn main() {
    let mut text = String::from("signal");
    let view = &text;
    println!("{view}"); // last use of the shared borrow
    text.push('!');
    println!("{text}");
}
