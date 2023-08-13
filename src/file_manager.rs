use std::path::PathBuf;
use std::io::{BufReader, Read, Write};
use std::fs::File;

pub struct FileManager {
    path: PathBuf
}

impl FileManager {
    pub fn new(file_path: PathBuf) -> Self {
        Self { path: file_path }
    }

    pub fn get_contents(&self) -> String {
        let file = File::open(&self.path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        contents
    }

    // TODO: Should we eventually switch to a Write trait implementation?
    pub fn write_content(&self, content: String) {
        let mut file = File::create(&self.path).unwrap();
        file.write(content.as_bytes()).unwrap();
    }
}
