use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Afrikaans (Afrikaans)
pub static STOPWORDS_AFR: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "'n", "aan", "af", "al", "as", "baie", "by", "daar", "dag", "dat", "die", "dit", "een",
        "ek", "en", "gaan", "gesê", "haar", "het", "hom", "hulle", "hy", "in", "is", "jou", "jy",
        "kan", "kom", "ma", "maar", "met", "my", "na", "nie", "om", "ons", "op", "saam", "sal",
        "se", "sien", "so", "sy", "te", "toe", "uit", "van", "vir", "was", "wat", "ŉ",
    ]
    .iter()
    .cloned()
    .collect()
});
