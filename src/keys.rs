// This file contains the implementation of key generation for the BGV scheme.

use crate::poly::Polynomial;
use num::BigUint;
use rand::rngs::OsRng;
use rand::Rng;
use thiserror::Error;

/// Errors that can occur during key generation.
#[derive(Debug, thiserror::Error)]
pub enum KeyGenError {
    #[error("Failed to generate random values for key generation")]
    RandomnessError,
}

// Struct representing the public key
pub struct PublicKey {
    pub a: Polynomial, // Polynomial a
    pub b: Polynomial, // Polynomial b
    pub modulus: BigUint, // Modulus
}

// Struct representing the secret key
pub struct SecretKey {
    pub s: Polynomial, // Secret polynomial
    pub modulus: BigUint, // Modulus
}

/// Generate a BGV keypair.
///
/// # Returns
///
/// A `Result` containing the public and secret keys, or a `KeyGenError` if key generation fails.
pub fn generate_keys() -> Result<(PublicKey, SecretKey), KeyGenError> {
    let mut rng = OsRng; // Use secure random number generator
    let degree = 4; // Small for demo
    let modulus = BigUint::from(97u64); // Small prime for demo
    let t = BigUint::from(2u64); // Plaintext modulus

    // Secret key: small coefficients {-1, 0, 1}
    let s_coeffs: Vec<BigUint> = (0..degree)
        .map(|_| BigUint::from(rng.gen_range(0..=1u64)))
        .collect();
    let s = Polynomial::new(s_coeffs, modulus.clone());

    // a: uniform random
    let a = Polynomial::random(degree, &modulus, &mut rng);
    // e: small error
    let e_coeffs: Vec<BigUint> = (0..degree)
        .map(|_| BigUint::from(rng.gen_range(0..=1u64)))
        .collect();
    let e = Polynomial::new(e_coeffs, modulus.clone());

    // b = -a*s + t*e
    let mut b = a.clone() * s.clone();
    b.coefficients.iter_mut().for_each(|c| *c = (modulus.clone() - c.clone()) % &modulus);
    let te = Polynomial::new(e.coefficients.iter().map(|c| (c * &t) % &modulus).collect(), modulus.clone());
    b = b + te;

    Ok((
        PublicKey { a, b, modulus: modulus.clone() },
        SecretKey { s, modulus },
    ))
}