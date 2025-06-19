// This is the entry point of the BGV homomorphic encryption scheme demonstration.
// It encrypts two messages, performs homomorphic addition and multiplication, and decrypts the results.

mod keys;
mod poly;
mod encrypt;
mod decrypt;
mod ops;

use poly::Polynomial;

fn main() {
    // Key generation
    let (pk, sk) = match keys::generate_keys() {
        Ok((pk, sk)) => (pk, sk),
        Err(e) => {
            eprintln!("Error during key generation: {}", e);
            return;
        }
    };

    let modulus = pk.modulus.clone();

    // Encode messages as polynomials
    let m1 = Polynomial::from_u64(&[1, 0, 0, 0], &modulus);
    let m2 = Polynomial::from_u64(&[2, 0, 0, 0], &modulus);

    // Encrypt
    let c1 = encrypt::encrypt(&m1, &pk);
    let c2 = encrypt::encrypt(&m2, &pk);

    // Homomorphic addition
    let c_add = ops::homomorphic_add(&c1, &c2);
    // Homomorphic multiplication (demo, not full BGV)
    let c_mul = ops::homomorphic_multiply(&c1, &c2);

    // Decrypt
    let m_add = decrypt::decrypt(&c_add, &sk);
    let m_mul = decrypt::decrypt(&c_mul, &sk);

    println!("Decrypted Sum: {:?}", m_add.coefficients);
    println!("Decrypted Product: {:?}", m_mul.coefficients);
}