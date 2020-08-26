use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Slovenčina (Slovak)
pub static STOPWORDS_SLK: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "a",
        "aby",
        "aj",
        "ak",
        "akej",
        "akejže",
        "ako",
        "akom",
        "akomže",
        "akou",
        "akouže",
        "akože",
        "aká",
        "akáže",
        "aké",
        "akého",
        "akéhože",
        "akému",
        "akémuže",
        "akéže",
        "akú",
        "akúže",
        "aký",
        "akých",
        "akýchže",
        "akým",
        "akými",
        "akýmiže",
        "akýmže",
        "akýže",
        "ale",
        "alebo",
        "ani",
        "asi",
        "avšak",
        "až",
        "ba",
        "bez",
        "bezo",
        "bol",
        "bola",
        "boli",
        "bolo",
        "bude",
        "budem",
        "budeme",
        "budete",
        "budeš",
        "budú",
        "buď",
        "by",
        "byť",
        "cez",
        "cezo",
        "dnes",
        "do",
        "ešte",
        "ho",
        "hoci",
        "i",
        "iba",
        "ich",
        "im",
        "inej",
        "inom",
        "iná",
        "iné",
        "iného",
        "inému",
        "iní",
        "inú",
        "iný",
        "iných",
        "iným",
        "inými",
        "ja",
        "je",
        "jeho",
        "jej",
        "jemu",
        "ju",
        "k",
        "kam",
        "kamže",
        "každou",
        "každá",
        "každé",
        "každého",
        "každému",
        "každí",
        "každú",
        "každý",
        "každých",
        "každým",
        "každými",
        "kde",
        "kej",
        "kejže",
        "keď",
        "keďže",
        "kie",
        "kieho",
        "kiehože",
        "kiemu",
        "kiemuže",
        "kieže",
        "koho",
        "kom",
        "komu",
        "kou",
        "kouže",
        "kto",
        "ktorej",
        "ktorou",
        "ktorá",
        "ktoré",
        "ktorí",
        "ktorú",
        "ktorý",
        "ktorých",
        "ktorým",
        "ktorými",
        "ku",
        "ká",
        "káže",
        "ké",
        "kéže",
        "kú",
        "kúže",
        "ký",
        "kýho",
        "kýhože",
        "kým",
        "kýmu",
        "kýmuže",
        "kýže",
        "lebo",
        "leda",
        "ledaže",
        "len",
        "ma",
        "majú",
        "mal",
        "mala",
        "mali",
        "mať",
        "medzi",
        "mi",
        "mne",
        "mnou",
        "moja",
        "moje",
        "mojej",
        "mojich",
        "mojim",
        "mojimi",
        "mojou",
        "moju",
        "možno",
        "mu",
        "musia",
        "musieť",
        "musí",
        "musím",
        "musíme",
        "musíte",
        "musíš",
        "my",
        "má",
        "mám",
        "máme",
        "máte",
        "máš",
        "môcť",
        "môj",
        "môjho",
        "môže",
        "môžem",
        "môžeme",
        "môžete",
        "môžeš",
        "môžu",
        "mňa",
        "na",
        "nad",
        "nado",
        "najmä",
        "nami",
        "naša",
        "naše",
        "našej",
        "naši",
        "našich",
        "našim",
        "našimi",
        "našou",
        "ne",
        "nech",
        "neho",
        "nej",
        "nejakej",
        "nejakom",
        "nejakou",
        "nejaká",
        "nejaké",
        "nejakého",
        "nejakému",
        "nejakú",
        "nejaký",
        "nejakých",
        "nejakým",
        "nejakými",
        "nemu",
        "než",
        "nich",
        "nie",
        "niektorej",
        "niektorom",
        "niektorou",
        "niektorá",
        "niektoré",
        "niektorého",
        "niektorému",
        "niektorú",
        "niektorý",
        "niektorých",
        "niektorým",
        "niektorými",
        "nielen",
        "niečo",
        "nim",
        "nimi",
        "nič",
        "ničoho",
        "ničom",
        "ničomu",
        "ničím",
        "no",
        "nám",
        "nás",
        "náš",
        "nášho",
        "ním",
        "o",
        "od",
        "odo",
        "on",
        "ona",
        "oni",
        "ono",
        "ony",
        "oň",
        "oňho",
        "po",
        "pod",
        "podo",
        "podľa",
        "pokiaľ",
        "popod",
        "popri",
        "potom",
        "poza",
        "pre",
        "pred",
        "predo",
        "preto",
        "pretože",
        "prečo",
        "pri",
        "práve",
        "s",
        "sa",
        "seba",
        "sebe",
        "sebou",
        "sem",
        "si",
        "sme",
        "so",
        "som",
        "ste",
        "svoj",
        "svoja",
        "svoje",
        "svojho",
        "svojich",
        "svojim",
        "svojimi",
        "svojou",
        "svoju",
        "svojím",
        "sú",
        "ta",
        "tak",
        "takej",
        "takejto",
        "taká",
        "takáto",
        "také",
        "takého",
        "takéhoto",
        "takému",
        "takémuto",
        "takéto",
        "takí",
        "takú",
        "takúto",
        "taký",
        "takýto",
        "takže",
        "tam",
        "teba",
        "tebe",
        "tebou",
        "teda",
        "tej",
        "tejto",
        "ten",
        "tento",
        "ti",
        "tie",
        "tieto",
        "tiež",
        "to",
        "toho",
        "tohoto",
        "tohto",
        "tom",
        "tomto",
        "tomu",
        "tomuto",
        "toto",
        "tou",
        "touto",
        "tu",
        "tvoj",
        "tvoja",
        "tvoje",
        "tvojej",
        "tvojho",
        "tvoji",
        "tvojich",
        "tvojim",
        "tvojimi",
        "tvojím",
        "ty",
        "tá",
        "táto",
        "tí",
        "títo",
        "tú",
        "túto",
        "tých",
        "tým",
        "tými",
        "týmto",
        "u",
        "už",
        "v",
        "vami",
        "vaša",
        "vaše",
        "vašej",
        "vaši",
        "vašich",
        "vašim",
        "vaším",
        "veď",
        "viac",
        "vo",
        "vy",
        "vám",
        "vás",
        "váš",
        "vášho",
        "však",
        "všetci",
        "všetka",
        "všetko",
        "všetky",
        "všetok",
        "z",
        "za",
        "začo",
        "začože",
        "zo",
        "áno",
        "čej",
        "či",
        "čia",
        "čie",
        "čieho",
        "čiemu",
        "čiu",
        "čo",
        "čoho",
        "čom",
        "čomu",
        "čou",
        "čože",
        "čí",
        "čím",
        "čími",
        "ďalšia",
        "ďalšie",
        "ďalšieho",
        "ďalšiemu",
        "ďalšiu",
        "ďalšom",
        "ďalšou",
        "ďalší",
        "ďalších",
        "ďalším",
        "ďalšími",
        "ňom",
        "ňou",
        "ňu",
        "že",
    ]
    .into_iter()
    .collect()
});
