// This module handles polynomial ring operations over modular integers.

use num::bigint::BigUint;
use num_bigint::RandBigInt;
use num_traits::Zero;
use std::ops::{Add, Mul, Sub};
use num::ToPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub coefficients: Vec<BigUint>,
    pub modulus: BigUint,
}

impl Polynomial {
    // Creates a new polynomial from a vector of coefficients
    pub fn new(coefficients: Vec<BigUint>, modulus: BigUint) -> Self {
        let mut poly = Polynomial { coefficients, modulus };
        poly.reduce();
        poly
    }

    // Creates a zero polynomial of given degree
    pub fn zero(degree: usize, modulus: &BigUint) -> Self {
        Polynomial::new(vec![BigUint::zero(); degree], modulus.clone())
    }

    // Creates a random polynomial of given degree
    pub fn random(degree: usize, modulus: &BigUint, rng: &mut impl rand::Rng) -> Self {
        let coeffs = (0..degree)
            .map(|_| rng.gen_biguint_below(modulus))
            .collect();
        Polynomial::new(coeffs, modulus.clone())
    }

    // Reduces the polynomial coefficients modulo the modulus
    pub fn reduce(&mut self) {
        for c in &mut self.coefficients {
            *c = (&*c % &self.modulus).clone();
        }
    }

    // Returns the degree of the polynomial
    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }

    // Encodes a vector of integers into a polynomial
    pub fn encode(data: &[u64], modulus: &BigUint) -> Self {
        let coeffs = data.iter().map(|&x| BigUint::from(x) % modulus).collect();
        Polynomial::new(coeffs, modulus.clone())
    }

    // Decodes a polynomial back into a vector of integers
    pub fn decode(&self) -> Vec<u64> {
        self.coefficients
            .iter()
            .map(|c| {
                if let Some(value) = c.to_u64() {
                    value
                } else {
                    panic!("Coefficient exceeds u64 range: {}", c);
                }
            })
            .collect()
    }

    // Encodes a real number into a polynomial using fixed-point representation
    pub fn encode_real(data: f64, precision: usize, modulus: &BigUint) -> Self {
        let scale = 10u64.pow(precision as u32);
        let scaled_data = (data * scale as f64).round() as i64;
        let coeffs = vec![BigUint::from(scaled_data.abs() as u64) % modulus];
        Polynomial::new(coeffs, modulus.clone())
    }

    // Decodes a polynomial back into a real number using fixed-point representation
    pub fn decode_real(&self, precision: usize) -> f64 {
        if let Some(c) = self.coefficients.get(0) {
            let scale = 10u64.pow(precision as u32);
            if let Some(value) = c.to_u64() {
                value as f64 / scale as f64
            } else {
                panic!("Coefficient exceeds u64 range: {}", c);
            }
        } else {
            0.0
        }
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    // Adds two polynomials
    fn add(self, rhs: Polynomial) -> Polynomial {
        let len = self.coefficients.len().max(rhs.coefficients.len());
        let mut coeffs = vec![BigUint::zero(); len];
        let zero = BigUint::zero();
        for i in 0..len {
            let a = self.coefficients.get(i).unwrap_or(&zero);
            let b = rhs.coefficients.get(i).unwrap_or(&zero);
            coeffs[i] = (a + b) % &self.modulus;
        }
        Polynomial::new(coeffs, self.modulus.clone())
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    // Subtracts two polynomials
    fn sub(self, rhs: Polynomial) -> Polynomial {
        let len = self.coefficients.len().max(rhs.coefficients.len());
        let mut coeffs = vec![BigUint::zero(); len];
        let zero = BigUint::zero();
        for i in 0..len {
            let a = self.coefficients.get(i).unwrap_or(&zero);
            let b = rhs.coefficients.get(i).unwrap_or(&zero);
            coeffs[i] = ((a + &self.modulus) - b) % &self.modulus;
        }
        Polynomial::new(coeffs, self.modulus.clone())
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    // Multiplies two polynomials
    fn mul(self, rhs: Polynomial) -> Polynomial {
        let n = self.coefficients.len();
        let mut coeffs = vec![BigUint::zero(); n];
        for i in 0..n {
            for j in 0..n {
                let idx = (i + j) % n;
                coeffs[idx] = (&coeffs[idx] + (&self.coefficients[i] * &rhs.coefficients[j])) % &self.modulus;
            }
        }
        Polynomial::new(coeffs, self.modulus.clone())
    }
}

// Helper to create polynomial from u64 coefficients
impl Polynomial {
    pub fn from_u64(coeffs: &[u64], modulus: &BigUint) -> Self {
        let coeffs = coeffs.iter().map(|&c| BigUint::from(c) % modulus).collect();
        Polynomial::new(coeffs, modulus.clone())
    }
}