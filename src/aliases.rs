use std::env;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;
use crate::file_manager::FileManager;

pub struct Aliases {
    aliases: HashMap<String, PathBuf>
}

impl Aliases {
    pub fn init () -> Self {
        let alias_data_path = get_alias_data_location();

        let file_manager = FileManager::new(alias_data_path);
        let alias_file_contents = file_manager.get_contents();

        match serde_json::from_str(&alias_file_contents) {
            Ok(parsed_aliases) => Self { aliases: parsed_aliases },
            Err(_) => Self { aliases: HashMap::new() }
        }
    }

    pub fn add (&mut self, alias_name: String, path: Option<String>) {
        let alias_path = match path {
            Some(path) => PathBuf::from(path),
            None => {
                match env::current_dir() {
                    Ok(location) => location,
                    Err(e) => {
                        eprintln!("{}", e);
                        return
                    }
                }
            },
        };

        self.aliases.insert(alias_name, alias_path);
        self.write_updates(get_alias_data_location());
    }

    pub fn remove (&mut self, alias_name: String) {
        let removed_value = self.aliases.remove_entry(&alias_name);
        match removed_value {
            Some((key, value)) => println!("{} {}", key, value.into_os_string().into_string().unwrap()),
            None => println!("Alias name not found.")
        };

        self.write_updates(get_alias_data_location());
    }

    // TODO: Show a clean output to the user
    pub fn all_aliases (&self) {
        let all_keys = self.aliases.keys();

        for key in all_keys {
            println!("{:?}", key);
        };
    }

    fn write_updates(&self, location: PathBuf) {
        let mut file = File::create(&location).unwrap();
        file.write(serde_json::to_string_pretty(&self.aliases).unwrap().as_bytes()).unwrap();
    }

}

// TODO: This probably isn't the correct location for this helper function
// New name: get_location
fn get_alias_data_location() -> PathBuf {
    let mut path = PathBuf::new();
    let directory = env!("OUT_DIR");
    path.push(&directory);
    path.push("aliases.json");

    path
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_valid_path() {
        let mut alias_instance = Aliases::init();
        let alias_name = String::from("TestName");
        let alias_path = String::from("/bin");
        let mut expected_hashmap_result = HashMap::new();

        alias_instance.add(alias_name.clone(), Some(alias_path.clone()));
        expected_hashmap_result.insert(alias_name.clone(), PathBuf::from(alias_path.clone()));

        assert_eq!(expected_hashmap_result, alias_instance.aliases);
    }

    #[test]
    fn add_alias_without_path() {
        let mut alias_instance = Aliases::init();
        let alias_name = String::from("TestName");
        let current_dir = env::current_dir();
        let mut expected_hashmap_result = HashMap::new();

        alias_instance.add(alias_name.clone(), None);
        expected_hashmap_result.insert(alias_name.clone(), current_dir.unwrap());

        assert_eq!(expected_hashmap_result, alias_instance.aliases);
    }

    #[test]
    fn remove_alias() {
        let mut alias_instance = Aliases::init();
        let alias_name = String::from("TestName");

        alias_instance.add(alias_name.clone(), None);
        alias_instance.remove(alias_name.clone());

        assert!(alias_instance.aliases.is_empty());
    }
}
