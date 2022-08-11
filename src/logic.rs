use rand::{thread_rng,Rng};
use std::{io, cmp::Ordering};

fn main() {

    println!("Hello! Welcome to Aritmetyze.");
    println!("This little game will help you get better at arithmetic. Don't use a calculator, cheater ;)");
    println!("The program uses 2 digits of accuracy, rounded to the nearest neighbor
    ");
    println!("Press Control+C to close the program!
    ");

    //The loop of the game

    loop {
        
        let mut rng = thread_rng();
        let a = rng.gen_range(0..=100);
        let b = rng.gen_range(0..=100);
        let mut _q = 0;
    
        //Decides operator to use
        let mut x = thread_rng();
        let x = x.gen_range(0..3);
  
          
        match x {
            0 =>    {println!("{} + {}",a,b); _q = a+b},
            1 =>    {println!("{} - {}",a,b); _q = a-b},
            2 =>    {println!("{} * {}",a,b); _q = a*b},
            _ =>    {println!("Fatal error"); panic!()},
        };
    
        let mut answer = String::new();
    
        io::stdin().read_line(&mut answer).expect("Can't read line");
    
        let answer:i32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Failed to read the number, overwriting to 0");0},
        };
    
        match answer.cmp(&_q) {
            Ordering::Equal =>  println!("Correct!"),
            _               =>  println!("Incorrect :("),
        };
    }
}