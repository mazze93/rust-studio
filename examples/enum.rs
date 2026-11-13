#[derive(Debug)]
enum Job { Queued, Running { worker: u32 }, Complete(String) }
fn describe(job: &Job) -> String {
    match job {
        Job::Queued => "waiting".to_owned(),
        Job::Running { worker } => format!("worker {worker}"),
        Job::Complete(output) => format!("done: {output}"),
    }
}
fn main() {
    let jobs = [Job::Queued, Job::Running { worker: 7 },
                Job::Complete("report".into())];
    for job in &jobs { println!("{}", describe(job)); }
}
