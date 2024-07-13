import dotenv from "dotenv";
import { Telegraf } from "telegraf";
import { message } from "telegraf/filters";
import { InlineQueryResultArticle } from "telegraf/typings/core/types/typegram";
import { compatibilityArticle } from "./inline-articles/compatibility";
import { validationArticle } from "./inline-articles/validation";
dotenv.config();

const bot = new Telegraf(process.env.BOT_TOKEN!);
bot.start((ctx) => ctx.reply("Welcome"));
bot.help((ctx) => ctx.reply("Send me a sticker"));
bot.on(message("sticker"), (ctx) => ctx.reply("👍"));
bot.on("inline_query", (ctx) => {
  try {
    const result: InlineQueryResultArticle[] = [
      compatibilityArticle(ctx),
      validationArticle(ctx),
    ];
    ctx.answerInlineQuery(result, { cache_time: 0 });
  } catch (err) {
    console.error(err);
  }
});
bot.hears("hi", (ctx) => ctx.reply("Hey there"));
bot.launch();

process.once("SIGINT", () => bot.stop("SIGINT"));
process.once("SIGTERM", () => bot.stop("SIGTERM"));
