use std::{collections::HashMap, path::PathBuf, env};

pub struct Aliases {
    aliases: HashMap<String, PathBuf>
}

impl Aliases {
    pub fn init () -> Self {
        Self {aliases: HashMap::new()}
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
    }

    pub fn all_aliases (&self) {
        let all_keys = self.aliases.keys();

        for key in all_keys {
            println!("{:?}", key);
        }
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
}
