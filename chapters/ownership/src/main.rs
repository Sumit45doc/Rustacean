fn main() {
    let a = 10;
    {
        let b = 10;   
        let c = a+ 20;   // Both b and c are declared and hence limited to the inner scope. Can't be accessed outside this scope.
        // Note that a is accessible in inner scope as well. As inner scope is subset of outer scope.
    }
    // let d = b +2;   This will result in compile error as b isn't accessible in outer scope
    println!("Value of a is {a}");


    let s = "Hello";        //This is a harcoded string literal where the size is already known and will be stored on stack.

    let mut S = String::from("Hello");    // Here a space is allocated to String. The size can shrink or grow. This string is stored on Heap.

    S.push_str(" World!!");             // The String type is immutable by default like ohers.
    
    println!("{S}");

    let x = 5;
    let y = x;      //Here x and y are two different variables, as x is copied to y. Changing x won't affect the value of y and vice versa.

    let s1 = String::from("Hello");
    let s2 = s1;                        //Here the value of s1 is passed to s1 and not copied.At this moment both strings point to the same memory location.
    //From onwards s1 won't be accessible as its scope ends on line 24 and it is no longer valid. This is to follow the single ownership rule of rust.

    let s3 = String::from("The value will be cloned");
    let s4 = s3.clone();                    //This is like deep copy where both s3 and s4 will be allocatd on heap pointing to different memory spaces.


//........................................Functions and Ownership.......................................

    takes_ownership(s4);                    //The string data type doesn't implements copy trait due to which the ownership of s4 is moved from main to the calling function
    //From here onwards the sstring s4 won't be accessble inside main

 //   println!("s4 is {s4}");      s4 Won't be accessible here
    makes_copy(x);                          //The integer data type implements copy trait and therefore the value of x is copied to the calling function.

    println!("Value of x Is {x}");          //x is accessible as it's value is copied into the calling function

    let s5 = gives_ownership();

    let s6 = takes_and_gives_back_ownership(s3);

    println!("Value of s6 is {s6}");
//    println!("Value of s1 is {s1}");           //s1 was moved to takes_and_gives_back function which is it's new owner. Making it inaccessible in main.

}

fn takes_ownership(some_string: String){
    println!("I'm the owner of {some_string}");
    //The function is new owner of some_string. Once the function is executed drop is called over some_string freeing up memory on heap.
}

fn makes_copy(some_int: i32){
    println!("The value of integer is {some_int}");         //Some_int won't be accessible outside the scope of this function.
}

fn gives_ownership() -> String{
    let s = String::from("string isnide give ownership");
    s
}

fn takes_and_gives_back_ownership(some_string: String) -> String{           
    println!("String inside takes and gives back is {some_string}");
    some_string
}