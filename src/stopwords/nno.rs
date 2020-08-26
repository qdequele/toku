use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Nynorsk (Nynorsk) - Not Yet Implemented
pub static STOPWORDS_NNO: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
