use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Ilokano (Ilocano) - Not Yet Implemented
pub static STOPWORDS_ILO: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
