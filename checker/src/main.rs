extern crate rand;

use std::io;
use std::Rng;


fn main() {
    let mut chekers = rand::thread_rng(30,50);
    if chekers%4==1{
        chekers += 1;
    }
    loop{
        println!("There are {} chekers", chekers);
        //player's number = input()
        //chekers -= number;
        if chekers==0{
            println!("You lose");
            break;
        }
        
        if chekers%4==1{
            //chekers -= 4-number;
        }else{
        chekers -= rand::thread_rng(1,3);
    }
        if chekers==0{
            println!("You win");
            break
        }

    }
    
}
