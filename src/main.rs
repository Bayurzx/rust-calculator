use std::env::{args, Args};

fn main() {
    let mut args:Args = args();

    // defining variables and declaring their types
    let first:String = args.nth(1).unwrap();
    // we need to do this apparently to unlock characters
    let operator:char = args.nth(0).unwrap().chars().next().unwrap();
    let second:String = args.nth(0).unwrap();

    // convert variables types with parse
    let first_num:f32 = first.parse::<f32>().unwrap();
    let second_num:f32 = second.parse::<f32>().unwrap();
    let result:f32 = operator_(operator, first_num, second_num);

    // println!("Hello, {}", result);
    return println!("{:?}", output(first_num, operator, second_num, result));
}

fn operator_(operator: char, first_num:f32, second_num:f32) -> f32 {
    // if operator == '+' {
    //     return first_num + second_num;
    // } else if operator == '-' {
    //     return first_num - second_num;
    // } else if operator == '/' {
    //     return first_num / second_num;
    // } else if operator == '*' {
    //     return first_num * second_num;
    // } else {
    //     return 0.0;
    // }

    match operator { // aka switch case
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '/' => first_num / second_num,
        '*' |'x' | 'X' => first_num * second_num,
        _ => panic!("Invalid operator was used") // panicking aka error_out

    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32)-> String {
    format!("{} {} {} is equal to {}", first_num, operator, second_num, result)
}