#![allow(unused)]
extern crate rand;

    use std::io;
    use rand::prelude::*;
    use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let mut chekers: u16 = rng.gen_range(20..51);
    if chekers%4==1{
        chekers += 1;
    }
    loop{
        println!("There are {} chekers", chekers);
        
        //player's number = input()
        let reader = io::stdin();
        let mut input_text = String::new();
        reader.read_line(&mut input_text).expect("failed to read line");
        let number = input_text.trim().parse::<u16>();


        chekers -= number;
        if chekers==0{
            println!("You lose");
            break;
        }
        
        if chekers%4==1{
            chekers -= 4-number;
        }else{
        chekers -= rng.gen_range(20..51);
    }
        if chekers==0{
            println!("You win");
            break
        }

    }
    
}
