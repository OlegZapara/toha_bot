package validation

import (
	"crypto/rand"
	"fmt"
	"math/big"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func NewArticle(update tgbotapi.Update) *tgbotapi.InlineQueryResultArticle {
	randomIndex, _ := rand.Int(rand.Reader, big.NewInt(int64(1e9)))
	randomPhrase := Phrases[randomIndex.Int64()%int64(len(Phrases))]
	randomTest := TestInputs[randomIndex.Int64()%int64(len(TestInputs))]
	article := tgbotapi.NewInlineQueryResultArticle("validation", "Перевірка коду від Ахаладзе", "")
	article.Description = "Наскільки в тебе хороша валідація?"
	article.ThumbURL = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3dgxO2Z75VAR2I2BxGLDUS9er4WvQjk1jSQ&s"
	article.InputMessageContent = tgbotapi.InputTextMessageContent{
		Text:      fmt.Sprintf("%s:\n> %s", randomPhrase, EscapeMarkdownV2(randomTest)),
		ParseMode: "MarkdownV2",
	}
	return &article
}
