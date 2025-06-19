#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn test_encode_decode_integers() {
        let modulus = BigUint::from_u64(97).unwrap();
        let data = vec![1, 2, 3, 4, 5];
        let poly = Polynomial::encode(&data, &modulus);
        let decoded = poly.decode();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_encode_decode_real() {
        let modulus = BigUint::from_u64(97).unwrap();
        let data = 3.14159;
        let precision = 5;
        let poly = Polynomial::encode_real(data, precision, &modulus);
        let decoded = poly.decode_real(precision);
        assert!((data - decoded).abs() < 1e-5);
    }
}
