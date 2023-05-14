use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    pub data: Data,
    pub config: Config,
}

// impl From<Config> for Projector {}
fn default_projector(config: Config) -> Projector {
    Projector {
        data: Data {
            projector: HashMap::new(),
        },
        config: config,
    }
}

fn default_data() -> Data {
    Data {
        projector: HashMap::new(),
    }
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());

        let mut paths = vec![];
        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent()
        }
        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            self.data.projector.get(path).map(|x| out.extend(x.iter()));
        }
        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(val) = dir.get(key) {
                    out = Some(val);
                    break;
                }
            };
            curr = p.parent();
        }
        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_insert_with(|| HashMap::new())
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data
            .projector
            .get_mut(&self.config.pwd)
            .map(|x| x.remove(key));
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config_path).is_ok() {
            let contents = std::fs::read_to_string(&config.config_path);
            let contents = contents.unwrap_or("{\"projector\":{}}".to_string());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Projector { config, data };
        }
        return default_projector(config);
    }
}
trait ValueSetter {}
impl ValueSetter for Projector {}

#[cfg(test)]

mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use super::{Data, Projector};

    #[test]
    fn get_value() {
        let pwd = PathBuf::from("/foo/bar");
        let projector = get_projector(pwd);

        assert_eq!(projector.get_value("key1"), Some(&"value3".to_string()));
        assert_eq!(projector.get_value("key2"), Some(&"topValue".to_string()));
        assert_eq!(projector.get_value("key3"), Some(&"isGreat".to_string()));
    }

    #[test]
    fn set_value() {
        let pwd = PathBuf::from("/foo/bar");
        let mut projector = get_projector(pwd);

        assert_eq!(projector.get_value("key1"), Some(&"value3".to_string()));
        projector.set_value(String::from("key1"), String::from("value4"));
        assert_eq!(projector.get_value("key1"), Some(&"value4".to_string()));

        assert_eq!(projector.get_value("key3"), Some(&"isGreat".to_string()));
        projector.set_value(String::from("key3"), String::from("isGreat2"));
        assert_eq!(projector.get_value("key3"), Some(&"isGreat2".to_string()));
    }

    #[test]
    fn remove_value() {
        let pwd = PathBuf::from("/foo/bar");
        let mut projector = get_projector(pwd);

        assert_eq!(projector.get_value("key1"), Some(&"value3".to_string()));
        projector.remove_value("key1");
        assert_eq!(projector.get_value("key1"), Some(&"value2".to_string()));
    }

    #[test]
    fn get_value_all() {
        let pwd = PathBuf::from("/foo/bar");
        let projector = get_projector(pwd);

        let mut expected = HashMap::new();

        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let key3 = "key3".to_string();

        let value1 = "value3".to_string();
        let value2 = "topValue".to_string();
        let value3 = "isGreat".to_string();

        // Order is important because of how the comparison works
        expected.insert(&key2, &value2);
        expected.insert(&key3, &value3);
        expected.insert(&key1, &value1);

        let get_value = projector.get_value_all();
        assert_eq!(get_value, expected);
    }

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
            PathBuf::from("/")=> hashmap!{
                "key1".into() => "value1".into(),
                "key3".into()=> "isGreat".into(),
            },
            PathBuf::from("/foo") => hashmap!{
                "key1".into()=> "value2".into(),
            },
            PathBuf::from("/foo/bar")=>hashmap!{
                "key1".into()=> "value3".into(),
                "key2".into()=> "topValue".into(),
            },
        };
    }
    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            data: Data {
                projector: get_data(),
            },
            config: crate::config::Config {
                operation: crate::config::Operation::Print(None),
                pwd: pwd,
                config_path: PathBuf::from(""),
            },
        };
    }
}
