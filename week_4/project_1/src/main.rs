// Rust program to find roots

use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter value for a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
     let a:f32 = input1.trim().parse().expect("Not a valid number");


     println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
     let b:f32 = input2.trim().parse().expect("Not a valid number");

println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
     let c:f32 = input3.trim().parse().expect("Not a valid number");

     //Discriminant
    let d = b*b - 4.0 * a * c;
    println!("d is {}",d);

    if d > 0.0{
        println!("It has two distinct roots");
    }

    else if d == 0.0{
        println!("It has exactly one root");
    }

    else if d < 0.0{
        println!("It has no real roots");
    }
    


}

    