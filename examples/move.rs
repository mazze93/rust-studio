// expect: E0382
fn main() {
    let first = String::from("signal");
    let second = first;
    println!("{first} / {second}");
}
