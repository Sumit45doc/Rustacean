use std::io;

fn main() {
    println!("Please input a number");

    let mut usr_input= String::new();

    io::stdin().read_line(&mut usr_input).expect("Failure");

    let usr_input: i32 = usr_input.trim().parse().expect("Not an integer");

    let statement= if is_even(usr_input){"The number is Even"} else{"The number is odd"};   //if expression can be used to assign value to an variable.

    println!("{}", &statement);
}

fn is_even(x: i32) -> bool{
    if x%2==0{
        return true
    }
    false
}