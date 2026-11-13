// expect: pass
fn main() {
    let values = [2, 4, 6];
    let total = std::thread::scope(|scope| {
        let handle = scope.spawn(|| values.iter().sum::<i32>());
        handle.join().expect("worker should not panic")
    });
    assert_eq!(total, 12);
}
