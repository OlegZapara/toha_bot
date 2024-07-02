use std::hash::{DefaultHasher, Hash, Hasher};

use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn get_user_query_random(username: &String, query: String) -> u32 {
    let seed = hash_str(&username).wrapping_add(hash_str(&query));
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen()
}

pub fn get_user_random(username: &String) -> u32 {
    let mut rng = StdRng::seed_from_u64(hash_str(&username));
    rng.gen()
}

pub fn get_user_user_random(username1: &String, username2: &String) -> u32 {
    let seed = {
        if username1 < username2 {
            hash_str(&username1).wrapping_add(hash_str(&username2))
        } else {
            hash_str(&username2).wrapping_add(hash_str(&username1))
        }
    };
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen()
}

pub fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
