fn main(){
    let x = 7;           // If a type is not annotated a 32 bit signed integer type is infered by default.

    let y: i8 = 157;     // Integer overflow concept

    let z = 1.23;        // The default size of a float type is 64 bits. 

    let b = true;

    let c = 'Ü'; 

    let tup = (500, 10.5, true);

    let (x, y, z) = tup;

    println!("The value of x is {x}");                // Accessing a tuple -> 500

    println!("The first element of the tuple is {}", tup.1);

    let arr = [1,3,5,8];

    let ar = [1;3];                                   //[1, 1, 1]

    let l: [bool; 3] = [true, false, false];

    println!("The first element is {}", l[1]);        // false
}
