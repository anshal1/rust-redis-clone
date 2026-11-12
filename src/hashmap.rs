pub mod MyMap {
    use std::{
        hash::{DefaultHasher, Hash, Hasher},
        panic, str,
    };
    #[derive(Debug, Clone)]
    pub struct HashItem {
        key: String,
        value: String,
        hash: u64,
        next: Option<Box<HashItem>>,
    }
    pub struct HashMap {
        map: Vec<Option<Box<HashItem>>>,
        len: usize,
    }

    impl HashMap {
        pub fn new(len: Option<usize>) -> Self {
            let mut map: Vec<Option<Box<HashItem>>> = Vec::new();
            if let Some(lenght) = len {
                for _ in 0..lenght {
                    map.push(None);
                }
                HashMap { map, len: lenght }
            } else {
                for _ in 0..20 {
                    map.push(None);
                }
                HashMap { map, len: 20 }
            }
        }
        pub fn hash_key_and_get_index(&mut self, key: &String) -> (u64, usize) {
            let mut hash = DefaultHasher::new();
            key.hash(&mut hash);
            let hash_data = hash.finish();
            let hash_result = match usize::try_from(hash_data) {
                Ok(h_s) => h_s,
                Err(e) => {
                    panic!("Something went wrong with hashing the key, {}", e)
                }
            };
            return (hash_data, hash_result % self.len);
        }
        pub fn add(&mut self, key: String, value: String) -> u8 {
            let (hash, idx) = self.hash_key_and_get_index(&key);
            let old_node = self.map[idx].take();
            self.map[idx] = Some(Box::new(HashItem {
                key,
                value,
                hash,
                next: old_node,
            }));
            return 0;
        }
        pub fn get(&mut self, key: &str) -> Result<String, String> {
            let mut val = String::new();
            let (hash, idx) = self.hash_key_and_get_index(&key.to_string());
            let mut item = match self.map.get(idx) {
                Some(item) => item,
                None => {
                    return Err("None".to_string());
                }
            };
            loop {
                let current_item = if let Some(it) = &*item {
                    it
                } else {
                    return Err("None".to_string());
                };

                if current_item.hash == hash {
                    val.push_str(&current_item.value);
                    return Ok(val);
                } else {
                    item = &current_item.next
                }
            }
        }

        pub fn del(&mut self, key: &str) -> Result<u8, String> {
            // check if the index only has one item.
            let (hash, idx) = self.hash_key_and_get_index(&key.to_string());
            let mut current = &mut self.map[idx];
            loop {
                let matches = match current.as_ref() {
                    Some(item) => item.hash == hash,
                    None => return Err("No key found".to_string()),
                };
                if matches {
                    let mut to_remove = current.take().ok_or("No key found".to_string())?;
                    *current = to_remove.next.take();
                    return Ok(0);
                }
                current = &mut current.as_mut().unwrap().next;
            }
        }
    }
}
