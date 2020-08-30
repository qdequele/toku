use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Українська (Ukrainian)
pub static STOPWORDS_UKR: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "але",
        "ви",
        "вона",
        "вони",
        "воно",
        "він",
        "в╡д",
        "з",
        "й",
        "коли",
        "ми",
        "нам",
        "про",
        "та",
        "ти",
        "хоча",
        "це",
        "цей",
        "чи",
        "чого",
        "що",
        "як",
        "яко╞",
        "із",
        "інших",
        "╙",
        "╞х",
        "╡",
    ]
    .iter()
    .cloned()
    .collect()
});
