use dotenv::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    env,
    hash::{DefaultHasher, Hash, Hasher},
};
use teloxide::{
    prelude::*,
    types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::new(env::var("TELOXIDE_TOKEN").unwrap());

    let handler = Update::filter_inline_query().branch(dptree::endpoint(|bot: Bot, q: InlineQuery| async move {
        let random_emoji = get_random_emoji(q.from.username.clone().unwrap(), q.query.clone());
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
                get_user_random(q.from.username.unwrap(), q.query.clone()) % 101,
                random_emoji
            ))),
        };

        let pair_random = InlineQueryResultArticle::new("01".to_string(), message, answer)
            .description("Обирай собі пару")
            .thumb_url("https://i.imgflip.com/4oqd5v.jpg?a477696".parse().unwrap());
        let results = vec![InlineQueryResult::Article(pair_random)];
        let response = bot.answer_inline_query(&q.id, results).cache_time(0).send().await;
        if let Err(err) = response {
            log::error!("Error in handler: {:?}", err);
        }
        respond(())
    }));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn get_user_random(username: String, query: String) -> u32 {
    let seed = hash_str(&username).wrapping_add(hash_str(&query));
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen()
}

fn get_random_emoji(username: String, query: String) -> String {
    let random = get_user_random(username, query);
    let emoji = vec!["❤️", "💕", "💜", "💙", "💖", "💗", "💕", "️‍🌈", "🏳️‍🌈", "🏳️‍⚧️"];
    emoji[random as usize % emoji.len()].to_string()
}

fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
