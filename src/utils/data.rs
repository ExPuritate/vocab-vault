use crate::dictionary_structures::dictionary_values::{
    Attachment, EnglishWordInfo, Inflection, LatinWordInfo, Modifier, Stem, UniqueLatinWordInfo,
};
use std::{include_bytes, sync::OnceLock};

pub fn get_english_dictionary() -> &'static [EnglishWordInfo] {
    static DICTIONARY: OnceLock<Vec<EnglishWordInfo>> = OnceLock::new();
    DICTIONARY.get_or_init(|| {
        let english_words_json = include_bytes!("../dictionary/english_words.json");
        serde_json::from_slice(english_words_json).unwrap()
    })
}

pub fn get_latin_dictionary() -> &'static [LatinWordInfo] {
    static DICTIONARY: OnceLock<Vec<LatinWordInfo>> = OnceLock::new();
    DICTIONARY.get_or_init(|| {
        let latin_words_json = include_bytes!("../dictionary/latin_dictionary.json");
        serde_json::from_slice(latin_words_json).unwrap()
    })
}

pub fn get_unique_latin_words() -> &'static [LatinWordInfo] {
    static UNIQUE_LATIN_WORDS: OnceLock<Vec<LatinWordInfo>> = OnceLock::new();
    UNIQUE_LATIN_WORDS.get_or_init(|| {
        let unique_latin_words_json = include_bytes!("../dictionary/unique_latin_words.json");
        let unique_latin_words: Vec<UniqueLatinWordInfo> =
            serde_json::from_slice(unique_latin_words_json).unwrap();

        unique_latin_words
            .iter()
            .map(|word| LatinWordInfo {
                orth: word.orth.to_string(),
                senses: word.senses.to_vec(),
                pos: word.pos,
                form: word.form.clone(),
                info: word.info,
                n: word.n.clone(),
                ..LatinWordInfo::new()
            })
            .collect()
    })
}

pub fn get_latin_inflections() -> &'static [Inflection] {
    static INFLECTIONS: OnceLock<Vec<Inflection>> = OnceLock::new();
    INFLECTIONS.get_or_init(|| {
        let latin_inflections_json = include_bytes!("../dictionary/latin_inflections.json");
        serde_json::from_slice(latin_inflections_json).unwrap()
    })
}

pub fn get_latin_stems() -> &'static [Stem] {
    static STEMS: OnceLock<Vec<Stem>> = OnceLock::new();
    STEMS.get_or_init(|| {
        let latin_stems_json = include_bytes!("../dictionary/latin_stems.json");
        serde_json::from_slice(latin_stems_json).unwrap()
    })
}

pub fn get_latin_prefixes() -> &'static [Modifier] {
    static PREFIXES: OnceLock<Vec<Modifier>> = OnceLock::new();
    PREFIXES.get_or_init(|| {
        let latin_prefixes_json = include_bytes!("../dictionary/latin_prefixes.json");
        serde_json::from_slice(latin_prefixes_json).unwrap()
    })
}

pub fn get_latin_suffixes() -> &'static [Modifier] {
    static SUFFIXES: OnceLock<Vec<Modifier>> = OnceLock::new();
    SUFFIXES.get_or_init(|| {
        let latin_suffixes_json = include_bytes!("../dictionary/latin_suffixes.json");
        serde_json::from_slice(latin_suffixes_json).unwrap()
    })
}

pub fn get_latin_packons() -> &'static [Attachment] {
    static PACKONS: OnceLock<Vec<Attachment>> = OnceLock::new();
    PACKONS.get_or_init(|| {
        let latin_packons_json = include_bytes!("../dictionary/latin_packons.json");
        serde_json::from_slice(latin_packons_json).unwrap()
    })
}

pub fn get_latin_not_packons() -> &'static [Attachment] {
    static NOT_PACKONS: OnceLock<Vec<Attachment>> = OnceLock::new();
    NOT_PACKONS.get_or_init(|| {
        let latin_not_packons_json = include_bytes!("../dictionary/latin_not_packons.json");
        serde_json::from_slice(latin_not_packons_json).unwrap()
    })
}

pub fn get_latin_tackons() -> &'static [Attachment] {
    static TACKONS: OnceLock<Vec<Attachment>> = OnceLock::new();
    TACKONS.get_or_init(|| {
        let latin_tackons_json = include_bytes!("../dictionary/latin_tackons.json");
        serde_json::from_slice(latin_tackons_json).unwrap()
    })
}

pub fn get_latin_tickons() -> &'static [Attachment] {
    static TICKONS: OnceLock<Vec<Attachment>> = OnceLock::new();
    TICKONS.get_or_init(|| {
        let latin_tickons_json = include_bytes!("../dictionary/latin_tickons.json");
        serde_json::from_slice(latin_tickons_json).unwrap()
    })
}
