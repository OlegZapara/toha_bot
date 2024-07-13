package compatibility

import (
	"fmt"
	"strings"
	pkg "toha_bot/pkg"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func NewArticle(update tgbotapi.Update) *tgbotapi.InlineQueryResultArticle {
	query := update.InlineQuery.Query
	var title, message string
	if strings.TrimSpace(query) != "" {
		probability := pkg.CompatibilityHash(update.InlineQuery.From.UserName, query) % 101
		title = fmt.Sprintf("Наскільки ви з %s підходите один одному?", query)
		emoji := RandomEmoji(probability)
		message = fmt.Sprintf("Ви з %s підходите один одному на %s %d%% %s", query, emoji, probability, emoji)
	} else {
		title = "Наскільки ви підходите один одному?"
		message = "* звуки тиші *"
	}

	article := tgbotapi.NewInlineQueryResultArticle("compatibility", title, message)
	article.Description = "Обирай собі пару"
	article.ThumbURL = "https://i.imgflip.com/4oqd5v.jpg?a477696"
	return &article
}
