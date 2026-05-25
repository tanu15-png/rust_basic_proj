use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
    loop{ 
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut turn: u32 = 0;

        loop{ 
            
            let mut guess = String::new();
            println!("\nEnter the number: ");

            io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
            
            let guess : i32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue
            };
            turn += 1;
            match guess.cmp(&secret_number){
                Ordering::Less => println!("too small"),
                Ordering::Greater  => println!("too large"),
                Ordering::Equal => { 
                    println!("you won");
                    break;
                }
            }
        }

        println!("number of turns taken : {turn}\n");
        println!("enter if you want to continue to next round (y/n): ");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("failed to readline");

        if choice.trim()=="y"{
            continue;
        }else{
            break;
        }
        

    }
}
