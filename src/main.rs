use dck_random::get_dck_random;
use dotenv::dotenv;
use pair_random::get_pair_random;
use std::env;
use teloxide::{prelude::*, types::InlineQueryResult};

pub mod dck_random;
pub mod pair_random;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::new(env::var("TELOXIDE_TOKEN").unwrap());

    let handler = Update::filter_inline_query().branch(dptree::endpoint(|bot: Bot, q: InlineQuery| async move {
        let results = vec![
            InlineQueryResult::Article(get_pair_random(&q)),
            InlineQueryResult::Article(get_dck_random(&q)),
        ];
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
