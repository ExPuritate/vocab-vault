use crate::{translators::{english_to_latin::translate_english_to_latin, latin_to_english::translate_latin_to_english, DisplayType, Language, Translation, TranslationType}, utils::{data::{get_english_dictionary, get_latin_dictionary}, sanitize_word}};

pub mod dictionary_structures;
pub mod translators;
pub mod use_data;
pub mod utils;

pub fn latin_to_english(
    latin_text: &str,
    max: usize,
    tricks: bool,
    sort: bool,
) -> Vec<Translation> {
    let latin_words: Vec<&str> = latin_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in latin_words {
        let mut definitions = translate_latin_to_english(&sanitize_word(word), tricks);
        definitions.truncate(max);
        let mut translation =
            Translation::new(word.to_string(), TranslationType::Latin(definitions));

        translation.post_process(Language::Latin, sort);
        translations.push(translation);
    }

    translations
}

pub fn english_to_latin(
    english_text: &str,
    max: usize,
    sort: bool,
) -> Vec<Translation> {
    let english_words: Vec<&str> = english_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    let latin_dictionary = get_latin_dictionary();
    let english_dictionary = get_english_dictionary();

    for word in english_words {
        let definitions = translate_english_to_latin(
            &english_dictionary,
            &latin_dictionary,
            &sanitize_word(word),
            max,
            sort,
        );
        let mut translation =
            Translation::new(word.to_string(), TranslationType::English(definitions));
        translation.post_process(Language::English, sort);
        translations.push(translation);
    }

    translations
}
