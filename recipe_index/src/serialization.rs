use inflections::Inflect;
use serde_yaml;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub source: Option<Source>,
    pub links: Option<Vec<String>>,
    pub ingredients: Vec<Ingredient>,
    pub timings: Option<Vec<Timing>>,
    pub notes: Option<Vec<String>>,
    pub instructions: Vec<String>,
    pub times_made: TimesMade,
}

impl Recipe {
    pub fn name(&self) -> String {
        self.name.to_title_case()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timing {
    pub time: String,
    #[serde(rename = "for")]
    pub timing_for: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Seq(Vec<String>),
    Str(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimesMade {
    pub count: u64,
    pub dates: Vec<String>,
}

fn deserialize_recipe(path: &Path, recipe_yaml: &str) -> Recipe {
    serde_yaml::from_str(recipe_yaml).expect(&format!("deserialize recipe {}", path.display()))
}

pub fn read_file(path: &Path) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path).expect("open file failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error read file to String");
    contents
}

pub fn read_recipe(path: &Path) -> Recipe {
    deserialize_recipe(path, &read_file(path))
}
