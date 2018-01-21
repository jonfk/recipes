
use std::path::Path;

pub mod id_tree;

pub trait Store {
    type Key;

    fn insert_path(&mut self, path: &Path);
    fn insert_dir(&mut self, parent_key: &Self::Key, dir: &str);
    fn insert_entry(&mut self, parent_key: &Self::Key, name: &str, path: &str);
}
