
use storage::id_tree::IdTree;
use model::Dir;

use id_tree::{NodeId, Node};

pub fn output(itree: &IdTree) {
    let mut contents = String::new();

    for node in itree.tree.traverse_pre_order(itree.tree.root_node_id().unwrap()).unwrap() {
        let depth = depth(itree, node);
        println!("{:?}, depth: {}", node.data(), depth);
        if node.data().name != "root" {
            contents.push_str(&format!("\n{} {}\n\n", "#".repeat(depth), node.data().name));
            for entry in &node.data().entries {
                contents.push_str(&format!("- [{}]({})\n", entry.name, entry.path));
            }
        }
    }

    println!("{}", contents);
    write_to_file("contents.md", &contents);
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
    file.write_all(contents.as_bytes()).expect("error writing to file");
}
