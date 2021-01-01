extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to pikes and fixed game");

    let secret: Vec<i32> = generate_secret();

    loop {
        // Variable to receive user input
        let mut guess = String::new();
        // Variable to pikes
        let mut p: i32 = 0;
        // Variable to fixed
        let mut f: i32 = 0;

            // Capture user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        if guess.len() > 5 {
            println!("Ingresa un número de 4 dígitos");
            continue
        }

        // Populate vector with user input
        let input_vec :Vec<char> = guess.trim().chars().collect();

        // Cast vector to i32 from user input vector
        let final_input_vec :Vec<i32> = input_vec
            .iter() // Iterate
            .map(|&e| e.to_digit(10).unwrap() as i32) // map and cast
            .collect();

        // Iterate over vectors and count pikes and fixed
        for (index, n) in secret.iter().enumerate() {
            for (pos, m) in final_input_vec.iter().enumerate() {
                if index == pos && n == m {
                    f = f + 1;
                }
                if index != pos && n == m {
                    p = p + 1;
                }
            }
        }

        // match fixed value
        match f {
            4 => {
                println!("Felicitaciones ganaste");
                break;
            }
            _ => {
                println!("Picas {} Fijas {}", p, f);
                continue;
            }
        }
    }
}

fn generate_secret() -> Vec<i32> {
    let mut secret_vec = Vec::new();

    loop {
        let number = rand::thread_rng().gen_range(0..10);
        if secret_vec.contains(&number) {
            continue;
        }
        if secret_vec.len() >= 4 {
            break;
        }
        secret_vec.push(number);
    }
    secret_vec
}
