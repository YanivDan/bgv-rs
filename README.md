# BGV Homomorphic Encryption in Rust

This project implements the Brakerski-Gentry-Vaikuntanathan (BGV) homomorphic encryption scheme in Rust. The BGV scheme allows computations to be performed on encrypted data without needing to decrypt it first, enabling privacy-preserving computations.

## Dependencies

This project uses the following dependencies:
- `num`: For handling big integers and modular arithmetic.
- `rand`: For random number generation during key generation.
- `rug`: For arbitrary precision arithmetic (if needed).

## Building the Project

To build the project, ensure you have Rust installed on your machine. You can build the project using the following command:

```
cargo build
```

## Running the Project

To run the project and see the BGV homomorphic encryption in action, use the following command:

```
cargo run
```

## Example Usage

The `main.rs` file contains an example that demonstrates the following steps:
1. Key generation to create public and secret keys.
2. Encoding messages into plaintext polynomials.
3. Encrypting the plaintext polynomials to obtain ciphertexts.
4. Performing homomorphic addition and multiplication on the ciphertexts.
5. Decrypting the resulting ciphertexts to recover the original messages.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.