fn main() {
    let question = "The Question".to_string();
    let answer = 43;
    println!("Question: {}\nAnswer: {}", question, answer);

    let a; // Rustは後のコードも評価して型推論をする。
    let b; // Kotlinだと型推論できずコンパイルエラーになる。
    a = 3;
    b = 4;
    println!("{}", max(a, b));
}

fn max(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b }
}
