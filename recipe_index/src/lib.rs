extern crate inflections;
extern crate walkdir;
extern crate id_tree;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod storage;
pub mod filters;
pub mod model;
pub mod output;
pub mod serialization;

use model::{Dir, Entry};
use storage::id_tree as itree;

use std::path::Path;

use walkdir::{DirEntry, WalkDir};

pub fn run() {
    let dirs = vec!["baking", "desserts", "meals", "drinks", "salads"];

    let mut storage = itree::IdTree::new();

    println!("indexing...");
    for dir in dirs {
        let walker = WalkDir::new(dir).into_iter();
        for entry in walker.filter_entry(|e| !filters::is_hidden(e)) {
            let entry = entry.unwrap();

            if filters::is_yaml(&entry) {
                let path = entry.path();
                //let recipe = read_recipe(path);
                //println!("{}", path.display());
                //println!("{:?}", recipe);
                storage.insert_path(path);
            }
        }
    }

    // for node in storage.tree.traverse_pre_order(storage.tree.root_node_id().unwrap()).unwrap() {
    //     println!("{:?}, ", node.data());
    // }

    output::output(&storage);
}
