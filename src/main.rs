use std::io::{stdin, stdout, Write};
use rand::Rng;

fn machine_play() -> &'static str {
    let mut rng = rand::thread_rng();
    let movements: Vec<&str> = vec!["piedra", "papel", "tijera"];
    let machine_movement = movements[rng.gen_range(0..3)];
    return machine_movement;
}

fn main() {
    loop {
        print!("piedra, papel o tijera: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.pop();
        match input.clone().as_str() {
            "piedra" => {
                match machine_play() {
                    "piedra" => {
                        println!("Empate");
                    },
                    "papel" => {
                        println!("Perdiste");
                    },
                    "tijera" => {
                        println!("Ganaste");
                        break;
                    },
                    _ => panic!("Error")
                }
            },
            "papel" => {
                match machine_play() {
                    "piedra" => {
                        println!("Ganaste");
                        break;
                    },
                    "papel" => {
                        println!("Empate");
                    },
                    "tijera" => {
                        println!("Perdiste");
                    },
                    _ => panic!("Error")
            }
        },
        "tijera" => {
            match machine_play() {
                "piedra" => {
                    println!("Perdiste");
                },
                "papel" => {
                    println!("Ganaste");
                    break;
                },
                "tijera" => {
                    println!("Empate");
                },
                _ => panic!("Error")
            }
            }
            _ => {
                panic!("This in not a valid input")
            }
        }
    }
}


