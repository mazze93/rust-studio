// expect: E0502
fn main() {
    let mut text = String::from("signal");
    let view = &text;
    text.push('!');
    println!("{view}");
}
