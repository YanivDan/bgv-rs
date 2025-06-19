// This is the entry point of the BGV homomorphic encryption scheme demonstration.
// It encrypts two messages, performs homomorphic addition and multiplication, and decrypts the results.

mod keys;
mod poly;
mod encrypt;
mod decrypt;
mod ops;

use clap::{Command, Arg};
use poly::Polynomial;
use std::io;

fn main() {
    let matches = Command::new("BGV CLI Tool")
        .version("1.0")
        .author("Yaniv")
        .about("Performs FHE operations using the BGV scheme")
        .arg(
            Arg::new("integers")
                .short('i')
                .long("integers")
                .value_name("INTEGERS")
                .help("A comma-separated list of integers to encrypt"),
        )
        .get_matches();

    // Parse integers from CLI
    let integers: Vec<u64> = matches
        .get_one::<String>("integers")
        .map(|s| s.split(',').filter_map(|s| s.trim().parse().ok()).collect())
        .unwrap_or_else(|| vec![1, 2]);

    // Key generation
    let (pk, sk) = match keys::generate_keys() {
        Ok((pk, sk)) => (pk, sk),
        Err(e) => {
            eprintln!("Error during key generation: {}", e);
            return;
        }
    };

    let modulus = pk.modulus.clone();

    // Encrypt integers
    let ciphertexts: Vec<_> = integers
        .iter()
        .map(|&int| {
            let poly = Polynomial::from_u64(&[int, 0, 0, 0], &modulus);
            encrypt::encrypt(&poly, &pk)
        })
        .collect();

    // Ask user for operations
    println!("How many operations do you want to perform?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_operations: usize = input.trim().parse().unwrap_or(1);

    for _ in 0..num_operations {
        println!("Choose operation: add or multiply");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let operation = input.trim();

        if operation == "add" {
            let result = ciphertexts.iter().skip(1).fold(ciphertexts[0].clone(), |acc, c| {
                ops::homomorphic_add(&acc, c)
            });
            let decrypted = decrypt::decrypt(&result, &sk);
            println!("Decrypted Sum: {:?}", decrypted.coefficients);
        } else if operation == "multiply" {
            let result = ciphertexts.iter().skip(1).fold(ciphertexts[0].clone(), |acc, c| {
                ops::homomorphic_multiply(&acc, c)
            });
            let decrypted = decrypt::decrypt(&result, &sk);
            println!("Decrypted Product: {:?}", decrypted.coefficients);
        } else {
            println!("Invalid operation. Please choose 'add' or 'multiply'.");
        }
    }
}