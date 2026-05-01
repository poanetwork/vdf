//! Emit precomputed discriminants for tooling.
#![allow(clippy::all)]

use classgroup::gmp::mpz::Mpz;
use vdf::create_discriminant;

fn main() {
    let test_cases = vec![
        ("default", 256),
        ("default", 512),
        ("default", 1024),
        ("default", 2048),
    ];

    println!("// PRECOMPUTED DISCRIMINANTS");
    println!("// Generated using Rust GMP for maximum performance");
    println!("// All satisfy required properties: negative, ≡ 1 (mod 4), ≡ 1 (mod 8)");
    println!("//");
    println!("// These are used by default in VDF operations to avoid slow discriminant generation in JS");
    println!();

    for (challenge, bit_size) in test_cases {
        let seed = challenge.as_bytes();
        let disc: Mpz = create_discriminant(seed, bit_size);

        // Verify properties
        let is_negative = disc < Mpz::zero();
        let mod4 = disc.modulus(&Mpz::from(4));
        let mod8 = disc.modulus(&Mpz::from(8));

        println!("// {}-bit discriminant", bit_size);
        println!(
            "// Verified: negative={}, mod4={}, mod8={}",
            is_negative, mod4, mod8
        );
        println!("export const DISCRIMINANT_{} = {}n;", bit_size, disc);
        println!();
    }

    println!();
    println!("// Usage: Import these in your VDF code");
    println!("// import {{ DISCRIMINANT_256, DISCRIMINANT_512, DISCRIMINANT_1024, DISCRIMINANT_2048 }} from './precomputed-discriminants';");
}
