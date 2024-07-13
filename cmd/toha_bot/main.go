package main

import (
	"log"
	"os"
	"toha_bot/internal/compatibility"
	"toha_bot/internal/validation"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
)

func main() {
	godotenv.Load(".env")
	token := os.Getenv("BOT_TOKEN")
	bot, err := tgbotapi.NewBotAPI(token)
	if err != nil {
		log.Panic(err)
	}
	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60

	updates := bot.GetUpdatesChan(u)
	for update := range updates {
		if update.InlineQuery == nil {
			continue
		}
		compatibility := compatibility.NewArticle(update)
		validation := validation.NewArticle(update)

		inlineConf := tgbotapi.InlineConfig{
			InlineQueryID: update.InlineQuery.ID,
			IsPersonal:    true,
			CacheTime:     1,
			Results:       []interface{}{compatibility, validation},
		}

		if _, err := bot.Request(inlineConf); err != nil {
			log.Println(err)
		}
	}
}
