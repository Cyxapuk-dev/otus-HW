use std::time::Instant;
fn measure_time<F, A, R>(f: F, args: A) -> (R, std::time::Duration)
where
    F: FnOnce(A) -> R,
{
    let start = Instant::now();
    let res = f(args);
    let duration = start.elapsed();
    (res, duration)
}

fn sum(a: u64, b: u64) -> u64 {
    a + b
}

fn concat(a: &[&str]) -> String {
    a.join(" ")
}

fn main() {
    let (res, execution_time) = measure_time(|(a, b)| sum(a, b), (10, 20));
    assert_eq!(30, res);
    println!("sum took {execution_time:?}");

    let (res, execution_time) = measure_time(|strs| concat(strs), &["hello", "world"]);
    assert_eq!("hello world", res);
    println!("concat took {execution_time:?}");

    let (_, execution_time) = measure_time(
        |_| {
            println!("printing to stdout");
        },
        (),
    );
    println!("printing took {execution_time:?}");
}
