use std::hash::{DefaultHasher, Hash, Hasher};

use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn hashed_rand(values: &[&str]) -> u64 {
    let seed = values.iter().fold(0u64, |acc: u64, s| acc.wrapping_add(hash_str(s)));
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen()
}
