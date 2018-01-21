
use filters;
use model::{Dir, Entry};

use std::path::Path;

use id_tree::*;
use id_tree::InsertBehavior;

pub struct IdTree {
    pub tree: Tree<Dir>,
}

impl IdTree {
    pub fn new() -> IdTree {
        let mut tree: Tree<Dir> = TreeBuilder::new().build();
        tree.insert(Node::new(Dir::new("root")), InsertBehavior::AsRoot)
            .unwrap();

        IdTree { tree: tree }
    }

    pub fn insert_path(&mut self, path: &Path) {
        let mut cur_node_id =
            self.tree.root_node_id().expect("root node should already be created").clone();
        for comp in path.components() {

            let dir = comp.as_os_str().to_str().unwrap();
            if filters::is_filename_yaml(dir) {
                self.insert_entry(&cur_node_id, dir, path.to_str().unwrap());
                return;
            } else {
                cur_node_id = self.insert_dir(&cur_node_id, dir);
            }
        }
    }

    pub fn insert_dir(&mut self, node_id: &NodeId, dir_name: &str) -> NodeId {
        if let Some(node_id) = find_from_id(&self.tree, node_id, dir_name) {
            node_id
        } else {
            let new_node_id = self.tree
                .insert(Node::new(Dir::new(dir_name)),
                        InsertBehavior::UnderNode(&node_id))
                .unwrap();
            new_node_id
        }
    }

    pub fn insert_entry(&mut self, node_id: &NodeId, file_name: &str, path: &str) {
        self.tree.get_mut(node_id).unwrap().data_mut().entries.push(Entry {
            name: file_name.to_owned(),
            path: path.to_owned(),
        });
    }
}



fn find_from_id(tree: &Tree<Dir>, node_id: &NodeId, dir_name: &str) -> Option<NodeId> {
    tree.children_ids(node_id)
        .unwrap()
        .find(|cur_node_id| tree.get(cur_node_id).unwrap().data().name == dir_name)
        .map(|node| node.clone())
}
