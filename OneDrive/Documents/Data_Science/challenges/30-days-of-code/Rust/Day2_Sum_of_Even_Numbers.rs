fn main() {
    let sum: i32 = (2..=100).step_by(2).sum();
    println!("Sum of even numbers from 2 to 100: {}", sum);
}
