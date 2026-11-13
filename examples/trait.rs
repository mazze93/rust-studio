trait Summary { fn summary(&self) -> String; }
struct Report { count: usize }
impl Summary for Report {
    fn summary(&self) -> String { format!("{} samples", self.count) }
}
fn show(item: &impl Summary) { println!("{}", item.summary()); }
fn main() { show(&Report { count: 4 }); }
