use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Русский (Russian)
pub static STOPWORDS_RUS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "а",
        "алло",
        "без",
        "близко",
        "более",
        "больше",
        "будем",
        "будет",
        "будете",
        "будешь",
        "будто",
        "буду",
        "будут",
        "будь",
        "бы",
        "бывает",
        "бывь",
        "был",
        "была",
        "были",
        "было",
        "быть",
        "в",
        "важная",
        "важное",
        "важные",
        "важный",
        "вам",
        "вами",
        "вас",
        "ваш",
        "ваша",
        "ваше",
        "ваши",
        "вверх",
        "вдали",
        "вдруг",
        "ведь",
        "везде",
        "весь",
        "вниз",
        "внизу",
        "во",
        "вокруг",
        "вон",
        "восемнадцатый",
        "восемнадцать",
        "восемь",
        "восьмой",
        "вот",
        "впрочем",
        "времени",
        "время",
        "все",
        "всё",
        "всегда",
        "всего",
        "всем",
        "всеми",
        "всему",
        "всех",
        "всею",
        "всю",
        "всюду",
        "вся",
        "второй",
        "вы",
        "г",
        "где",
        "говорил",
        "говорит",
        "год",
        "года",
        "году",
        "да",
        "давно",
        "даже",
        "далеко",
        "дальше",
        "даром",
        "два",
        "двадцатый",
        "двадцать",
        "две",
        "двенадцатый",
        "двенадцать",
        "двух",
        "девятнадцатый",
        "девятнадцать",
        "девятый",
        "девять",
        "действительно",
        "дел",
        "день",
        "десятый",
        "десять",
        "для",
        "до",
        "довольно",
        "долго",
        "должно",
        "другая",
        "другие",
        "других",
        "друго",
        "другое",
        "другой",
        "е",
        "его",
        "ее",
        "её",
        "ей",
        "ему",
        "если",
        "есть",
        "еще",
        "ещё",
        "ею",
        "ж",
        "же",
        "жизнь",
        "за",
        "занят",
        "занята",
        "занято",
        "заняты",
        "затем",
        "зато",
        "зачем",
        "здесь",
        "значит",
        "и",
        "из",
        "или",
        "им",
        "именно",
        "иметь",
        "ими",
        "имя",
        "иногда",
        "их",
        "к",
        "каждая",
        "каждое",
        "каждые",
        "каждый",
        "кажется",
        "как",
        "какая",
        "какой",
        "кем",
        "когда",
        "кого",
        "ком",
        "кому",
        "конечно",
        "которая",
        "которого",
        "которой",
        "которые",
        "который",
        "которых",
        "кроме",
        "кругом",
        "кто",
        "куда",
        "лет",
        "ли",
        "лишь",
        "лучше",
        "люди",
        "м",
        "мало",
        "между",
        "меля",
        "менее",
        "меньше",
        "меня",
        "миллионов",
        "мимо",
        "мира",
        "мне",
        "много",
        "многочисленная",
        "многочисленное",
        "многочисленные",
        "многочисленный",
        "мной",
        "мною",
        "мог",
        "могут",
        "моё",
        "мож",
        "может",
        "можно",
        "можхо",
        "мои",
        "мой",
        "мор",
        "мочь",
        "моя",
        "мы",
        "на",
        "наверху",
        "над",
        "надо",
        "назад",
        "наиболее",
        "наконец",
        "нам",
        "нами",
        "нас",
        "начала",
        "наш",
        "наша",
        "наше",
        "наши",
        "не",
        "него",
        "недавно",
        "недалеко",
        "нее",
        "неё",
        "ней",
        "нельзя",
        "нем",
        "немного",
        "нему",
        "непрерывно",
        "нередко",
        "несколько",
        "нет",
        "нею",
        "ни",
        "нибудь",
        "ниже",
        "низко",
        "никогда",
        "никуда",
        "ними",
        "них",
        "ничего",
        "но",
        "ну",
        "нужно",
        "нх",
        "о",
        "об",
        "оба",
        "обычно",
        "один",
        "одиннадцатый",
        "одиннадцать",
        "однажды",
        "однако",
        "одного",
        "одной",
        "около",
        "он",
        "она",
        "они",
        "оно",
        "опять",
        "особенно",
        "от",
        "отовсюду",
        "отсюда",
        "очень",
        "первый",
        "перед",
        "по",
        "под",
        "пожалуйста",
        "позже",
        "пока",
        "пор",
        "пора",
        "после",
        "посреди",
        "потом",
        "потому",
        "почему",
        "почти",
        "прекрасно",
        "при",
        "про",
        "просто",
        "против",
        "процентов",
        "пятнадцатый",
        "пятнадцать",
        "пятый",
        "пять",
        "раз",
        "разве",
        "рано",
        "раньше",
        "рядом",
        "с",
        "сам",
        "сама",
        "сами",
        "самим",
        "самими",
        "самих",
        "само",
        "самого",
        "самой",
        "самом",
        "самому",
        "саму",
        "свое",
        "своего",
        "своей",
        "свои",
        "своих",
        "свою",
        "сеаой",
        "себе",
        "себя",
        "сегодня",
        "седьмой",
        "сейчас",
        "семнадцатый",
        "семнадцать",
        "семь",
        "сих",
        "сказал",
        "сказала",
        "сказать",
        "сколько",
        "слишком",
        "сначала",
        "снова",
        "со",
        "собой",
        "собою",
        "совсем",
        "спасибо",
        "стал",
        "суть",
        "т",
        "та",
        "так",
        "такая",
        "также",
        "такие",
        "такое",
        "такой",
        "там",
        "твоё",
        "твой",
        "твоя",
        "те",
        "тебе",
        "тебя",
        "тем",
        "теми",
        "теперь",
        "тех",
        "то",
        "тобой",
        "тобою",
        "тогда",
        "того",
        "тоже",
        "только",
        "том",
        "тому",
        "тот",
        "тою",
        "третий",
        "три",
        "тринадцатый",
        "тринадцать",
        "ту",
        "туда",
        "тут",
        "ты",
        "тысяч",
        "у",
        "уж",
        "уже",
        "уметь",
        "хорошо",
        "хотеть",
        "хоть",
        "хотя",
        "хочешь",
        "часто",
        "чаще",
        "чего",
        "человек",
        "чем",
        "чему",
        "через",
        "четвертый",
        "четыре",
        "четырнадцатый",
        "четырнадцать",
        "что",
        "чтоб",
        "чтобы",
        "чуть",
        "шестнадцатый",
        "шестнадцать",
        "шестой",
        "шесть",
        "эта",
        "эти",
        "этим",
        "этими",
        "этих",
        "это",
        "этого",
        "этой",
        "этом",
        "этому",
        "этот",
        "эту",
        "я",
    ]
    .iter()
    .cloned()
    .collect()
});
