use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Raad maar raak!");
    
    let secret_number = rand::thread_rng().gen_range(1, 1001);

    loop {
        println!("Geef ons een nummer, tussen 0 en 1000.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Kon helaas 🥜kaas je invoer kan ik niet lezen");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("je hebt geraden : {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Dat is veel te klein!"),
            Ordering::Greater => println!("Dat is wel beetje te veel!"),
            Ordering::Equal => {
                println!("Winner winner \"praise the rng\". 🐈😸");
                break;
            }
        }
    }
}
