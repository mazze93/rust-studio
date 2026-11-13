// expect: E0515
fn invalid() -> &'static str {
    let local = String::from("signal");
    &local
}
fn main() { println!("{}", invalid()); }
