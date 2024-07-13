package validation

var Phrases = []string{
	"Введи", "Давай перевіримо", "Давай спробуємо", "Спробуй ввести", "Протестуємо", "Перевіримо",
	"Спробуй це", "Чи впорається твоя програма з", "А ну спробуй", "Пеrевіrимо", "Спrобуємо",
}

var TestInputs = []string{
	"пустий рядок", "пробіл", "5 пробілів", "таб", "просто натисни Enter",
	Repeat("a", 100), Repeat("a", 1000), Repeat("1", 500), Repeat("0", 200) + "1", "1." + Repeat("0", 100) + "1",
	"'; DROP TABLE users; --", "'; SELECT * FROM users; --", " OR '1'='1", " OR '1'='1' --", " OR 1=1", " OR 1=1 --",
	"<script>alert('XSS')</script>", "<img src=x onerror=alert('XSS')>", "'><script>alert('XSS')</script>", "<svg onload=alert('XSS')>",
	"!", "@", "#", "$", "%",
	"&", "<", ">", "&amp;", "&lt;", "&gt;",
	"\\u202E", "\\u200B", "\\uFFFD", "𝓣𝓮𝓼𝓽",
	"999999999999999999999", "-999999999999999999999", "9223372036854775807", "-9223372036854775808",
	"3.141592653589793", "-3.141592653589793", "1.7976931348623157e+308", "-1.7976931348623157e+308",
	"ÅÄÖ", "你好", "こんにちは", "Привіт",
	"../../etc/passwd", "/etc/passwd", "C:\\Windows\\System32",
	"; ls -la", "| cat /etc/passwd", "&& whoami", "|| echo 'Hello'",
	"}{", "[}", "{{}", "(((", "))", "{",
	"test@example.com\nTo: toha@example.com\n\nSpam message",
}
