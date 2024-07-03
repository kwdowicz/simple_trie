use std::collections::HashMap;

/// A node in a Trie data structure.
///
/// Each node stores a map of child nodes (`children`) indexed by characters,
/// and a boolean flag (`is_end_of_word`) indicating whether the node represents the end of a valid word.
#[derive(Debug)]
pub struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_word: bool,
}

/// A Trie (prefix tree) data structure for efficient string prefix matching.
///
/// A Trie stores a set of strings in a tree structure, where each node represents a character in a string,
/// and paths from the root to nodes marked as `is_end_of_word` represent complete words.
#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl TrieNode {
    /// Creates a new `TrieNode` with an empty `children` map and `is_end_of_word` set to `false`.
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

impl Trie {
    /// Creates a new, empty `Trie` with a root node.
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// Inserts a word into the `Trie`.
    ///
    /// This method iterates over the characters of the word, creating new nodes or following existing ones as needed.
    /// The `is_end_of_word` flag is set to `true` on the last node to mark the end of the inserted word.
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for char in word.chars() {
            node = node.children.entry(char).or_insert(Box::new(TrieNode::new()));
        }
        node.is_end_of_word = true;
    }

    /// A private helper method for searching for a word or prefix in the Trie.
    ///
    /// Returns `true` if the word or prefix is found, `false` otherwise.
    fn search(&mut self, word: &str, prefix: bool) -> bool {
        let mut node = &self.root;
        for char in word.chars() {
            if let Some(next_node) = node.children.get(&char) {
                node = next_node;
            } else {
                return false;
            }
        }
        if prefix {
            return true; // If searching for a prefix, any match is sufficient
        } else {
            node.is_end_of_word // If searching for a full word, check if we're at the end of a word
        }
    }

    /// Searches for a prefix in the `Trie`.
    ///
    /// Returns `true` if the prefix exists in the `Trie`, `false` otherwise.
    pub fn search_prefix(&mut self, word: &str) -> bool {
        self.search(word, true)
    }

    /// Searches for a full word in the `Trie`.
    ///
    /// Returns `true` if the exact word exists in the `Trie`, `false` otherwise.
    pub fn search_full_world(&mut self, word: &str) -> bool {
        self.search(word, false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert("abcde");
        assert_eq!(trie.search_prefix("abc"), true);
        assert_eq!(trie.search_full_world("abcde"), true);
        assert_eq!(trie.search_full_world("abc"), false);
    }
}
