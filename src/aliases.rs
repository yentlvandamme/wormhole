use std::env;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use crate::file_manager::FileManager;

// Look into: https://stackoverflow.com/questions/66801681/is-there-any-use-for-str-in-rust
// https://stackoverflow.com/questions/54488127/getting-temporary-value-dropped-while-borrowed-when-trying-to-update-an-option
pub struct Aliases {
    aliases: HashMap<AliasName, PathBuf>,
    file_manager: FileManager
}

pub type AliasName = String;

impl Aliases {
    pub fn init () -> Self {
        let file_manager = FileManager::new(Self::get_aliases_data_location());
        let alias_file_contents = file_manager.get_contents();

        match serde_json::from_str(&alias_file_contents) {
            Ok(parsed_aliases) => Self { aliases: parsed_aliases, file_manager },
            Err(_) => Self { aliases: HashMap::new(), file_manager }
        }
    }

    pub fn add (&mut self, alias_name: AliasName, path: Option<String>) {
        if self.aliases.contains_key(&alias_name) {
            println!("Cannot add new alias. Key already exists");
            return
        }

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
        self.file_manager.write_content(self.serialize_aliases());
    }

    pub fn remove (&mut self, alias_name: AliasName) {
        let removed_value = self.aliases.remove_entry(&alias_name);

        match removed_value {
            Some((key, value)) => {
                println!("The following alias has been removed:");
                self.print_alias(key.as_str(), value.as_os_str().to_str().unwrap());
            },
            None => println!("Alias name not found.")
        };

        self.file_manager.write_content(self.serialize_aliases());
    }

    pub fn all_aliases (&self) {
        let all_keys = self.aliases.keys();

        for key in all_keys {
            if let Some(value) = self.aliases.get(key) {
                self.print_alias(key.as_str(), value.as_os_str().to_str().unwrap());
            }
        };
    }

    pub fn navigate_to_alias(&self,  alias_name: AliasName) {
        match self.aliases.get(&alias_name) {
            Some(path) => {
                println!("cd {}", path.to_string_lossy());
                // https://www.reddit.com/r/rust/comments/iqw2zo/how_to_implement_shell_built_in_commands_in_rust/
                // We cannot use set_current_dir, since changing the current location would mean
                // changing the location of the parent process. (The processing calling our
                // program)
                //
                // 1) A workaround might be to copy the command to the user's clipboard
                // 2) Maybe we can ask the OS to open a new terminal with the requested path,
                //    providing the user passes a flag
            },
            None => println!("Alias not found")
        }
    }

    fn serialize_aliases(&self) -> String {
        serde_json::to_string_pretty(&self.aliases).unwrap()
    }

    fn print_alias<T: Display, K: Display>(&self, key: T, value: K) {
         println!("{0: <20} {1: <20}", key, value);
    }

    fn get_aliases_data_location() -> PathBuf {
        let mut path = PathBuf::new();
        let directory = env!("OUT_DIR");
        path.push(&directory);
        path.push("aliases.json");

        path
    }
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
    fn add_duplicated_alias() {
        let mut alias_instance = Aliases::init();
        let alias_name = String::from("TestName");
        let first_alias_path = String::from("/bin");
        let second_alias_path = String::from("/etc");
        let mut expected_hashmap_result = HashMap::new();

        alias_instance.add(alias_name.clone(), Some(first_alias_path.clone()));
        alias_instance.add(alias_name.clone(), Some(second_alias_path.clone()));
        expected_hashmap_result.insert(alias_name.clone(), PathBuf::from(first_alias_path.clone()));

        assert_eq!(expected_hashmap_result, alias_instance.aliases);
    }

    #[test]
    fn add_duplicated_path() {
        let mut alias_instance = Aliases::init();
        let first_alias_name = String::from("TestName");
        let second_alias_name = String::from("AnotherTestName");
        let alias_path = String::from("/bin");
        let mut expected_hashmap_result = HashMap::new();

        alias_instance.add(first_alias_name.clone(), Some(alias_path.clone()));
        alias_instance.add(second_alias_name.clone(), Some(alias_path.clone()));
        expected_hashmap_result.insert(first_alias_name.clone(), PathBuf::from(alias_path.clone()));
        expected_hashmap_result.insert(second_alias_name.clone(), PathBuf::from(alias_path.clone()));

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
