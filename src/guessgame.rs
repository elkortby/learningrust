use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run () {
    
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut cpt = 0;
    loop {
        println!("input your guess");    
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your input is {}", guess);
        
        match secret.cmp(&guess) {
            Ordering::Less => {
                println!("Too Big");
                cpt = cpt + 1;
            },
            Ordering::Greater => {
                println!("Too Small");
                cpt = cpt + 1;
            },
            Ordering::Equal => {
                println!("You Win! {att} attempts", att = cpt);
                break;
            },
        }
    }
}