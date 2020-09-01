use once_cell::sync::Lazy;
use std::collections::HashSet;

/// 日本語 (Japanese)
pub static STOPWORDS_JPN: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "あそこ",
        "あの",
        "あのかた",
        "あの人",
        "あります",
        "あれ",
        "います",
        "え",
        "おります",
        "が",
        "から",
        "ここ",
        "こちら",
        "この",
        "これ",
        "し",
        "しかし",
        "そこ",
        "その",
        "それ",
        "それで",
        "だれ",
        "で",
        "です",
        "と",
        "どこ",
        "どの",
        "なに",
        "なん",
        "に",
        "の",
        "は",
        "まで",
        "も",
        "より",
        "を",
        "何",
        "彼",
        "彼女",
        "我々",
        "私",
        "私達",
        "貴方",
        "貴方方",
    ]
    .iter()
    .cloned()
    .collect()
});
