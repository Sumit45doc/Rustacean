use std::io;

fn main() {
    println!("Enter a number of your choice");

    let mut usr_input= String::new();

    io::stdin().
                read_line(&mut usr_input)
                .expect("Failed to read");

    let usr_input: i32 = usr_input.trim().parse().expect("Not a integer");

    print_till_input(usr_input);

    loop_inside_loop(5);

    conditional_loop(3);

    for_loop();

    for_range();
}

fn print_till_input(x:i32){

    let mut y= 0;

    loop {
        if y < x{
            println!("The value of Y is {y}");
            y += 1;
        }
        else{
            break;
        }
    }
    println!("Bye bye !!");
}

fn loop_inside_loop(mut x:i32){
    'count_down: loop{                  //Loop labels can be used to distinguish between loops
       
        loop{
            x -= 1; 
            if x == 1{
                break;
            }
            else if x == 2{
                break 'count_down;      //A break statement can be used to break outer loop as well.
            }
            else{
                continue;
            }
        }
    }
}

fn conditional_loop(mut x:i32){
    while x!=0{                         //Until the condition is true the loop runs    
        println!("Value of X is {x}");
        x-=1;
    }
}

fn for_loop(){
    let a = [12, 13, 45, 78];

    for ele in a {
        println!("The value is: {ele}");
    }
}

fn for_range() {
    for number in (1..4).rev() {    //For loop can be iterated over a range both in reverse as well as standard manner.
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}