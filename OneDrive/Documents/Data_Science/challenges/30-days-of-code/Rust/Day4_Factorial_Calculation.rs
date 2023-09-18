fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let num = 5; // Change this to the number for which you want to calculate the factorial
    let result = factorial(num);
    println!("Factorial of {} is {}", num, result);
}
