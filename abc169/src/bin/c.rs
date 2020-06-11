use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let nums: Vec<f64> = input.split(" ").map(|arg| arg.parse().unwrap()).collect();
    let result = nums[0] * nums[1];
    println!("{:?}", result as i128);
}
