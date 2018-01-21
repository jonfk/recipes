extern crate walkdir;
extern crate id_tree;

pub mod storage;
pub mod filters;
pub mod model;
pub mod output;

use model::{Dir, Entry};
use storage::id_tree as itree;

use std::path::Path;

use walkdir::{DirEntry, WalkDir};

pub fn run() {
    let dirs = vec!["baking", "desserts", "meals"];

    let mut storage = itree::IdTree::new();

    println!("indexing...");
    for dir in dirs {
        let walker = WalkDir::new(dir).into_iter();
        for entry in walker.filter_entry(|e| !filters::is_hidden(e)) {
            let entry = entry.unwrap();

            if filters::is_yaml(&entry) {
                let path = entry.path();
                //println!("{}", path.display());
                storage.insert_path(path);
            }
        }
    }

    // for node in storage.tree.traverse_pre_order(storage.tree.root_node_id().unwrap()).unwrap() {
    //     println!("{:?}, ", node.data());
    // }

    output::output(&storage);
}


pub fn deserialize_yaml() {}
