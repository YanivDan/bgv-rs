// This file implements the encryption process for the BGV homomorphic encryption scheme.

use crate::poly::Polynomial;
use crate::keys::PublicKey;
use rand::RngCore;

/// Represents a ciphertext in the BGV scheme.
#[derive(Debug, Clone)]
pub struct Ciphertext {
    pub c0: Polynomial, // First part of the ciphertext
    pub c1: Polynomial, // Second part of the ciphertext
}

/// Encrypts a plaintext polynomial using the provided public key.
///
/// # Arguments
///
/// * `plaintext` - The plaintext polynomial to be encrypted.
/// * `pk` - The public key used for encryption.
///
/// # Returns
///
/// A `Ciphertext` representing the encrypted polynomial.
pub fn encrypt(plaintext: &Polynomial, pk: &PublicKey) -> Ciphertext {
    let mut rng = rand::thread_rng();
    let degree = plaintext.coefficients.len();

    // Generate a random polynomial for the encryption process
    let r = Polynomial::random(degree, &pk.modulus, &mut rng);

    // Compute the ciphertext components
    let c0 = pk.a.clone() * r.clone() + plaintext.clone(); // c0 = a*r + m
    let c1 = pk.b.clone() * r; // c1 = b*r

    Ciphertext { c0, c1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::generate_keys;
    use num_bigint::BigUint;

    #[test]
    fn test_encrypt() {
        let Ok((pk, _sk)) = generate_keys();
        let modulus = pk.modulus.clone();
        let plaintext = Polynomial::from_u64(&[1, 2, 3, 4], &modulus);

        let ciphertext = encrypt(&plaintext, &pk);

        assert_eq!(ciphertext.c0.coefficients.len(), plaintext.coefficients.len());
        assert_eq!(ciphertext.c1.coefficients.len(), plaintext.coefficients.len());
    }
}