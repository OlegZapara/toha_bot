use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

use crate::utils::hashed_rand;

const EMOJI: &'static [&'static str] = &["🌶️", "🥕", "🍆", "🍌", "🥖", "🌭"];

pub fn get_dck_random(q: &teloxide::types::InlineQuery) -> teloxide::types::InlineQueryResultArticle {
    let username = q.from.username.as_ref().unwrap();
    let random_value = hashed_rand(&[username]);
    let answer = match random_value % 100 {
        0..=10 => "У тебе немає члена 🌚".to_string(),
        11..=20 => "👆 Найменший член в групі".to_string(),
        21..=90 => format!(
            "{} Твій пісюн - {}см",
            EMOJI[(random_value as usize) % EMOJI.len()],
            ((random_value % 3000) as f64) / 100.0 + 1.0
        ),
        _ => "🔥 Найбільший член в групі 🔥".to_string(),
    };
    let dck_random = InlineQueryResultArticle::new(
        "02",
        "Якого розміру в тебе пісюн?",
        InputMessageContent::Text(InputMessageContentText::new(answer)),
    )
    .description("Роздівайся")
    .thumb_url(
        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTCr6MmVDOTyOxj9OH8iUBZOB78-1InmDBBFg&s"
            .parse()
            .unwrap(),
    );

    return dck_random;
}
