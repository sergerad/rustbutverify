use crate::Error;
use core::hash::Hash;
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Immutable reference to a value in the map (RAII guarded).
pub use dashmap::mapref::one::Ref;
/// Mutable reference to a value in the map (RAII guarded).
pub use dashmap::mapref::one::RefMut;

/// The trait that all keys for [`DurableKv`] are bound to.
pub trait Key: Eq + Hash + Serialize {}
impl<T: Eq + Hash + Serialize> Key for T {}

/// The trait that all values for [`DurableKv`] are bound to.
pub trait Value: Serialize {}
impl<T: Serialize> Value for T {}

/// A durable, thread-safe, in-memory key-value store.
pub struct DurableKv<K, V>
where
    K: Key,
    V: Value,
{
    db_file_path: PathBuf,
    dmap: DashMap<K, V>,
}

impl<K: Key + DeserializeOwned, V: Value + DeserializeOwned> DurableKv<K, V> {
    /// Constructs a `DurableKv` instance based on a DB file path.
    ///
    /// If the file is non-empty, attempts to deserialize its contents
    /// as the key-value store itself.
    pub fn new(file_path: impl AsRef<Path>) -> Result<Self, Error> {
        let file_path = file_path.as_ref();

        if Path::exists(file_path) {
            // Deserialize the kv from file.
            let mut raw = Vec::new();
            File::open(file_path)?.read_to_end(&mut raw)?;
            let dmap = bincode::deserialize::<DashMap<K, V>>(&raw)?;
            Ok(Self {
                db_file_path: file_path.to_path_buf(),
                dmap,
            })
        } else {
            // Empty kv.
            Ok(Self {
                db_file_path: file_path.to_path_buf(),
                dmap: DashMap::default(),
            })
        }
    }
}

impl<'a, K: Key, V: Value> DurableKv<K, V> {
    /// Inserts a value into the store.
    ///
    /// Returns the existing value for the respective key
    /// if one exists.
    pub fn put(&self, key: K, value: V) -> Option<V> {
        self.dmap.insert(key, value)
    }

    /// Retrieves a reference to a value from the store.
    pub fn get_ref(&'a self, key: K) -> Option<Ref<'a, K, V>> {
        self.dmap.get(&key)
    }

    /// Retrieves a mutable reference to a value from the store.
    pub fn get_mut(&'a self, key: K) -> Option<RefMut<'a, K, V>> {
        self.dmap.get_mut(&key)
    }
}

impl<K: Key, V: Value + Copy> DurableKv<K, V> {
    /// Retrieves a value from the store.
    pub fn get(&self, key: K) -> Option<V> {
        self.dmap.get(&key).as_deref().copied()
    }
}

impl<K: Key, V: Value + Clone> DurableKv<K, V> {
    /// Retrieves a value from the store and clones it.
    pub fn get_cloned(&self, key: K) -> Option<V> {
        self.dmap.get(&key).as_deref().cloned()
    }
}

impl<K: Key, V: Value> DurableKv<K, V> {
    /// Serializes and stores the `DurableKv` to file.
    ///
    /// Should only be called by `DurableKv::drop` in order
    /// to avoid parallel file writes. A lock could be added to
    /// `DurableKv` to allow this fn to be public.
    fn commit(&self) -> Result<(), Error> {
        // Serialize the kv.
        let bin = bincode::serialize(&self.dmap)?;

        // Write the serialized kv to file.
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&self.db_file_path)?;
        file.write_all(&bin)?;
        Ok(())
    }
}

impl<K: Key, V: Value> Drop for DurableKv<K, V> {
    /// Dumps the store's contents to file.
    ///
    /// # Panics:
    /// - If the store's contents cannot be serialized and stored
    ///   to file for any reason.
    fn drop(&mut self) {
        self.commit().expect("Failed to commit on drop")
    }
}

#[cfg(test)]
mod tests {
    use super::DurableKv;
    use rand::{distr::Alphanumeric, Rng};
    use std::{path::PathBuf, sync::Arc, thread};
    use temp_testdir::TempDir;

    /// Creates a random file path in a temporary dir.
    /// The returned [`TempDir`] owns the system directory and will delete it when dropped.
    fn random_file_path() -> (TempDir, PathBuf) {
        let dir_path = TempDir::default();
        let mut file_path = PathBuf::from(dir_path.as_ref());
        let s: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        file_path.push(s);
        (dir_path, file_path)
    }

    #[test]
    fn put() {
        let (_dir, file_path) = random_file_path();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

        // Put into empty.
        assert_eq!(None, kv.put("hello".to_string(), 0));
        // Put into non-empty.
        assert_eq!(Some(0), kv.put("hello".to_string(), 1));
        assert_eq!(Some(1), kv.put("hello".to_string(), 2));
    }

