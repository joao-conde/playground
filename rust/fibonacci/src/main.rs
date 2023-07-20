fn main() {
    let mut memo = [0; 100];
    for x in 0..100 {
        println!("Fibo #{}: {}", x, fibonacci(x, &mut memo))
    }
}

fn fibonacci(n: usize, memo: &mut [u128; 100]) -> u128 {
    if memo[n] == 0 {
        memo[n] = match n {
            0 | 1 => n as u128,
            _ => fibonacci(n - 1, memo) + fibonacci(n - 2, memo),
        };
    }
    memo[n]
}
