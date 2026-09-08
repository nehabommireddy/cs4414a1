use p1_maxagram::{anagrams, corpus};
use std::path::Path;

fn load_sample() -> Vec<String> {
    let path = Path::new("tests/sample.txt");
    corpus::load_words(path).expect("Could not open sample.txt")
}

fn sorted_class<'d>(dict: &'d [String], class: &[usize]) -> Vec<&'d str> {
    let mut class: Vec<&str> = class.into_iter().map(|&i| dict[i].as_str()).collect();
    class.sort();
    class
}

#[test]
fn check_histogram() {
    let refhist: [u8; 26] = [
        0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let hist = anagrams::histogram("food");
    assert_eq!(refhist, hist);
}

#[test]
fn check_unicode() {
    let refhist: [u8; 26] = [
        0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let hist = anagrams::histogram("food❤️");
    assert_eq!(refhist, hist);
}

#[test]
fn check_num_classes() {
    let dict = load_sample();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    assert_eq!(anagrams.num_classes(), 9);
}

#[test]
fn check_len_class_max() {
    let dict = load_sample();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    assert_eq!(anagrams.len_class_max(), 5);
}

#[test]
fn check_maxagrams() {
    let dict = load_sample();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    let classes = anagrams.maxagrams();
    assert_eq!(classes.len(), 2);
    assert_eq!(anagrams.len_class(classes[0]), 5);
    assert_eq!(anagrams.len_class(classes[1]), 5);
    let c0 = sorted_class(&dict, anagrams.get_class(classes[0]));
    let c1 = sorted_class(&dict, anagrams.get_class(classes[1]));
    let listen = ["enlist", "inlets", "listen", "silent", "tinsel"];
    let pots = ["opts", "pots", "spot", "stop", "tops"];
    assert!(c0 == listen && c1 == pots || c1 == listen && c0 == pots);
}

#[test]
fn check_empty_anagram() {
    let dict = load_sample();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    assert!(anagrams.lookup("food").is_none());
}

#[test]
fn check_anagrams() {
    let dict = load_sample();
    let anagrams = anagrams::AnagramClasses::new(&dict);
    let rustx = anagrams.lookup("rust");
    let postx = anagrams.lookup("post");
    let Some(rust) = rustx else {
        panic!("Expected some rust");
    };
    let Some(post) = postx else {
        panic!("expected some post");
    };
    assert_eq!(rust.len(), 1);
    assert_eq!(dict[rust[0]], "rust");
    let posts = sorted_class(&dict, &post);
    assert_eq!(posts, ["opts", "pots", "spot", "stop", "tops"]);
}
