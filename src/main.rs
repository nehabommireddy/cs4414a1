use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

mod anagrams;
mod corpus;

fn print_dict_stats(dict: &[String]) {
    let nwords = dict.iter().map(|w| w.len()).max().unwrap_or_default();
    println!("# Dictionary of {} words", dict.len());
    println!("# Longest word is {} letters", nwords);
}

fn print_words(dict: &[String], ids: &[usize]) {
    println!("---");
    for id in ids {
        println!("{}", dict[*id]);
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // Check for dictionary argument
    if args.len() <= 1 {
        eprintln!("No dictionary specified");
        return ExitCode::FAILURE;
    }

    // Load dictionary
    let start = Instant::now();
    let path = Path::new(&args[1]);
    let dict = match corpus::load_words(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    println!("# Load time is {:?}", start.elapsed());

    // Create anagram index
    let start = Instant::now();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    println!("# Index time is {:?}", start.elapsed());

    // Run commands one at a time
    let mut iarg = 2;
    while iarg < args.len() {
        println!("# --- Run {}", args[iarg]);
        let start = Instant::now();
        match args[iarg].as_str() {
            // Print stats about the dictionary
            "stats" => {
                print_dict_stats(&dict);
            }

            // Print histogram for some word
            "histogram" => {
                if iarg + 1 < args.len() {
                    iarg += 1;
                    println!(
                        "# Histogram for {} is {:?}",
                        args[iarg],
                        anagrams::histogram(&args[iarg])
                    );
                } else {
                    eprintln!("Syntax: histogram word");
                    return ExitCode::FAILURE;
                }
            }

            // Print max use of each letter over all words in dict
            "max_histogram" => {
                let mut hist = [0u8; 26];
                for key in anagrams.class_keys.iter() {
                    for i in 0..26 {
                        if key[i] > hist[i] {
                            hist[i] = key[i];
                        }
                    }
                }
                println!("# {:?}", hist);
            }

            // Print anagram class for a particular word
            "anagram" => {
                if iarg + 1 < args.len() {
                    iarg += 1;
                    println!("# Look for anagrams of {}", args[iarg]);
                    let class = anagrams.lookup(&args[iarg]);
                    if let Some(c) = class {
                        print_words(&dict, c);
                    } else {
                        println!("# No match found");
                    }
                } else {
                    eprintln!("Syntax: anagram word");
                    return ExitCode::FAILURE;
                }
            }

            // Print all maxagram classes
            "maxagrams" => {
                for c in anagrams.maxagrams() {
                    print_words(&dict, anagrams.get_class(c));
                }
            }

            // Print histogram of class lengths
            "anagram_lens" => {
                println!("# Number of classes {}", anagrams.num_classes());
                println!("# Class lengths {:?}", anagrams.len_class_hist());
            }

            _ => {
                eprintln!("Unknown command {}", args[iarg]);
                return ExitCode::FAILURE;
            }
        };
        println!("# --- operation time is {:?}", start.elapsed());
        iarg += 1;
    }

    ExitCode::SUCCESS
}
