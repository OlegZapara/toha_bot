import { Context, NarrowedContext } from "telegraf";
import {
  InlineQueryResultArticle,
  Update,
} from "telegraf/typings/core/types/typegram";
import { phrases, tests } from "../constants/validation-data";

export const validationArticle = (
  ctx: NarrowedContext<Context<Update>, Update.InlineQueryUpdate>
): InlineQueryResultArticle => {
  const phrase = phrases[Math.floor(Math.random() * phrases.length)];
  const test = tests[Math.floor(Math.random() * tests.length)];
  return {
    type: "article",
    id: "validation",
    title: "Перевірка коду від Ахаладзе",
    input_message_content: {
      message_text: `${phrase}:\n> ${test}`,
      parse_mode: "MarkdownV2",
    },
    description: "Наскільки в тебе хороша валідація?",
    thumbnail_url:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3dgxO2Z75VAR2I2BxGLDUS9er4WvQjk1jSQ&s",
  };
};
