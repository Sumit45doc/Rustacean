use std::io;

fn main() {
    println!("Enter the index of the element you want to access");

    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Unable to read");

    let x: i32= x.trim().parse().expect("Unable to parse");

    let y = fib(x);

    println!("The elemet at {x}th index is {y}")
}

fn fib(x:i32) -> i32{
    if x <= 1{
        return  x;
    }
    return fib(x-1) + fib(x-2);
}