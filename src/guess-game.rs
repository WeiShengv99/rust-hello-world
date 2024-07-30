use std::{ cmp::Ordering, io };

use rand::Rng;

fn main() {
    println!("Guess number start!!!!!");
    let secret_number = rand::thread_rng().gen_range(0..101);
    // println!("secret_number: {}", secret_number);

    loop {
        println!("Please guess number");

        // value must clear, because its reference value &guess_string, so value must recreate
        let mut guess_string = String::new();

        io::stdin().read_line(&mut guess_string).expect("You give a invalid value");

        println!("Your guess number is : {}", guess_string);

        let guess_number: u32 = match guess_string.trim().parse() {
            // "Please give a number"
            Ok(num) => num,
            Err(_) => {
                println!("Please give a valid number");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You right");
                break;
            }
        }
    }
}
