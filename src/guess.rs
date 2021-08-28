fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("not valid: {}", n);
    }
    n == 5
}

fn main() {
    guess(15);
}