    #[test]
    fn get() {
        let (_dir, file_path) = random_file_path();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

        // Get non-existent.
        assert_eq!(None, kv.get("hello".to_string()));
        // Put into empty.
        assert_eq!(None, kv.put("hello".to_string(), 1));
        // Get existent.
        assert_eq!(Some(1), kv.get("hello".to_string()));
    }

    #[test]
    fn get_ref() {
        let (_dir, file_path) = random_file_path();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

        // Get non-existent.
        assert_eq!(None, kv.get_ref("hello".to_string()).as_deref());
        // Put into empty.
        assert_eq!(None, kv.put("hello".to_string(), 1));
        // Get existent.
        assert_eq!(Some(&1), kv.get_ref("hello".to_string()).as_deref());
    }

    #[test]
    fn get_mut() {
        let (_dir, file_path) = random_file_path();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

        // Get non-existent.
        assert_eq!(None, kv.get_mut("hello".to_string()).as_deref());
        // Put into empty.
        assert_eq!(None, kv.put("hello".to_string(), 1));
        // Get existent.
        assert_eq!(Some(&1), kv.get_mut("hello".to_string()).as_deref());

        // Mutate
        *kv.get_mut("hello".to_string()).unwrap() = 99;
        assert_eq!(Some(&99), kv.get_mut("hello".to_string()).as_deref());
    }

    #[test]
    fn commit() {
        let (_dir, file_path) = random_file_path();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

        // Put into empty.
        assert_eq!(None, kv.put("hello".to_string(), 0));
        // Get existent.
        assert_eq!(Some(0), kv.get("hello".to_string()));

        // Commit and re-open db file.
        kv.commit().unwrap();
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();
        // Get existent from previous.
        assert_eq!(Some(0), kv.get("hello".to_string()));
    }

    #[test]
    fn drop() {
        let (_dir, file_path) = random_file_path();
        // Store and commit via drop.
        {
            let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();

            // Put into empty.
            assert_eq!(None, kv.put("hello".to_string(), 0));
            // Get existent.
            assert_eq!(Some(0), kv.get("hello".to_string()));
        }

        // Re-open db file after drop commit.
        let kv: DurableKv<String, i32> = DurableKv::new(&file_path).unwrap();
        // Get existent from previous.
        assert_eq!(Some(0), kv.get("hello".to_string()));
    }

    #[test]
    fn put_concurrent() {
        let (_dir, file_path) = random_file_path();

        // Perform tests in scope to drop commit.
        {
            // Create Arc clones.
            let kv: DurableKv<String, String> = DurableKv::new(&file_path).unwrap();
            let kv_arc = Arc::new(kv);
            let kv_arc_clone = kv_arc.clone();
            let kv_arc_clone_two = kv_arc.clone();

            // Use clones in threads.
            let mut handles = Vec::new();
            handles.push(thread::spawn(move || {
                kv_arc.put("0".to_string(), "0".to_string());
            }));
            handles.push(thread::spawn(move || {
                kv_arc_clone.put("1".to_string(), "1".to_string());
            }));

            // Wait for every thread.
            handles.into_iter().for_each(|h| h.join().unwrap());

            // Check values were written.
            assert_eq!(
                Some("0".to_string()),
                kv_arc_clone_two.get_cloned("0".to_string())
            );
            assert_eq!(
                Some("1".to_string()),
                kv_arc_clone_two.get_cloned("1".to_string())
            );
        }
        // Re-open db file after drop commit.
        let kv: DurableKv<String, String> = DurableKv::new(&file_path).unwrap();
        // Get existent from previous.
        assert_eq!(Some("0".to_string()), kv.get_cloned("0".to_string()));
        assert_eq!(Some("1".to_string()), kv.get_cloned("1".to_string()));
    }

    #[test]
    fn get_concurrent() {
        let (_dir, file_path) = random_file_path();

        // Create kv and store values.
        let kv: DurableKv<String, String> = DurableKv::new(&file_path).unwrap();
        kv.put("0".to_string(), "0".to_string());
        kv.put("1".to_string(), "1".to_string());

        // Create clones.
        let kv_arc = Arc::new(kv);
        let kv_arc_clone = kv_arc.clone();

        // Use clones in threads.
        let mut handles = Vec::new();
        handles.push(thread::spawn(move || {
            assert_eq!(Some("0".to_string()), kv_arc.get_cloned("0".to_string()));
            assert_eq!(Some("1".to_string()), kv_arc.get_cloned("1".to_string()));
        }));
        handles.push(thread::spawn(move || {
            assert_eq!(
                Some("0".to_string()),
                kv_arc_clone.get_cloned("0".to_string())
            );
            assert_eq!(
                Some("1".to_string()),
                kv_arc_clone.get_cloned("1".to_string())
            );
        }));

        // Wait for every thread.
        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}
