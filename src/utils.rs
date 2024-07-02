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

pub fn get_random_emoji(username: &String, query: String) -> String {
    let random = get_user_query_random(username, query);
    let emoji = vec!["❤️", "💕", "💜", "💙", "💖", "💗", "🏳️‍🌈", "🏳️‍⚧️"];
    emoji[random as usize % emoji.len()].to_string()
}

pub fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
