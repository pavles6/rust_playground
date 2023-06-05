use std::time::Instant;

fn main() {
    let fib_timer = Instant::now();

    const FIB_N: u8 = 184;

    let result = fib(FIB_N);

    println!(
        "{}. fibonacci number: {:?}, time elapsed: {:.2?}",
        FIB_N,
        result,
        fib_timer.elapsed()
    );

    const F: f64 = 32.00;

    println!("{:.2}F is {:.2}c", F, f_to_c(F));
}

fn fib(n: u8) -> i128 {
    let mut prev = 0;
    let mut sum = 0;

    for i in 0..n + 1 {
        if i == 1 {
            sum = 1;
            continue;
        };

        sum = sum + prev;
        prev = sum - prev;
    }

    return sum;
}

fn f_to_c(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}
