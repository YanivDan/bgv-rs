// This file implements the decryption process for the BGV homomorphic encryption scheme.

use crate::poly::Polynomial;
use crate::keys::SecretKey;
use crate::encrypt::Ciphertext;

/// Decrypts the given ciphertext using the provided secret key.
/// 
/// # Arguments
/// 
/// * `ciphertext` - The ciphertext to be decrypted, represented as a polynomial.
/// * `secret_key` - The secret key used for decryption.
///
/// # Returns
/// 
/// Returns the original plaintext polynomial after decryption.
pub fn decrypt(ct: &Ciphertext, sk: &SecretKey) -> Polynomial {
    // m' = c0 + c1 * s
    let m = ct.c0.clone() + (ct.c1.clone() * sk.s.clone());
    // Reduce coefficients mod t (plaintext modulus)
    let t = num::BigUint::from(2u64); // For demo, plaintext modulus is 2
    let mut m = m;
    m.coefficients.iter_mut().for_each(|c| *c = &*c % &t);
    m
}