use crate::serialization::Recipe;

use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Dir {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl Dir {
    pub fn new(name: &str) -> Dir {
        Dir {
            name: name.to_owned(),
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub created_on: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub recipe: Recipe,
    pub path: String,
}

use crate::{filters, git, serialization};

use std::path::Path;

use id_tree::InsertBehavior;
use id_tree::*;

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
        let mut cur_node_id = self
            .tree
            .root_node_id()
            .expect("root node should already be created")
            .clone();
        for comp in path.components() {
            let dir = comp.as_os_str().to_str().unwrap();
            if filters::is_filename_yaml(dir) {
                self.insert_entry(&cur_node_id, path);
                return;
            } else {
                cur_node_id = self.insert_dir(&cur_node_id, dir);
            }
        }
    }

    pub fn insert_dir(&mut self, node_id: &NodeId, dir_name: &str) -> NodeId {
        if let Some(node_id) = self.find_by_id(node_id, dir_name) {
            node_id
        } else {
            let new_node_id = self
                .tree
                .insert(
                    Node::new(Dir::new(dir_name)),
                    InsertBehavior::UnderNode(&node_id),
                )
                .unwrap();
            new_node_id
        }
    }

    pub fn insert_entry(&mut self, node_id: &NodeId, path: &Path) {
        let recipe = serialization::read_recipe(path);
        let created_on = git::created_on(&path);
        let last_modified = git::last_modified(&path);

        self.tree
            .get_mut(node_id)
            .unwrap()
            .data_mut()
            .entries
            .push(Entry {
                created_on: created_on,
                last_modified: last_modified,
                recipe: recipe,
                path: path.to_str().unwrap().to_owned(),
            });
    }

    fn find_by_id(&self, node_id: &NodeId, dir_name: &str) -> Option<NodeId> {
        self.tree
            .children_ids(node_id)
            .unwrap()
            .find(|cur_node_id| self.tree.get(cur_node_id).unwrap().data().name == dir_name)
            .map(|node| node.clone())
    }
}
