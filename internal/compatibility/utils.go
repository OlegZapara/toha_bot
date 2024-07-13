package compatibility

var emoji = []string{"🔥", "💔", "💖", "💘", "💗", "💓", "💕", "💞", "💝", "💟"}

func RandomEmoji(seed uint32) string {
	return emoji[seed%uint32(len(emoji))]
}
