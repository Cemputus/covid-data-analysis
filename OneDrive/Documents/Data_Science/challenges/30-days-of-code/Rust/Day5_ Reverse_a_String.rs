fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "Hello, World!";
    let reversed = reverse_string(s);
    println!("Reversed string: {}", reversed);
}
