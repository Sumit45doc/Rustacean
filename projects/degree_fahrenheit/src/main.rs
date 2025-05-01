use std::io;

fn main() {
    println!("Degrees(0) or Fahrenheit(1)");

    let mut x = String::new();
    
    io::stdin().read_line(&mut x).expect("Unable to read");

    let x:i32 = x.trim().parse().expect("Unable to parse");

    if x == 0{
        let fah= degree_to_fahreheit();
        println!("The fahrenheit equivalent is: {fah} fahrenheit");
    }
    else{
        let deg= fahrenheit_to_degrees();
        println!("The degrees equivalent is: {deg} degrees");

    }
}


fn degree_to_fahreheit() -> f32{
    println!("Enter the degrees");

    let mut x = String::new();
    
    io::stdin().read_line(&mut x).expect("Unable to read");

    let x:f32 = x.trim().parse().expect("Unable to parse");

    return (x*1.8) + 32.0;
}

fn fahrenheit_to_degrees() -> f32{
    println!("Enter the fahrenheits");

    let mut x = String::new();
    
    io::stdin().read_line(&mut x).expect("Unable to read");

    let x:f32 = x.trim().parse().expect("Unable to parse");

    return (x-32.0) * 0.556;
}