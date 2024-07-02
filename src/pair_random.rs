use crate::utils::{get_user_query_random, get_user_user_random};
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

pub fn get_pair_random(q: &teloxide::types::InlineQuery) -> teloxide::types::InlineQueryResultArticle {
    let random_emoji = get_random_emoji(q.from.username.as_ref().unwrap(), q.query.clone());
    let message = match q.query.trim() {
        "" => String::from("Наскільки ви підходите один одному?"),
        query if query.starts_with("@") => format!("Наскільки ви з {} підходите один одному?", &q.query[1..]),
        _ => format!("Наскільки ви з {} підходите один одному?", q.query),
    };
    let answer = match q.query.trim() {
        "" => InputMessageContent::Text(InputMessageContentText::new("* звуки мовчання *")),
        query if query.starts_with("@") => InputMessageContent::Text(InputMessageContentText::new(format!(
            "{} Ви з {} підходите один одному на {}% {}",
            random_emoji,
            q.query[1..].to_string(),
            get_user_user_random(q.from.username.as_ref().unwrap(), &q.query[1..].to_string()) % 101,
            random_emoji
        ))),
        _ => InputMessageContent::Text(InputMessageContentText::new(format!(
            "Ви з {} підходите один одному на {} {}% {}",
            q.query,
            random_emoji,
            get_user_query_random(q.from.username.as_ref().unwrap(), q.query.clone()) % 101,
            random_emoji
        ))),
    };

    let pair_random = InlineQueryResultArticle::new("01", message, answer)
        .description("Обирай собі пару")
        .thumb_url("https://i.imgflip.com/4oqd5v.jpg?a477696".parse().unwrap());

    return pair_random;
}

fn get_random_emoji(username: &String, query: String) -> String {
    let random = get_user_query_random(username, query);
    let emoji = vec!["❤️", "💕", "💜", "💙", "💖", "💗", "🏳️‍🌈", "🏳️‍⚧️"];
    emoji[random as usize % emoji.len()].to_string()
}
