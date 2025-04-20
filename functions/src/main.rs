use std::i32;

fn main() {
    println!("Hello, world!");

    my_function();
    paramaterized_function(170, 'i', "Sac");
    let sum = return_sum(12, 13);

    println!("The value of sum is {sum}");

    if(is_even(5)){
        println!("The number is even");
    }
    else {
        println!("The number is odd");
    }
    
}

fn my_function(){
    println!("Hello, from my function");
}

fn paramaterized_function(x: i32, y: char, n: &str){
    println!("The height of {n} is {x} {y}");
}

fn return_sum(x: i32, y: i32) -> i32{
    x + y                                               // This is an return expression as it doesn't have a ending semicolon
}

fn is_even(x: i32) -> bool{
    if x%2 == 0{
        return true;                                    //return keyword is used for Early return
    }
    false
}