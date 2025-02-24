// qrng.rs - Quantum Random Number Generation
extern crate rand_chacha;
use rand_chacha::ChaChaRng;
use rand::SeedableRng;

/// Generiert eine Zufallszahl basierend auf einem optionalen Seed.
pub fn generate_random_number(seed: Option<u64>) -> u64 {
    let mut rng = match seed {
        Some(s) => ChaChaRng::seed_from_u64(s),
        None => ChaChaRng::from_entropy(),
    };
    rng.gen()
}