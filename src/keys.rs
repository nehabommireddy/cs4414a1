/// Process histogram keys
use crate::anagrams;
use std::ops::{Add, Sub};

/// Trait for histogram hash key
pub trait Key:
    Clone + Copy + Add<Output = Self> + Sub<Output = Self> + PartialEq + Eq + PartialOrd + Ord
{
    fn new(s: &str) -> Self; // Map a string to a histogram hash key
}

/// Implementation for a 64-bit hash key
/// TODO: Derive all the required traits
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashKey64(u64);

impl HashKey64 {
    /// Array mapping each character to a pseudorandom 64-bit value
    const HIST_CHAR_VAL: [u64; 26] = [
        12574092071919236688,
        7473479917386431180,
        17688396586958240970,
        16988523605502484928,
        8743134980943255681,
        1951701038415341461,
        10832840116491959655,
        10249001058406476492,
        4503371295589978965,
        16200870910380599828,
        7923536944881947025,
        6073912002436141577,
        5629334492774015757,
        3509246976664838924,
        1827825826472368150,
        4484647854781889018,
        6083457950696995301,
        138862978672854273,
        13334089059697626189,
        15088915516239810995,
        7199188598562822861,
        9396154019677091704,
        9636747050973341286,
        2660062595604763090,
        17488327319427655727,
        1641055753520184748,
    ];
}

/// TODO: Implement Add, Sub, and Key traits on HashKey64
/// NB: You should handle upper case (by conversion to lower)
///     and non-ASCII-alphabetic characters (by skipping them)

impl Key for HashKey64 {
    fn new(s: &str) -> Self {
        let mut value = 0;
        for c in s.bytes() {
            let c = c.to_ascii_lowercase();
            if c >= b'a' && c <= b'z' {
                let num = (c as u8 - b'a') as usize;
                value += HashKey64::HIST_CHAR_VAL[num];
            }
        }
        HashKey64(value)
    }
}

impl Add for HashKey64 {
    type Output = Self;

    fn add(self, other:Self) -> Self {
        HashKey64(self.0.wrapping_add(other.0))
    }
}

impl Sub for HashKey64 {
    type Output = Self;

    fn sub(self, other:Self) -> Self {
        HashKey64(self.0.wrapping_sub(other.0))
    }
}

/// Key-based dictionary index data type
#[derive(Debug)]
pub struct DictKeyIndex<'d, T: Key> {
    dict: &'d [String],
    index: Vec<(T, usize)>
}

/// TODO: Add an iterator for the DictKeyIndex
pub struct KeyClassIterator<'a, T: Key> {
    index: &'a [(T, usize)],
    position: usize,
}

/// TODO: Implement the Iterator trait for KeyClassIterator<'a, T>
/// NB: The Item type should be a slice of the index array from DictKeyIndex.
impl<'a, T: Key> Iterator for KeyClassIterator<'a, T> {
    type Item = &'a [(T, usize)];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.index.len() {
            return None;
        }

        let start = self.position;
        let current_key = self.index[start].0;

        while self.position < self.index.len()
            && self.index[self.position].0 == current_key
        {
            self.position += 1;
        }

        Some(&self.index[start..self.position])
    }
}

impl<'d, T> DictKeyIndex<'d, T>
where
    T: Key,
{
    /// Create a new index (you will need to add a lifetime annotation here)
    pub fn new(dict: &'d [String]) -> Self {
        let mut index = Vec::new();
        for (i, value) in dict.iter().enumerate() {
            let key = T::new(value);
            index.push((key, i));
        }
        index.sort();
        DictKeyIndex {
            dict,
            index,
        }
    }

    /// Return true iff the histogram equivalence and anagram equivalence
    /// are the same for this dictionary.
    pub fn keys_are_unique(&self) -> bool {
        for class in self.classes() {
        if class.len() <= 1 {
            continue;
        }

        let first_index = class[0].1;
        let first_hist = anagrams::histogram(&self.dict[first_index]);

        for (_, dict_index) in &class[1..] {
            let hist = anagrams::histogram(&self.dict[*dict_index]);

            if hist != first_hist {
                return false;
                }
            }
        }
        true
    }

    /// Return an iterator through the hash key classes
    pub fn classes<'a>(&'a self) -> KeyClassIterator<'a, T> {
        KeyClassIterator {
        index: &self.index,
        position: 0,
        }
    }

    /// Get the number of hash key classes
    pub fn num_classes(&self) -> usize {
        self.classes().count()
    }

    /// Get the maximum hash key class length
    pub fn len_class_max(&self) -> usize {
        self.classes().map(|c| c.len()).max().unwrap_or_default()
    }

    /// Make a histogram of hash key class lengths
    pub fn len_class_hist(&self) -> Vec<usize> {
        let mut hist: Vec<usize> = vec![0; self.len_class_max()];
        for c in self.classes() {
            hist[c.len()-1] += 1;
        }
        hist
    }

    /// Get the maximal classes according to the hash function.
    /// This may differ from the maxagrams if there are collisions.
    pub fn maxagrams(&self) -> Vec<Vec<usize>> {
        let max_len = self.len_class_max();
        let mut result = Vec::new();
        for class in self.classes() {
            if class.len() == max_len {
                let mut indices = Vec::new();
                for (_, i) in class {
                indices.push(*i);
                }
                result.push(indices);
            }
        }
        result
    }

    /// Look up class (should work even if the keys are non-unique)
    /// Return an empty vector if there is no match
    pub fn lookup(&self, s: &str) -> Vec<usize> {
        todo!("Look up an anagram class (may be finer than a hash class)")
    }

    /// Look up all word pairs that anagram to the target, where words
    /// are represented by their dictionary indices.  Each pair should
    /// occur only once, i.e. if (id1,id2) appears then (id2,id1)
    /// should not appear.  Double words are allowed, i.e. (id1,id1).
    pub fn lookup2(&self, s: &str) -> Vec<(usize, usize)> {
        todo!("Look up all unique word pairs that anagram to s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_addsub_hash64() {
        let h1 = HashKey64::new("cat");
        let h2 = HashKey64::new("chore");
        let h3 = HashKey64::new("catch");
        let h4 = HashKey64::new("ore");
        assert_eq!(h1 + h2, h3 + h4);
        assert_eq!(h1 + h2 - h3, h4);
    }
}
