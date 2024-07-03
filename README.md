# Rust Trie Implementation

A Trie (pronounced "try") or prefix tree is a specialized tree-like data structure used for efficient storage and retrieval of strings. It provides fast operations for prefix-based searches and is particularly useful in applications like autocomplete, spell checking, and IP routing.

## Features

- **Efficient Prefix Searching:** Quickly search for words or prefixes in a set of strings.
- **Memory Optimization:**  Shares common prefixes among words, reducing storage overhead.
- **Customizable:** Easily adaptable for various use cases.

## Usage

1. Cargo.toml:

```toml
[dependencies]
simple_trie = "0.1.0"
```

2. main.rs:

```rust
use simple_trie::Trie;

let mut my_trie = Trie::new();

my_trie.insert("hello");
my_trie.insert("world");
my_trie.insert("help");

// Search for a full word
let exists = my_trie.search_full_world("hello"); // true
// Search for a prefix
let prefix_exists = my_trie.search_prefix("hel"); // true
```
## Testing

Run tests with:

```bash
cargo test
```

### License

This project is licensed under the MIT License.