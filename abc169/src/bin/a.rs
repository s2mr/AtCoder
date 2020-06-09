use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let args = input.split(" ");
    let nums: Vec<i64> = args.map(|arg| arg.parse().unwrap() ).collect();
    println!("{:?}", nums[0] * nums[1]);
}
