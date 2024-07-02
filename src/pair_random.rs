use crate::utils::hashed_rand;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

const EMOJI: &'static [&'static str] = &["❤️", "💕", "💜", "💙", "💖", "💗", "🏳️‍🌈", "🏳️‍⚧️"];

pub fn get_pair_random(q: &teloxide::types::InlineQuery) -> teloxide::types::InlineQueryResultArticle {
    let username = q.from.username.as_ref().unwrap().as_str();
    let message = match q.query.trim() {
        "" => String::from("Наскільки ви підходите один одному?"),
        query if query.starts_with("@") => format!("Наскільки ви з {} підходите один одному?", &q.query[1..]),
        _ => format!("Наскільки ви з {} підходите один одному?", q.query),
    };
    let answer = match q.query.trim() {
        "" => InputMessageContent::Text(InputMessageContentText::new("* звуки мовчання *")),
        query if query.starts_with("@") => InputMessageContent::Text(InputMessageContentText::new(format!(
            "{} Ви з {} підходите один одному на {}% {}",
            EMOJI[hashed_rand(&[username, &query[1..]]) as usize % EMOJI.len()].to_string(),
            q.query[1..].to_string(),
            hashed_rand(&[username, &query[1..]]) % 101,
            EMOJI[hashed_rand(&[username, &query[1..]]) as usize % EMOJI.len()].to_string(),
        ))),
        _ => InputMessageContent::Text(InputMessageContentText::new(format!(
            "Ви з {} підходите один одному на {} {}% {}",
            q.query,
            EMOJI[hashed_rand(&[username, &q.query]) as usize % EMOJI.len()],
            hashed_rand(&[username, &q.query]) % 101,
            EMOJI[hashed_rand(&[username, &q.query]) as usize % EMOJI.len()],
        ))),
    };

    let pair_random = InlineQueryResultArticle::new("01", message, answer)
        .description("Обирай собі пару")
        .thumb_url("https://i.imgflip.com/4oqd5v.jpg?a477696".parse().unwrap());

    return pair_random;
}
