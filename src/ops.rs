// This file contains functions for homomorphic operations, specifically addition and multiplication of ciphertexts.

pub use crate::encrypt::Ciphertext;

// Homomorphic addition: (c0, c1) + (d0, d1) = (c0+d0, c1+d1)
pub fn homomorphic_add(c1: &Ciphertext, c2: &Ciphertext) -> Ciphertext {
    Ciphertext {
        c0: c1.c0.clone() + c2.c0.clone(),
        c1: c1.c1.clone() + c2.c1.clone(),
    }
}

// Homomorphic multiplication (naive, not full BGV relinearization):
// (c0, c1) * (d0, d1) = (c0*d0, c0*d1 + c1*d0)
pub fn homomorphic_multiply(c1: &Ciphertext, c2: &Ciphertext) -> Ciphertext {
    let new_c0 = c1.c0.clone() * c2.c0.clone();
    let new_c1 = (c1.c0.clone() * c2.c1.clone()) + (c1.c1.clone() * c2.c0.clone());
    Ciphertext {
        c0: new_c0,
        c1: new_c1,
    }
}