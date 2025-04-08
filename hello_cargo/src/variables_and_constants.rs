const PI : f32 = 3.14159; //Global constant (the name of constant is in capital<Naming convention>)


fn main(){

    const AGE: u32 = 55; // constant within local scope.

    let x = 5;  //Creates a immutable variable(no type annotation need to be provided, rust itself assigns a type)

    let mut y = 6;  //Creates a mutable variable y.

    y = 8;  //Re-assigning a value to y(Mutability).

    let y = y + 2;  // Shadowing: Now the variable y is immutable which is a cool feature of shadowing.

    let mut z = 3.15;

    let z = "I am now a string"; // Shadowing can be used to change type of variable as well. (Also now z is immutable)

}