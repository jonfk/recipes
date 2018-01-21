
use serde_yaml;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub source: Option<Source>,
    pub links: Option<Vec<String>>,
    pub ingredients: Vec<Ingredient>,
    pub notes: Option<Vec<String>>,
    pub instructions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Seq(Vec<String>),
    Str(String),
}

pub fn deserialize_recipe(recipe_yaml: &str) -> Recipe {
    serde_yaml::from_str(recipe_yaml).expect(&format!("deserialize recipe {}", recipe_yaml))
}

pub fn read_file(path: &Path) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path).expect("open file failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error read file to String");
    contents
}

pub fn read_recipe(path: &Path) -> Recipe {
    deserialize_recipe(&read_file(path))
}