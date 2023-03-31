fn fib(n: u32) -> u32 {
    if n > 1 {
        fib(n - 1) + fib(n - 2)
    } else {
        n
    }
}
