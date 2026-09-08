# CS4414 HW1 - Maximal anagrams

## Logistics

- This homework may be done individually or with a partner (recommended)
- Task 1 will be due Tue, Sep 9 at 11:59 PM via Gradescope
- Tasks 2-4 are still being re-done, will be posted in the next couple days

## Changelog

Sept 7th:
- Change crate name to `p1_maxagram` for `Cargo.toml`
- Add student-visible tests
- Add task 2-4 under `src/keys.rs`
- Update docs regarding submission format

Sept 5th:
- Update docs regarding submission format

Sept 4th:
- Clarify docs for `AnagramClasses::lookup`
- Fix docs for `AnagramClasses:::lookup` where `Ok` was used instead of `Some`
- Define class key in comments for `AnagramClasses::new`

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
maxagrams, both for individual words and for pairs of words.

As an anti-procrastination measure, the first task will be due on
Tuesday, Sep 8 at 11:59 PM.  The remaining tasks will be due one week
after that on Mon, Sep 14 at 11:59 PM.  All submissions will be due
via Gradescope.  You make work alone or with a partner; in the latter
case, please make a single submission.

For the check-in, you may submit `anagrams.rs` on its own, or a zip
file containing the full package, including `Cargo.toml` and the
`src/*.rs` files.  You may also submit a `crate` created with the
`crate package` command.

For the final submission, please create a `crate` file created with
the `crate package` command.

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
has a bit under 173K English words.

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
  of classes of different sizes, largest class size)o
- Look up an anagram class by a query string
- Return the word lists for all maxagrams

### Task 2: Integer keys

A 26-byte histogram works fine as a key for identifying anagram
classes.  But an ordinary 64-bit integer is more compact, and 64-bit
numbers enjoy hardware support that our histogram type does not have.
Can we encode histograms as 64-bit integers?  Yes!

#### Hash functions

Suppose $\mathcal{S}$ is the set of strings over our alphabet.
Let's define a set of *histogram hash functions* on $\mathcal{S}$ of the form
$$
  h(s) = \sum_{c \in s} u(c)
$$
where $u$ maps a character in our alphabet to a value in
$\mathbb{Z}/(N\mathbb{Z})$.

We define a trait `Key` in `keys.rs` that can be implemented by a
`struct` type representing histogram hashes.  The trait has an
associated function `new` to create a histogram hash from a string.
Types implementing `Key` must also implement several other traits,
including `Add` and `Sub` (both of which should be implemented modulo
$N$).

```rs
/// Trait for histogram hash key
pub trait Key: Clone + Copy + Add<Output = Self> + Sub<Output = Self> +
    PartialEq + Eq + PartialOrd + Ord {
    fn new(s: &str) -> Self; // Map a string to a histogram hash key
}
```

We give a partial implementation for a particular hash key type:

```rs
pub struct HashKey64(u64);
```

For the `HashKey64` type, we set $N$ to $2^{64}$, corresponding to
wrappped arithmetic with `u64` values.  We provide an array
`HIST_CHAR_VAL` for the character values (the $u(c)$) for an example;
these were generated by a pseudo-random number generator.

Each histogram hash function defines its own equivalence relation,
which may in general be coarser than the `is_anagram` relation.  That
is, if $s$ and $t$ are anagrams then we will always have $h(s) =
h(t)$, but the converse may not be true.  Non-anagrams that map to the
same hash key are said to *collide*.  While the `HashKey64` hashes are
collision-free for all of our sample dictionaries, dealing with
collisions may be needed in general.

#### Index structure

For functionality similar to what we had with `AnagramsClasses`,
we define a type `DictKeyIndex<'d, T>` that takes a lifetime parameter
`'d` and a hash key type `T`.  This struct borrows a reference to a
dictionary and constructs a vector of `(key, index)` pairs in sorted
order.  To look up an anagram using this data structure, we can find
the first index where the key might occur (e.g. using
[`partition_point`][partition_point]) and then compare check each
dicionary entry with a key that matches the lookup string key.
Because of the possibility of collisions, we need to actually compare
the histograms for these potential matches, and not just the hash
keys.

We do not provide offset ranges for the hash key classes in this case.
Instead, we provide an iterator for walking through the hash key
classes in sequence.  The iterator returns a slice into the index
vector, which is useful in various ways.

The machinery for the integer key indexing is in `src/keys.rs`.  Once
you have finished implementing the `HashKey64`, you should complete
the definition of `DictKeyIndex` and `KeyClassIterator` and their
associated implementation blocks.

We provide a test harness in `src/keys_test.rs` (we have also added
a Cargo test harness for the task 1 in `src/anagram_test.rs`).
Though your `lookup` method is supposed to work even when there are
collisions, that functionality just using `HashKey64`; you will
want to edit `src/keys_test.rs` accordingly to add this check.

## Task 3: Word pair anagrams

The `lookup2` function in `src/keys.rs` computes *word pair* anagrams;
that is, given a target word or pair of words $t$, find all pairs of words
$w_1$ and $w_2$ such that the letters in $w_1$ and $w_2$ together are
an anagram of the target word(s).

To solve this problem, we recommend a strategy of iterating over
dictionary words $w_1$ and then checking every word $w_2$ such that
$h(w_2) = h(t) - h(w_2)$.  It *is* necessary to check every word

## Task 4: Maximal word pair anagrams (for fun)

Even the larger of our two dictionaries (`enable1.txt`) is small by
modern standards.  There are about 173K words, averaging a little over
nine characters each; including newline characters, the file is only
about 1.74MB.  We can use the Unix `wc` command to see the actual
number of lines, words, and characters:
```
% wc enable1.txt
  172823  172823 1743363 enable1.txt
```
Storing the histograms for these words takes more space than storing
the words themselves, a total of about 4.5MB.  But 4.5MB is still
modest relative to modern memory sizes.

Things change enormously if we consider general manipulation of pairs
of words.  While iterating through word pairs is entirely feasible,
*storing* an index structure for those word pairs is too much even
on most modern computers.  Moreover, the issue of collisions can no
longer be so easily ignored.

The challenge, then, for the interested student: what are the
maxagrams over *word pairs* for the `enable1.txt` and `popular.txt`
dictionaries?  And how quickly can you compute them?

[partition_point]: https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
