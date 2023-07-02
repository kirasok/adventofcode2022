use std::fs;

fn main() {
    let input_file = "input";
    let input = fs::read_to_string(input_file).expect("input");
    let mut v: Vec<i32> = Vec::new();
    for var in input.split("\n\n") {
        let mut i: Vec<i32> = Vec::new();
        for item in var.split_whitespace() {
            i.append(&mut vec![item.parse::<i32>().unwrap()]);
        }
        let mut sum = 0;
        for item in i {
            sum += item;
        }
        v.append(&mut vec![sum]);
    }
    v.sort();
    let v1 = v.pop().unwrap();
    let v2 = v.pop().unwrap();
    let v3 = v.pop().unwrap();
    let sum = v1 + v2 + v3;
    println!("{sum}")
}
