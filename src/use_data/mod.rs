use std::str::FromStr;

use self::parsers::attachment_parser::parse_attachments;
use self::parsers::english_dictionary_parser::parse_english_dictionary;
use self::parsers::latin_dictionary_parser::parse_latin_dictionary;
use self::parsers::latin_inflection_parser::parse_latin_inflections;
use self::parsers::modifiers_parser::parse_modifiers;
use self::parsers::stem_parser::parse_latin_stems;
use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::{
    Attachment, EnglishWordInfo, Inflection, LatinWordInfo, Modifier, Stem,
};
use crate::utils::data::{
    get_latin_dictionary, get_latin_not_packons, get_latin_packons, get_latin_prefixes,
    get_latin_suffixes, get_latin_tackons, get_latin_tickons, get_unique_latin_words,
};
use crate::Error;
use serde::Serialize;

mod parsers {
    pub mod attachment_parser;
    pub mod english_dictionary_parser;
    pub mod latin_dictionary_parser;
    pub mod latin_inflection_parser;
    pub mod modifiers_parser;
    pub mod stem_parser;
}

mod utils;

#[derive(Debug)]
pub enum WordType {
    English,
    Latin,
    Inflections,
    NotPackons,
    Packons,
    Prefixes,
    Stems,
    Suffixes,
    Tackons,
    Tickons,
    UniqueLatin,
}

impl FromStr for WordType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "english" => Ok(WordType::English), // done
            "latin" => Ok(WordType::Latin),     // done
            "inflections" | "inflection" => Ok(WordType::Inflections),
            "not_packons" | "not_packon" => Ok(WordType::NotPackons),
            "packons" | "packon" => Ok(WordType::Packons),
            "prefixes" | "prefix" => Ok(WordType::Prefixes),
            "stems" | "stem" => Ok(WordType::Stems),
            "suffixes" | "suffix" => Ok(WordType::Suffixes),
            "tackons" | "tackon" => Ok(WordType::Tackons),
            "tickons" | "tickon" => Ok(WordType::Tickons),
            "unique_latin" => Ok(WordType::UniqueLatin), // done
            _ => Err(Error::InvalidWordType(s.to_string())),
        }
    }
}

impl WordType {
    pub fn is_valid_word_type(s: &str) -> bool {
        matches!(s, "english" | "latin" | "inflections" | "inflection" | "not_packons" | "not_packon"
            | "packon" | "packons" | "prefixes" | "prefix" | "stems" | "stem" | "suffixes"
            | "suffix" | "tackons" | "tackon" | "tickons" | "tickon" | "unique_latin")
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OutputList {
    Latin(Vec<LatinWordInfo>),
    English(Vec<EnglishWordInfo>),
    Inflections(Vec<Inflection>),
    Attachment(Vec<Attachment>),
    Modifiers(Vec<Modifier>),
    Stems(Vec<Stem>),
}

#[allow(clippy::too_many_arguments)]
pub fn get_list(
    word_type: WordType,
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> OutputList {
    match word_type {
        WordType::English => {
            let list = parse_english_dictionary(pos_list, max, min, exact, amount, random);
            OutputList::English(list)
        }
        WordType::Latin => {
            let dictionary = get_latin_dictionary();
            let list =
                parse_latin_dictionary(dictionary, pos_list, max, min, exact, amount, random);
            OutputList::Latin(list)
        }
        WordType::Inflections => {
            let list = parse_latin_inflections(pos_list, max, min, exact, amount, random);
            OutputList::Inflections(list)
        }
        WordType::NotPackons => {
            let attachments = get_latin_not_packons();
            let list = parse_attachments(attachments, None, max, min, exact, amount, random);
            OutputList::Attachment(list)
        }
        WordType::Packons => {
            let attachments = get_latin_packons();
            let list = parse_attachments(attachments, None, max, min, exact, amount, random);
            OutputList::Attachment(list)
        }
        WordType::Prefixes => {
            let modifiers = get_latin_prefixes();
            let list = parse_modifiers(modifiers, pos_list, max, min, exact, amount, random);
            OutputList::Modifiers(list)
        }
        WordType::Stems => {
            let list = parse_latin_stems(pos_list, max, min, exact, amount, random);
            OutputList::Stems(list)
        }
        WordType::Suffixes => {
            let modifiers = get_latin_suffixes();
            let list = parse_modifiers(modifiers, pos_list, max, min, exact, amount, random);
            OutputList::Modifiers(list)
        }
        WordType::Tackons => {
            let attachments = get_latin_tackons();
            let list = parse_attachments(attachments, None, max, min, exact, amount, random);
            OutputList::Attachment(list)
        }
        WordType::Tickons => {
            let attachments = get_latin_tickons();
            let list = parse_attachments(attachments, None, max, min, exact, amount, random);
            OutputList::Attachment(list)
        }
        WordType::UniqueLatin => {
            let dictionary = get_unique_latin_words();
            let list =
                parse_latin_dictionary(dictionary, pos_list, max, min, exact, amount, random);
            OutputList::Latin(list)
        }
    }
}
