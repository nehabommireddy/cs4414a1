# CS4414 HW1 - Maximal anagrams

## Logistics

- This homework may be done individually or with a partner (recommended)
- Task 1 will be due Tue, Sep 9 at 11:59 PM via Gradescope
- Tasks 2-4 are still being re-done, will be posted in the next couple days

## Changelog
Sept 4th:
- Clarify docs for `AnagramClasses::lookup`
- Fix docs for `AnagramClasses:::lookup` where `Ok` was used instead of `Some`

Sept 2nd:
- Specify behavior of `AnagramClasses::len_class_max()` when no classes exist
- Autoformat with `cargo fmt`
- Add gitignore
- Change line comments `//` to doc comments doc comments `///`

## Introduction

Anagrams are pairs of words that use the same letters, but in a
different order.  For example, "listen" and "silent" are anagrams of
each other.  Abstractly, we write `is_anagram(w1,w2)` to denote that
`w1` and `w2` are anagrams of each other.

The relation `is_anagram` is an [equivalence relation][wiki-equivalence],
and therefore partitions any dictionary of words into equivalence
classes, which we'll call anagram classes.  For a given dictionary,
we will call the largest anagram classes *maxagrams*.

We have two main goals for this assignment:

1. Get some practice writing Rust code!
2. Think through performance implications of how we represent data.

To this end, we'll tackle a few tasks involving anagram classes and
maxagrams, both for individual words and for pairs of words.  As an
anti-procrastination measure, the first task will be due on Monday,
Sep 7 at 11:59 PM.  The remaining tasks will be due one week after
that on Sep 14.

[wiki-equivalence]: https://en.wikipedia.org/wiki/Equivalence_relation

## Setup

We provide a short sample dictionary in `tests/sample.txt`.  It
contains two maxagrams of size 5: the "listen" cluster ("listen",
"silent", "tinsel", "enlist", "inlets") and the "stop" cluster
("stop", "spot", "pots", "tops", "opts"). It also holds "sin" and
"let", which together spell "listen", plus a few decoy words like
"hello" and "rust` that belong to no maxagrams.

There are two more serious dictionaries in `examples/` (taken from
<https://github.com/dolph/dictionary/>).
The `examples/popular.txt` dictionary consists of a bit over 25K
popular English words; the `examples/enable1.txt` dictionary
has a bit over 172K English words.

The `src/main.rs` file contains the main command-line driver code.
You can execute the driver by directly invoking the executable or
by using `cargo run` (we recommend the latter).  For example,
to get some basic statistics, you can run the following command
from the crate root directory:
```bash
cargo run --release examples/popular.txt stats
```
You will definitely want to use the release mode for the later tasks.
The debug mode is great for testing things out, but the release mode
code is much faster.

We provide starter code that loads the dictionaries and normalizes the
word representations to lower-case ASCII.  You should probably leave
this code alone.  We also implements the command-line driver; you're
welcome to add to this if you want, but you shouldn't need to.  The
code you need to implement is in various library files, and is
marked with [`todo!`][todo].

You will probably want to run the [Clippy][clippy] linter on your
code.  You don't need to follow all of Clippy's recommendations,
but it is worth seeing what they are.

[todo]: https://doc.rust-lang.org/std/macro.todo.html
[clippy]: https://doc.rust-lang.org/clippy/index.html

## Output specifications

Your code can output a variety of auxiliary information as you please.
These lines might contain timing information, relevant statistics,
etc.  Please prefix any such auxiliary lines with a `#` character.
Also, when writing out anagram classes, your code should prefix each
anagram class with a line consisting of three dashes (`---`).  For
example, if we were looking up the "listen" cluster in the test
dictionary, the output might be

```
# Looking up "listen"
# Dictionary load took xx ms
# Sorting into classes took xx ms
---
enlist
inlets
listen
silent
tinsel
```

Following these output conventions will simplify the process of
auto-grading.

## Steps

### Task 1: Histograms and maxagrams

The letter histogram for a word is an array of counts for how many
times each letter `a` through `z` appears.  We will normalize
everything to lower case and assume an English alphabet, so there are
26 bins in our histogram.  We will default to using 8-bit counters
for each bin; we can therefore represent a histogram with a fixed-length
Rust array type:

```rs
type LetterHist = [u8; 26];
```

You will need to implement the `histogram` method in `src/anagram.rs`
to compute these histograms.

Arrays with elements of a type that implements the `Ord` trait will
themselves automatically implement `Ord`, based on lexicographic
ordering by the elements.  This suggests the following algorithm for
partitioning the dictionary into anagram classes:

- Compute a vector of `(LetterHist, usize)` pairs for the letter
  histogram and the index of the associated word in the dictionary.
- Sort the vector (using the `sort` method)

At this point, the words in each anagram class will appear
consecutively within the vector.  Within each class, words will
be sorted by their dictionary ordering.  We then compress this
representation into the structure

```rs
struct AnagramClasses {
    class_keys: Vec<LetterHist>,
    class_offsets: Vec<usize>,
    word_ids: Vec<usize>
}
```

The words represented in
`word_ids[class_offsets[i]..class_offsets[i+1]]` should be anagrams of
each other, all associated with the letter histogram `class_keys[i]`.

After constructing this data structure, you should implement methods
for the following tasks:

- Get statistics about the anagram classes (number of classes, number
  of classes of different sizes, largest class size)
- Look up an anagram class by a query string
- Return the word lists for all maxagrams

### Task 2: Integer keys

- Variable length encoding
- Short keys and coarse partitioning
- Speed implications and timing

### Task 3: Two-word anagram lookup

- Solving two-sum efficiently

### Task 4: Two-word maxagram

- Cost implications of materializing
- Two-word maxagram size estimates
- Finding two-word maxagrams
