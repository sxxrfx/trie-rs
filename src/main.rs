use trie_rs::Trie;

fn main() {
     let mut trie = Trie::new();
     trie.insert("hello");
     trie.insert("hi");
     trie.insert("hey");
     trie.insert("world");

     
     println!("{trie:#?}");

     println!("hiiii {}", trie.contains("hiiii"));

 }
