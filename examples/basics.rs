fn label(score: u8) -> &'static str {
    if score >= 80 { "high" } else { "ordinary" }
}
fn main() {
    let raw = "81";
    let score: u8 = raw.parse().expect("the literal is a valid u8");
    let mut attempts = 0;
    attempts += 1;
    println!("{} after {attempts} attempt", label(score));
}
