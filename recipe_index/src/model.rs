
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub path: String,
}
