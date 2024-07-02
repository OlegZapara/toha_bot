use std::hash::{DefaultHasher, Hash, Hasher};

use rand::{rngs::StdRng, Rng, SeedableRng};
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

pub fn get_pair_random(q: &teloxide::types::InlineQuery) -> teloxide::types::InlineQueryResultArticle {
    let random_emoji = get_random_emoji(q.from.username.as_ref().unwrap(), q.query.clone());
    let message = match q.query.trim() {
        "" => String::from("Наскільки ви підходите один одному?"),
        _ => format!("Наскільки ви з {} підходите один одному?", q.query),
    };
    let answer = match q.query.trim() {
        "" => InputMessageContent::Text(InputMessageContentText::new("* звуки мовчання *")),
        _ => InputMessageContent::Text(InputMessageContentText::new(format!(
            "Ви з {} підходите один одному на {} {}% {}",
            q.query,
            random_emoji,
            get_user_random(q.from.username.as_ref().unwrap(), q.query.clone()) % 101,
            random_emoji
        ))),
    };

    let pair_random = InlineQueryResultArticle::new("01".to_string(), message, answer)
        .description("Обирай собі пару")
        .thumb_url("https://i.imgflip.com/4oqd5v.jpg?a477696".parse().unwrap());

    return pair_random;
}

fn get_user_random(username: &String, query: String) -> u32 {
    let seed = hash_str(&username).wrapping_add(hash_str(&query));
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen()
}

fn get_random_emoji(username: &String, query: String) -> String {
    let random = get_user_random(username, query);
    let emoji = vec!["❤️", "💕", "💜", "💙", "💖", "💗", "🏳️‍🌈", "🏳️‍⚧️"];
    emoji[random as usize % emoji.len()].to_string()
}

fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
