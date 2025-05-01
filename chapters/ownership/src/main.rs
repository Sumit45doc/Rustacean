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




}