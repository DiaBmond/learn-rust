mod text;

pub fn analyze_text(text: &str) {
    // Absolute path
    crate::text::analyze::find_first_word(text);

    // Relative path
    text::analyze::find_last_word(text);
}
use crate::text::analyze;
use crate::text::analyze::find_last_word;

pub fn analyze_text_2(text: &str) {
    analyze::find_first_word(text);
    find_last_word(text);
}
