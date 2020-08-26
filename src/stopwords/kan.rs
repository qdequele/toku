use once_cell::sync::Lazy;
use std::collections::HashSet;

/// ಕನ್ನಡ (Kannada)
pub static STOPWORDS_KAN: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "ಅಂತ",
        "ಅಥವಾ",
        "ಅದಕ್ಕೆ",
        "ಅದನ್ನು",
        "ಅದು",
        "ಅದೇ",
        "ಅವಕಾಶ",
        "ಅವರ",
        "ಅವರನ್ನು",
        "ಅವರಿಗೆ",
        "ಅವರು",
        "ಅಷ್ಟೇ",
        "ಆ",
        "ಆಗ",
        "ಆಗಿ",
        "ಆದರೆ",
        "ಇತ್ತು",
        "ಇದು",
        "ಇದೆ",
        "ಇದೇ",
        "ಇನ್ನು",
        "ಇನ್ನೂ",
        "ಇಲ್ಲಿ",
        "ಈ",
        "ಈಗ",
        "ಎಂದರೆ",
        "ಎಂದು",
        "ಎಂಬ",
        "ಎಂಬುದು",
        "ಎರಡು",
        "ಎಲ್ಲಾ",
        "ಎಷ್ಟು",
        "ಏನು",
        "ಒಳ್ಳೆಯ",
        "ಕನ್ನಡ",
        "ಕಾರಣ",
        "ಕುರಿತು",
        "ಕೂಡ",
        "ಕೆಲವು",
        "ಕೆಲಸ",
        "ಕೇವಲ",
        "ಜನ",
        "ಜೊತೆಗೆ",
        "ತಮ್ಮ",
        "ತುಂಬಾ",
        "ದಿನ",
        "ದೊಡ್ಡ",
        "ನಂತರ",
        "ನಮ್ಮ",
        "ನಾವು",
        "ನಿಮ್ಮ",
        "ನೀವು",
        "ನೋಡಿ",
        "ಬಂದ",
        "ಬಂದು",
        "ಬಗ್ಗೆ",
        "ಬಳಿಕ",
        "ಬೇರೆ",
        "ಮತ್ತು",
        "ಮತ್ತೆ",
        "ಮಾಡಬೇಕು",
        "ಮಾಡಿ",
        "ಮಾಡಿದ",
        "ಮಾಡುವ",
        "ಮಾತ್ರ",
        "ಮುಂದಿನ",
        "ಮುಂದೆ",
        "ಮೂಲಕ",
        "ಮೇಲೆ",
        "ಮೊದಲು",
        "ಯಾವ",
        "ಯಾವುದೇ",
        "ವರ್ಷ",
        "ವೇಳೆ",
        "ಸಾಕಷ್ಟು",
        "ಹಾಗಾಗಿ",
        "ಹಾಗೂ",
        "ಹೀಗೆ",
        "ಹೆಚ್ಚು",
        "ಹೇಗೆ",
        "ಹೇಳಿ",
        "ಹೊಸ",
    ]
    .into_iter()
    .collect()
});