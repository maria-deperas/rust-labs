use std::io;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let break_value = loop {
        println!("Podaj liczbę");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let mut x: u64 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => break true,
        };

        if x == 0 {
            break false;
        }

        let rand =  rand::rng().random_range(0..=5);
        x += rand;

        println!("Nowa wartość x: {}", x);

        let tab = powers(x);

        println!("Wartości potęg liczby {}: {:?}", x, tab);
        
        let collatz = collatz(tab);

        println!("Wyniki testu Collatza: {:?}", collatz);

        let mut file = File::create("xyz.txt").expect("Failed to create file");
        writeln!(file, "{:?}", collatz).expect("Failed to write");
    };
    
    if break_value {
        println!("Pętla zakończona z powodu błędu");
    } else {
        println!("Pętla zakończona z woli użytkownika");
    }

    println!("Podaj liczbę od 0 do 100");
    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let x: u32 = input.trim().parse().expect("Not a number");

    let (znaleziono, a, b) = znajdz(x);
    if znaleziono {
    println!("{x} = {a} * {b}");
    } else {
        println!("{x} nie da się zapisać jako iloczyn liczb od 1 do 10");
    }
}

fn powers(x: u64) -> [u64; 10] {
    let mut tab = [0u64; 10];

    tab[0] = x;
    for i in 1..10 {
        tab[i] = tab[i - 1] * x;
    }

    tab
}

fn collatz(tab: [u64; 10]) -> [bool; 10] {
    let mut result = [false; 10];

    for i in 0..10 {
        let mut val = tab[i];
        for _ in 0..100 {
            if val.is_multiple_of(2) {
                val /= 2;
            } else {
                val = val * 3 + 1;
            }
            if val == 1 {
                result[i] = true;
                break;
            }
        }
    }

    result
}

fn znajdz(x: u32) -> (bool, u32, u32) {
    let mut result = (false, 0, 0);

    'outer: for a in 1..=10 {
        for b in 1..=10 {
            if a * b == x {
                result = (true, a, b);
                break 'outer;
            }
        }
    }

    result
}
