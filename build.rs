use std::env;
use std::path::Path;
use std::fs::File;

fn main () {
    // Create a JSON-file which will be used to store all the aliases
    let file_name = "aliases.json";
    let target_directory = env::var("OUT_DIR").unwrap();
    let destination_path = Path::new(&target_directory).join(&file_name);
    File::create(&destination_path).unwrap();
}
