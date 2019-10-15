use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub struct Store {
    pub base_path: String,
}

impl Store {
    pub fn new(base_path: &str) -> Result<Store, &'static str> {
        let store = Store {
            base_path: String::from(base_path),
        };
        Ok(store)
    }

    pub fn get(&self, key: &str) -> std::io::Result<String> {
        let path = Path::new(&self.base_path).join(key);
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn set(&self, key: &str, value: &str) -> std::io::Result<()> {
        let path = Path::new(&self.base_path).join(key);

        let folder = path.parent().unwrap();
        fs::create_dir_all(folder)?;

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(&path)?;
        file.write_all(value.as_bytes())?;

        Ok(())
    }

    pub fn list(&self) -> Vec<String> {
        fn is_hidden(entry: &DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        let entries: Vec<String> = WalkDir::new(&self.base_path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(|e| {
                let path = e.path();
                let stripped = path.strip_prefix(&self.base_path).unwrap();
                String::from(stripped.to_str().unwrap())
            })
            .collect();

        return entries;
    }
}
