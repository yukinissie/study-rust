fn main() {
    println!("{}", max(1, 10));
}

fn max(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b }
}
