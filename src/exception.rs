use std::env;

enum _Error {
    // Error
}

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap();
    let n: i32 = arg.parse().unwrap(); 
    println!("{}", 2 * n);
}