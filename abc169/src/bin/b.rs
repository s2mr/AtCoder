use std::io;
use reduce::Reduce;

fn main() {
    let mut arg_count_str = String::new();
    io::stdin().read_line(&mut arg_count_str).unwrap();
    let mut args_str = String::new();
    io::stdin().read_line(&mut args_str).unwrap();

    // let arg_count: i64 = arg_count_str.parse().unwrap();
    let args: Vec<i128> = args_str.split(" ").map(|arg| arg.parse().unwrap() ).collect();

    let mut result: i128 = args.into_iter().reduce(|a, b| a * b).unwrap_or(0);

    if result > (10 as i128).pow(18) {
        result = -1;
    }

    println!("{:?}", result);
}