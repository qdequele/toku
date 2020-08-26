use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Soomaaliga (Somali)
pub static STOPWORDS_SOM: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "aad",
        "albaabkii",
        "atabo",
        "ay",
        "ayaa",
        "ayee",
        "ayuu",
        "dhan",
        "hadana",
        "in",
        "inuu",
        "isku",
        "jiray",
        "jirtay",
        "ka",
        "kale",
        "kasoo",
        "ku",
        "kuu",
        "lakin",
        "markii",
        "oo",
        "si",
        "soo",
        "uga",
        "ugu",
        "uu",
        "waa",
        "waxa",
        "waxuu",
    ]
    .into_iter()
    .collect()
});
