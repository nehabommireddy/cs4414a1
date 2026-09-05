//! Process anagrams

/// Letter histogram type
pub type LetterHist = [u8; 26];

/// Make a histogram from a word
pub fn histogram(word: &str) -> LetterHist {
    todo!("Compute word histogram");
}

/// Storage for anagram classes and their histogram keys
pub struct AnagramClasses {
    pub class_keys: Vec<LetterHist>, // Histograms for each class
    pub class_offsets: Vec<usize>,   // off[i]..of[i+1] is range for class i
    pub word_ids: Vec<usize>,        // List of word IDs
}

/// Recommended key-value pair for sorting/searching anagrams
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AnagramKV(LetterHist, usize);

impl AnagramClasses {
    /// Set up a new Anagrams struct
    pub fn new(dict: &[String]) -> Self {
        // Recommended strategy:
        //
        // - Form a vector of (histogram, word id) pairs
        // - Sort the vector (you can use the sort method)
        // - Get the id part of the entries; this will be word_ids
        // - Find the start of each equivalence class and store in
        //   the class_offsets vector (and store the key in class_keys); the
        //   index of a class's histogram in `class_keys` and its offset in
        //   `class_offsets` should be the same and we will refer to this index
        //   as the class key
        // - Add the number of words to the end of the class_offsets vector
        // - Form and return the struct
        //
        // There's more than one way to do this.  I used a functional
        // approach (and used dedup_by_key to get the class offsets /
        // keys).

        // Placeholder code so that everything compiles initially
        let word_ids = Vec::new();
        let class_keys = Vec::new();
        let class_offsets = Vec::new();

        Self {
            class_keys,
            class_offsets,
            word_ids,
        }
    }

    /// Get the number of classes
    pub fn num_classes(&self) -> usize {
        self.class_keys.len()
    }

    /// Get the length of a class
    pub fn len_class(&self, id: usize) -> usize {
        self.class_offsets[id + 1] - self.class_offsets[id]
    }

    /// Get length of longest class. Returns 0 if `self.num_classes() == 0`.
    pub fn len_class_max(&self) -> usize {
        todo!("Find the length of the longest anagram class")
    }

    /// Get the class lengths
    pub fn len_class_hist(&self) -> Vec<usize> {
        todo!("Return a vector where len[i] is the number of classes of len i+1");
    }

    /// Get anagram class by index
    pub fn get_class(&self, id: usize) -> &[usize] {
        let lo = self.class_offsets[id];
        let hi = self.class_offsets[id + 1];
        &self.word_ids[lo..hi]
    }

    /// Look up an anagram class. Returns `Some(ids)`, the word ids of class whose members are anagrams of `word`, or
    /// `None` if no such class exists.
    pub fn lookup(&self, word: &str) -> Option<&[usize]> {
        todo!()
    }

    /// Get maxagram classes. Returns a vector of all class ids for maxagram classes.
    pub fn maxagrams(&self) -> Vec<usize> {
        todo!()
    }
}
