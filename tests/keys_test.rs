use p1_maxagram::{corpus, keys};
use p1_maxagram::keys::Key;
use std::ops::{Add, Sub};
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

fn keys_are_unique<T: keys::Key>() -> bool {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<T>::new(&dict);
    idx.keys_are_unique()
}

fn test_key_lookup<T: keys::Key>() {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<T>::new(&dict);
    assert!(idx.lookup("food").is_empty());
    assert_eq!(sorted_class(&dict, &idx.lookup("rust")), ["rust"]);
    let post = sorted_class(&dict, &idx.lookup("post"));
    let postref = ["opts", "pots", "spot", "stop", "tops"];
    assert_eq!(post, postref);
}

#[test]
fn check_num_classes() {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<keys::HashKey64>::new(&dict);
    assert_eq!(idx.num_classes(), 9);
}

#[test]
fn check_len_class_max() {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<keys::HashKey64>::new(&dict);
    assert_eq!(idx.len_class_max(), 5);
}

#[test]
fn test_maxagrams64() {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<keys::HashKey64>::new(&dict);
    let classes = idx.maxagrams();
    assert_eq!(classes.len(), 2);
    assert_eq!(classes[0].len(), 5);
    assert_eq!(classes[1].len(), 5);
    let c0 = sorted_class(&dict, &classes[0]);
    let c1 = sorted_class(&dict, &classes[1]);
    let listen = ["enlist", "inlets", "listen", "silent", "tinsel"];
    let pots = ["opts", "pots", "spot", "stop", "tops"];
    assert!(c0 == listen && c1 == pots || c1 == listen && c0 == pots);
}

#[test]
fn test_unique64() {
    assert!(keys_are_unique::<keys::HashKey64>());
}

#[test]
fn test_key_lookup64() {
    test_key_lookup::<keys::HashKey64>();
}

/// TODO: Add a silly key type with lots of collisions -- something like
///  1 if there are an odd number of 'e's in a word and 0 otherwise.
///  Make sure the following tests still run, then

pub struct SillyHash(u8);

/// Per spec, this is *by hash key class* rather than by anagram class
#[test]
fn test_maxagrams_silly() {
    let dict = load_sample();
    let idx = keys::DictKeyIndex::<keys::HashKey64>::new(&dict);
    let classes = idx.maxagrams();

    // We assume the hash is still fine enough to produce two classes
    // (no fair just returning "0" for all cases)
    assert_eq!(classes.len(), 2);

    let c0 = &classes[0];
    let c1 = &classes[1];

    // The two classes should have different keys
    let k0 = SillyHash::new(&dict[c0[0]]);
    let k1 = SillyHash::new(&dict[c1[0]]);
    assert_ne!(k0, k1);

    // And the same key within each hash
    for word_id in c0 {
        assert_eq!(SillyHash::new(&dict[*word_id]), k0);
    }
    for word_id in c1 {
        assert_eq!(SillyHash::new(&dict[*word_id]), k1);
    }
}

#[test]
fn test_unique_silly() {
    assert!(!keys_are_unique::<SillyHash>());
}

#[test]
fn test_key_lookup_silly() {
    test_key_lookup::<SillyHash>();
}

