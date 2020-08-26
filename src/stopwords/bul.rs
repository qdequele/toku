use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Български (Bulgarian)
pub static STOPWORDS_BUL: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "ð°",
        "ð°ð²ñ‚ðµð½ñ‚ð¸ñ‡ðµð½",
        "ð°ð·",
        "ð°ðºð¾",
        "ð°ð»ð°",
        "ð±ðµ",
        "ð±ðµð·",
        "ð±ðµñˆðµ",
        "ð±ð¸",
        "ð±ð¸ð²ñˆ",
        "ð±ð¸ð²ñˆð°",
        "ð±ð¸ð²ñˆð¾",
        "ð±ð¸ð»",
        "ð±ð¸ð»ð°",
        "ð±ð¸ð»ð¸",
        "ð±ð¸ð»ð¾",
        "ð±ð»ð°ð³ð¾ð´ð°ñ€ñ",
        "ð±ð»ð¸ð·ð¾",
        "ð±ññ…ð°",
        "ð±ñšð´ð°ñ‚",
        "ð±ñšð´ðµ",
        "ð²",
        "ð²ð°ñ",
        "ð²ð°ñˆ",
        "ð²ð°ñˆð°",
        "ð²ðµñ‡ðµ",
        "ð²ðµñ€ð¾ññ‚ð½ð¾",
        "ð²ð·ðµð¼ð°",
        "ð²ð¸",
        "ð²ð¸ðµ",
        "ð²ð¸ð½ð°ð³ð¸",
        "ð²ð½ð¸ð¼ð°ð²ð°",
        "ð²ñðµ",
        "ð²ñðµðºð¸",
        "ð²ñð¸ñ‡ðºð¸",
        "ð²ñð¸ñ‡ðºð¾",
        "ð²ññðºð°",
        "ð²ñšð²",
        "ð²ñšð¿ñ€ðµðºð¸",
        "ð²ñšñ€ñ…ñƒ",
        "ð²ñ€ðµð¼ðµ",
        "ð³",
        "ð³ð¸",
        "ð³ð»ð°ð²ðµð½",
        "ð³ð»ð°ð²ð½ð°",
        "ð³ð»ð°ð²ð½ð¾",
        "ð³ð»ð°ñ",
        "ð³ð¾",
        "ð³ð¾ð´ð¸ð½ð°",
        "ð³ð¾ð´ð¸ð½ð¸",
        "ð³ð¾ð´ð¸ñˆðµð½",
        "ð´",
        "ð´ð°",
        "ð´ð°ð»ð¸",
        "ð´ð²ð°",
        "ð´ð²ð°ð¼ð°",
        "ð´ð²ð°ð¼ð°ñ‚ð°",
        "ð´ð²ðµ",
        "ð´ð²ðµñ‚ðµ",
        "ð´ðµð½",
        "ð´ð½ðµñ",
        "ð´ð½ð¸",
        "ð´ð¾",
        "ð´ð¾ð±ñšñ€",
        "ð´ð¾ð±ñ€ð°",
        "ð´ð¾ð±ñ€ðµ",
        "ð´ð¾ð±ñ€ð¾",
        "ð´ð¾ðºð°ñ‚ð¾",
        "ð´ð¾ðºð¾ð³ð°",
        "ð´ð¾ñðµð³ð°",
        "ð´ð¾ññ‚ð°",
        "ð´ð¾ñ€ð¸",
        "ð´ñ€ñƒð³",
        "ð´ñ€ñƒð³ð°",
        "ð´ñ€ñƒð³ð¸",
        "ðµ",
        "ðµð²ñ‚ð¸ð½",
        "ðµð´ð²ð°",
        "ðµð´ð¸ð½",
        "ðµð´ð½ð°",
        "ðµð´ð½ð°ðºð²ð°",
        "ðµð´ð½ð°ðºð²ð¸",
        "ðµð´ð½ð°ðºñšð²",
        "ðµð´ð½ð¾",
        "ðµðºð¸ð¿",
        "ðµñ‚ð¾",
        "ð¶ð¸ð²ð¾ñ‚",
        "ð·ð°",
        "ð·ð°ð±ð°ð²ñð¼",
        "ð·ð°ð´",
        "ð·ð°ðµð´ð½ð¾",
        "ð·ð°ñðµð³ð°",
        "ð·ð°ñð¿ð°ð»",
        "ð·ð°ñ‚ð¾ð²ð°",
        "ð·ð°ñ‰ð¾",
        "ð·ð°ñ‰ð¾ñ‚ð¾",
        "ð·ð°ñ€ð°ð´ð¸",
        "ð¸",
        "ð¸ð·",
        "ð¸ð»ð¸",
        "ð¸ð¼",
        "ð¸ð¼ð°",
        "ð¸ð¼ð°ñ‚",
        "ð¸ñðºð°",
        "ð¹",
        "ðºð°ð·ð°",
        "ðºð°ðº",
        "ðºð°ðºð²ð°",
        "ðºð°ðºð²ð¾",
        "ðºð°ðºñšð²",
        "ðºð°ðºñ‚ð¾",
        "ðºð°ñ‚ð¾",
        "ðºð¾ð³ð°",
        "ðºð¾ð³ð°ñ‚ð¾",
        "ðºð¾ðµñ‚ð¾",
        "ðºð¾ð¸ñ‚ð¾",
        "ðºð¾ð¹",
        "ðºð¾ð¹ñ‚ð¾",
        "ðºð¾ð»ðºð¾",
        "ðºð¾ññ‚ð¾",
        "ðºñšð´ðµ",
        "ðºñšð´ðµñ‚ð¾",
        "ðºñšð¼",
        "ð»ðµñðµð½",
        "ð»ðµñð½ð¾",
        "ð»ð¸",
        "ð»ð¾ñˆ",
        "ð¼",
        "ð¼ð°ð¹",
        "ð¼ð°ð»ðºð¾",
        "ð¼ðµ",
        "ð¼ðµð¶ð´ñƒ",
        "ð¼ðµðº",
        "ð¼ðµð½",
        "ð¼ðµñðµñ†",
        "ð¼ð¸",
        "ð¼ð½ð¾ð³ð¾",
        "ð¼ð½ð¾ð·ð¸ð½ð°",
        "ð¼ð¾ð³ð°",
        "ð¼ð¾ð³ð°ñ‚",
        "ð¼ð¾ð¶ðµ",
        "ð¼ð¾ðºñšñ€",
        "ð¼ð¾ð»ñ",
        "ð¼ð¾ð¼ðµð½ñ‚ð°",
        "ð¼ñƒ",
        "ð½",
        "ð½ð°",
        "ð½ð°ð´",
        "ð½ð°ð·ð°ð´",
        "ð½ð°ð¹",
        "ð½ð°ð¿ñ€ð°ð²ð¸",
        "ð½ð°ð¿ñ€ðµð´",
        "ð½ð°ð¿ñ€ð¸ð¼ðµñ€",
        "ð½ð°ñ",
        "ð½ðµ",
        "ð½ðµð³ð¾",
        "ð½ðµñ",
        "ð½ðµñ‰ð¾",
        "ð½ð¸",
        "ð½ð¸ðµ",
        "ð½ð¸ðºð¾ð¹",
        "ð½ð¸ñ‚ð¾",
        "ð½ð¸ñ‰ð¾",
        "ð½ð¾",
        "ð½ð¾ð²",
        "ð½ð¾ð²ð°",
        "ð½ð¾ð²ð¸",
        "ð½ð¾ð²ð¸ð½ð°",
        "ð½ñðºð¾ð¸",
        "ð½ñðºð¾ð¹",
        "ð½ñðºð¾ð»ðºð¾",
        "ð½ñð¼ð°",
        "ð¾ð±ð°ñ‡ðµ",
        "ð¾ðºð¾ð»ð¾",
        "ð¾ñð²ðµð½",
        "ð¾ñð¾ð±ðµð½ð¾",
        "ð¾ñ‚",
        "ð¾ñ‚ð³ð¾ñ€ðµ",
        "ð¾ñ‚ð½ð¾ð²ð¾",
        "ð¾ñ‰ðµ",
        "ð¿ð°ðº",
        "ð¿ð¾",
        "ð¿ð¾ð²ðµñ‡ðµ",
        "ð¿ð¾ð²ðµñ‡ðµñ‚ð¾",
        "ð¿ð¾ð´",
        "ð¿ð¾ð½ðµ",
        "ð¿ð¾ñð»ðµ",
        "ð¿ð¾ñ‡ñ‚ð¸",
        "ð¿ð¾ñ€ð°ð´ð¸",
        "ð¿ñšðº",
        "ð¿ñšñ‚ð¸",
        "ð¿ñšñ€ð²ð°ñ‚ð°",
        "ð¿ñšñ€ð²ð¸",
        "ð¿ñšñ€ð²ð¾",
        "ð¿ñ€ð°ð²ð¸",
        "ð¿ñ€ðµð´",
        "ð¿ñ€ðµð´ð¸",
        "ð¿ñ€ðµð·",
        "ð¿ñ€ð¸",
        "ñ",
        "ñð°",
        "ñð°ð¼",
        "ñð°ð¼ð¾",
        "ñðµ",
        "ñðµð³ð°",
        "ñð¸",
        "ñð¸ð½",
        "ñðºð¾ñ€ð¾",
        "ñð»ðµð´",
        "ñð»ðµð´ð²ð°ñ‰",
        "ñð¼ðµ",
        "ñð¼ññ…",
        "ñð¿ð¾ñ€ðµð´",
        "ññšð¼",
        "ññšñ",
        "ññšñ‰ð¾",
        "ññ‚ðµ",
        "ññ€ðµð´",
        "ññ€ðµñ‰ñƒ",
        "ñ",
        "ñðº",
        "ñžð¼ñ€ñƒðº",
        "ñƒ",
        "ñƒñ‚ñ€ðµ",
        "ñ‚",
        "ñ‚.ð½.",
        "ñ‚ð°ð·ð¸",
        "ñ‚ð°ðºð°",
        "ñ‚ð°ðºð¸ð²ð°",
        "ñ‚ð°ðºñšð²",
        "ñ‚ð°ð¼",
        "ñ‚ð²ð¾ð¹",
        "ñ‚ðµ",
        "ñ‚ðµð·ð¸",
        "ñ‚ð¸",
        "ñ‚ð¾",
        "ñ‚ð¾ð²ð°",
        "ñ‚ð¾ð³ð°ð²ð°",
        "ñ‚ð¾ð·ð¸",
        "ñ‚ð¾ð¹",
        "ñ‚ð¾ð»ðºð¾ð²ð°",
        "ñ‚ð¾ñ‡ð½ð¾",
        "ñ‚ñ",
        "ñ‚ññ…",
        "ñ‚ñšð¹",
        "ñ‚ñƒðº",
        "ñ‚ñ€ð¸",
        "ñ‚ñ€ñð±ð²ð°",
        "ñ‡",
        "ñ‡ð°ñð°",
        "ñ‡ðµ",
        "ñ‡ðµññ‚ð¾",
        "ñ‡ñ€ðµð·",
        "ñ…ð°ñ€ðµñð²ð°",
        "ñ…ð¸ð»ñð´ð¸",
        "ñ‰ðµ",
        "ñ‰ð¾ð¼",
        "ñ€ð°ð²ðµð½",
        "ñ€ð°ð²ð½ð°",
        "а",
        "автентичен",
        "аз",
        "ако",
        "ала",
        "бе",
        "без",
        "беше",
        "би",
        "бивш",
        "бивша",
        "бившо",
        "бил",
        "била",
        "били",
        "било",
        "благодаря",
        "близо",
        "бъдат",
        "бъде",
        "бяха",
        "в",
        "вас",
        "ваш",
        "ваша",
        "вероятно",
        "вече",
        "взема",
        "ви",
        "вие",
        "винаги",
        "внимава",
        "време",
        "все",
        "всеки",
        "всички",
        "всичко",
        "всяка",
        "във",
        "въпреки",
        "върху",
        "г",
        "ги",
        "главен",
        "главна",
        "главно",
        "глас",
        "го",
        "година",
        "години",
        "годишен",
        "д",
        "да",
        "дали",
        "два",
        "двама",
        "двамата",
        "две",
        "двете",
        "ден",
        "днес",
        "дни",
        "до",
        "добра",
        "добре",
        "добро",
        "добър",
        "докато",
        "докога",
        "дори",
        "досега",
        "доста",
        "друг",
        "друга",
        "други",
        "е",
        "евтин",
        "едва",
        "един",
        "една",
        "еднаква",
        "еднакви",
        "еднакъв",
        "едно",
        "екип",
        "ето",
        "живот",
        "за",
        "забавям",
        "зад",
        "заедно",
        "заради",
        "засега",
        "заспал",
        "затова",
        "защо",
        "защото",
        "и",
        "из",
        "или",
        "им",
        "има",
        "имат",
        "иска",
        "й",
        "каза",
        "как",
        "каква",
        "какво",
        "както",
        "какъв",
        "като",
        "кога",
        "когато",
        "което",
        "които",
        "кой",
        "който",
        "колко",
        "която",
        "къде",
        "където",
        "към",
        "лесен",
        "лесно",
        "ли",
        "лош",
        "м",
        "май",
        "малко",
        "ме",
        "между",
        "мек",
        "мен",
        "месец",
        "ми",
        "много",
        "мнозина",
        "мога",
        "могат",
        "може",
        "мокър",
        "моля",
        "момента",
        "му",
        "н",
        "на",
        "над",
        "назад",
        "най",
        "направи",
        "напред",
        "например",
        "нас",
        "не",
        "него",
        "нещо",
        "нея",
        "ни",
        "ние",
        "никой",
        "нито",
        "нищо",
        "но",
        "нов",
        "нова",
        "нови",
        "новина",
        "някои",
        "някой",
        "няколко",
        "няма",
        "обаче",
        "около",
        "освен",
        "особено",
        "от",
        "отгоре",
        "отново",
        "още",
        "пак",
        "по",
        "повече",
        "повечето",
        "под",
        "поне",
        "поради",
        "после",
        "почти",
        "прави",
        "пред",
        "преди",
        "през",
        "при",
        "пък",
        "първата",
        "първи",
        "първо",
        "пъти",
        "равен",
        "равна",
        "с",
        "са",
        "сам",
        "само",
        "се",
        "сега",
        "си",
        "син",
        "скоро",
        "след",
        "следващ",
        "сме",
        "смях",
        "според",
        "сред",
        "срещу",
        "сте",
        "съм",
        "със",
        "също",
        "т",
        "т.н.",
        "тази",
        "така",
        "такива",
        "такъв",
        "там",
        "твой",
        "те",
        "тези",
        "ти",
        "то",
        "това",
        "тогава",
        "този",
        "той",
        "толкова",
        "точно",
        "три",
        "трябва",
        "тук",
        "тъй",
        "тя",
        "тях",
        "у",
        "утре",
        "харесва",
        "хиляди",
        "ч",
        "часа",
        "че",
        "често",
        "чрез",
        "ще",
        "щом",
        "юмрук",
        "я",
        "як",
    ]
    .into_iter()
    .collect()
});
