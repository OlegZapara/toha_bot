import { Context, NarrowedContext } from "telegraf";
import {
  InlineQueryResultArticle,
  Update,
} from "telegraf/typings/core/types/typegram";
import { hash } from "../utils/hash";
import { emoji } from "../constants/compatibility";

export const compatibilityArticle = (
  ctx: NarrowedContext<Context<Update>, Update.InlineQueryUpdate>
): InlineQueryResultArticle => {
  let query = ctx.update.inline_query.query;
  let title: string, message: string;
  if (query.trim()) {
    if (query.charAt(0) === "@") query = query.slice(1);
    const probability = Math.abs(hash(ctx.from.username ?? "", query) % 101);
    title = `Наскільки ви з ${query} підходите один одному?`;
    const e = emoji[probability % emoji.length];
    message = `Ви з ${query} підходите один одному на ${e} ${probability}% ${e}`;
  } else {
    title = "Наскільки ви підходите один одному?";
    message = "* звуки тиші *";
  }
  return {
    type: "article",
    id: "compatibility",
    title,
    input_message_content: { message_text: message },
    description: "Обирай собі пару",
    thumbnail_url: "https://i.imgflip.com/4oqd5v.jpg?a477696",
  };
};
