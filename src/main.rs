pub mod cli;
use std::io::Write;
use std::str::FromStr;

use cli::{Arg, Cli, Command, ArgValue};
use vocab_vault::dictionary_structures::dictionary_keys::PartOfSpeech;
use vocab_vault::translators::Language;
use vocab_vault::use_data::{get_list, WordType};
use vocab_vault::{english_to_latin, latin_to_english};

use vocab_vault::{translators::DisplayType};
//TODO: add a command for searching a word by id in english or latin dictionary
//TODO: display the amount of time it took for a command to execute
fn main() {
    let global_args_for_translation = vec![
        Arg::new()
            .with_name("words")
            .with_value_name("WORDS")
            .with_help("The words to translate"),
        Arg::new()
            .with_name("max")
            .with_short('m')
            .with_long("max")
            .with_value_name("MAX")
            .default("6")
            .with_help("The maximum number of translations per definition"),
        Arg::new()
            .with_name("sort")
            .with_short('s')
            .with_long("sort")
            .with_help("Sort the output by word frequency"),
        Arg::new()
            .with_name("pretty")
            .with_short('p')
            .with_long("pretty")
            .with_help("Prints the output in a pretty format"),
        Arg::new()
            .with_name("detailed")
            .with_short('d')
            .with_long("detailed")
            .with_help("Adds more information to the pretty output")
            .requires("pretty"),
    ];

    let cli = Cli::new().with_default_command("tui").with_commands(vec![
        Command::new("transEng", "Translate english to latin")
            .with_args(&global_args_for_translation),
        Command::new("transLat", "Translate latin to english")
            .with_args(&global_args_for_translation)
            .with_arg(
                Arg::new()
                    .with_name("tricks")
                    .with_short('t')
                    .with_long("tricks")
                    .with_help("Will attempt to use various tricks to find the translation"),
            ),
        Command::new("getList", "Gets a list of words based on the options provided")
            .with_arg(
                Arg::new()
                .with_name("type")
                .with_value_name("TYPE")
                .with_help("The type of words to get. Options: english, latin, inflections, not_packons, packons, prefixes, stems, suffixes, tackons, tickons, unique_latin"),
            )
            .with_arg(
                Arg::new()
                .with_name("pos")
                .with_short('p')
                .with_long("pos")
                .with_value_name("POS")
                .with_help("The part of speeches to include, separated by commas"),
            )
            .with_arg(
                Arg::new()
                .with_name("max")
                .with_short('m')
                .with_long("max")
                .with_value_name("MAX")
                .with_help("The maximum word length"),
            )
            .with_arg(
                Arg::new()
                .with_name("min")
                .with_short('n')
                .with_long("min")
                .with_value_name("MIN")
                .with_help("The minimum word length"),
            )
            .with_arg(
                Arg::new()
                .with_name("exact")
                .with_short('e')
                .with_long("exact")
                .with_value_name("EXACT")
                .with_help("The exact word length"),
            )
            .with_arg(
                Arg::new()
                .with_name("amount")
                .with_short('a')
                .with_long("amount")
                .with_value_name("AMOUNT")
                .with_help("The amount of words to get"),
            )
            .with_arg(
                Arg::new()
                .with_name("random")
                .with_short('r')
                .with_long("random")
                .with_help("Get words from a random position")
                .requires("amount"),
            )
            .with_arg(
                Arg::new()
                .with_name("display")
                .with_short('d')
                .with_long("display")
                .with_help("Will display as json"),
            )
            .with_arg(
                Arg::new()
                .with_name("to")
                .with_short('t')
                .with_long("to")
                .with_value_name("TO")
                .with_help("The file to export the results to"),
            ),
        Command::new("help", "Helps you")
            .with_arg(
                Arg::new()
                .with_name("command")
                .with_value_name("COMMAND")
                .with_help("A command to help with"),
            ),
        Command::new("tui", "Starts the tui (.help for info)"),
    ]);

    let command = cli.match_commands();

    match command.name {
        "transEng" => {
            let words = command.get_value().throw_if_none();
            let max = command
                .get_value_of("max")
                .throw_if_none()
                .parse::<usize>()
                .unwrap();
            let sort = command.has("sort");
            let pretty = command.has("pretty");
            let detailed = command.has("detailed");

            let translations = english_to_latin(&words, max, sort);
            if pretty {
                for translation in translations {
                    translation.display(DisplayType::Pretty(detailed));
                }
            } else {
                println!("{}", serde_json::to_string_pretty(&translations).unwrap());
            }
        }
        "transLat" => {
            let words = command.get_value().throw_if_none();
            let max = command
                .get_value_of("max")
                .throw_if_none()
                .parse::<usize>()
                .unwrap();
            let sort = command.has("sort");
            let pretty = command.has("pretty");
            let detailed = command.has("detailed");
            let tricks = command.has("tricks");

            let translations = latin_to_english(&words, max, tricks, sort);
            if pretty {
                for translation in translations {
                    translation.display(DisplayType::Pretty(detailed));
                }
            } else {
                println!("{}", serde_json::to_string_pretty(&translations).unwrap());
            }
        }
        "getList" => {
            let type_of_words = command.get_value().throw_if_none();
            let pos = command.get_value_of("pos");
            let max = command.get_value_of("max");
            let min = command.get_value_of("min");
            let exact = command.get_value_of("exact");
            let amount = command.get_value_of("amount");
            let random = command.has("random");
            let display = command.has("display");
            let to = command.get_value_of("to");

            if !WordType::is_valid_word_type(&type_of_words) {
                println!(
                    "Invalid type of words. Please use `help` to see the available types of words."
                );
                return;
            }

            let word_type = WordType::from_str(type_of_words.as_str()).unwrap_or_else(|e| {
                println!("{e}");
                std::process::exit(0);
            });

            let pos_list = match pos {
                ArgValue::Present(pos) => {
                    let pos_list: Vec<PartOfSpeech> = pos
                        .split(",")
                        .map(|pos| PartOfSpeech::dict_key_to_part_of_speech(pos.trim()))
                        .collect();
                    Some(pos_list)
                }
                ArgValue::Missing(_) => None,
            };

            if pos_list.is_some() && pos_list.as_ref().unwrap().contains(&PartOfSpeech::Unknown) {
                println!("Invalid part of speech entered.");
                println!("Please use the following: noun, verb, participle, adjective, preposition, pronoun, interjection, numeral, conjunction, adverb, number, supine, packon, tackon, prefix, suffix");
                std::process::exit(0);
            }

            let max = match max {
                ArgValue::Present(max) => Some(max.parse::<usize>().unwrap() as i32),
                ArgValue::Missing(_) => None,
            };

            let min = match min {
                ArgValue::Present(min) => Some(min.parse::<usize>().unwrap() as i32),
                ArgValue::Missing(_) => None,
            };

            let exact = match exact {
                ArgValue::Present(exact) => Some(exact.parse::<usize>().unwrap() as i32),
                ArgValue::Missing(_) => None,
            };

            let amount = match amount {
                ArgValue::Present(amount) => Some(amount.parse::<usize>().unwrap() as i32),
                ArgValue::Missing(_) => None,
            };

            let list = get_list(
                word_type, pos_list, max, min, exact, amount, random
            );
            
            if display {
                println!("{}", serde_json::to_string_pretty(&list).unwrap());
            }
        
            if let ArgValue::Present(mut file_path) = to {
                if !file_path.ends_with(".json") {
                    file_path.push_str(".json");
                }
        
                if std::path::Path::new(&file_path).exists() {
                    println!("File already exists, do you want to overwrite it? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "y" {
                        return;
                    }
                }
        
                let path = std::path::Path::new(&file_path);
        
                if !path.exists() {
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                }
        
                let file = std::fs::File::create(&file_path).unwrap();
                serde_json::to_writer_pretty(file, &list).unwrap();
                println!("File created successfully at {file_path}");
            }
        }
        "help" => {
            let command = command.get_value().to_option();
            cli.help(command);
        }
        "tui" => {
            let mut input = String::new();
            let mut language = Language::Latin;
            loop {
                print!("> ");
                input.clear();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input {
                    ".exit" | ".quit" | "q" => {
                        break;
                    }
                    ".help" => {
                        println!("Commands:");
                        println!(".help - Displays this message");
                        println!(".exit - Exits the program");
                        println!(".switch - Switches between latin and english");
                        println!("enter a word to translate it")
                    }
                    ".switch" => {
                        language = match language {
                            Language::Latin => Language::English,
                            Language::English => Language::Latin,
                        };
                        println!("Switched to {:?}", language.as_str());
                    }
                    ".clear" => {
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    _ => match language {
                        Language::Latin => {
                            let translations = latin_to_english(input, 6, true, true);
                            if false {
                                for translation in translations {
                                    translation.display(DisplayType::Pretty(false));
                                }
                            } else {
                                println!("{}", serde_json::to_string_pretty(&translations).unwrap());
                            }
                        }
                        Language::English => {
                            let translations = english_to_latin(input, 6, true);
                            if false {
                                for translation in translations {
                                    translation.display(DisplayType::Pretty(false));
                                }
                            } else {
                                println!("{}", serde_json::to_string_pretty(&translations).unwrap());
                            }
                        }
                    },
                }
            }
        }
        _ => {
            println!("Invalid command. Please use `help` to see the available commands.");
        }
    }
}

//TODO: get dictionaries here, to not repeat getting them for each word
