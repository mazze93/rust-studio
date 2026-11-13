fn main() {
    let first = 42_u32;
    let second = first; // u32 implements Copy
    assert_eq!(first, second);
}
