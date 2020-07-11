use crate::model::{Dir, IdTree};

use id_tree::Node;
use inflections::Inflect;
use std::fmt::Write;

pub fn output(itree: &IdTree) {
    let mut contents = String::new();

    for node in itree
        .tree
        .traverse_pre_order(itree.tree.root_node_id().unwrap())
        .unwrap()
    {
        let depth = depth(itree, node);
        println!("{:?}, depth: {}", node.data(), depth);
        if node.data().name != "root" {
            contents.push_str(&format!(
                "\n{} {}\n\n",
                "#".repeat(depth),
                node.data().name.to_title_case()
            ));
            for entry in &node.data().entries {
                write!(&mut contents, "- [{}]({})", entry.recipe.name(), entry.path).unwrap();
                if let Some(desc) = &entry.recipe.description {
                    write!(&mut contents, ": {}\n", desc).unwrap();
                } else {
                    write!(&mut contents, "\n").unwrap();
                }
            }
        }
    }

    println!("{}", contents);
    write_to_file("contents.md", &contents);
}

pub fn output_json_contents(itree: &IdTree) {
    let recipes: Vec<_> = itree
        .tree
        .traverse_pre_order(itree.tree.root_node_id().unwrap())
        .unwrap()
        .map(|node| node.data().entries.clone())
        .flatten()
        .collect();

    let recipes_json = serde_json::to_string(&recipes).unwrap();
    write_to_file("recipes.json", &recipes_json);
}

fn depth(tree: &IdTree, node: &Node<Dir>) -> usize {
    if node.parent().is_none() {
        0
    } else {
        let mut depth = 1;
        let mut node = tree.tree.get(node.parent().unwrap()).unwrap();
        while node.parent().is_some() {
            depth += 1;
            node = tree.tree.get(node.parent().unwrap()).unwrap();
        }
        depth
    }
}

fn write_to_file(output_file: &str, contents: &str) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(output_file).expect("error creating file");
    file.write_all(contents.as_bytes())
        .expect("error writing to file");
}
