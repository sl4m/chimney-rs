use std::collections::{HashMap, VecDeque};
use std::fs;

use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};

use crate::config_store::ClientConfig;

pub const GLOBAL: &str = "global";
type MachineId = String;

#[derive(Debug)]
pub struct ConfigStore {
    pub cache: HashMap<MachineId, ClientConfig>,
    pub path: Utf8PathBuf,
}

fn find_config_files<P: AsRef<Utf8Path>>(
    path: P,
    config_map: &mut HashMap<MachineId, ClientConfig>,
) -> Result<(), anyhow::Error> {
    let mut queue: VecDeque<Utf8PathBuf> = VecDeque::new();
    queue.push_back(path.as_ref().to_path_buf());
    let mut errors: Vec<String> = vec![];

    while let Some(dir_path) = queue.pop_front() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let entry_path = Utf8PathBuf::from_path_buf(entry.path()).expect("valid UTF-8 path");
            if entry_path.is_dir() {
                queue.push_back(entry_path);
            } else {
                match ClientConfig::from_file(entry_path) {
                    Ok(config) => {
                        config_map
                            .entry(config.machine_id.clone())
                            .or_insert(config);
                    }
                    Err(e) => errors.push(e.to_string()),
                }
            }
        }
    }
    if !errors.is_empty() {
        Err(anyhow!("{}", errors.join(", ")))
    } else {
        Ok(())
    }
}

impl ConfigStore {
    pub fn from_path<P: AsRef<Utf8Path>>(path: P) -> Result<Self, anyhow::Error> {
        let path_ref = path.as_ref();
        if !path_ref.is_dir() {
            return Err(anyhow!("path {:?} is not a directory", path_ref));
        }
        let mut cache = HashMap::new();
        find_config_files(path_ref, &mut cache)?;
        if !cache.contains_key(GLOBAL) {
            return Err(anyhow!(
                "path {:?} does not contain {}.toml",
                path_ref,
                GLOBAL
            ));
        }

        Ok(ConfigStore {
            cache,
            path: path_ref.to_path_buf(),
        })
    }

    pub fn config_for(&self, machine_id: &str) -> ClientConfig {
        self.cache
            .get(machine_id)
            .or_else(|| self.cache.get(GLOBAL))
            .cloned()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errs_if_path_is_non_dir() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls/good/global.toml");
        assert!(store.is_err());
    }

    #[test]
    fn caches_config_files_from_path() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls/good");
        assert!(store.is_ok());
        assert_eq!(3, store.unwrap().cache.len());
    }

    #[test]
    fn errs_if_path_contains_bad_config_files() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls");
        assert!(store.is_err());
    }

    #[test]
    fn errs_if_path_does_not_contain_global() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls/bad");
        assert!(store.is_err());
    }

    #[test]
    fn returns_machine_specific_config() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls/good");
        let store = store.unwrap();
        let config = store.config_for("machine-id-1234");
        assert_eq!(8, config.rules.len());
    }

    #[test]
    fn falls_back_to_global_config() {
        let store = ConfigStore::from_path("tests/tomls/client-tomls/good");
        let store = store.unwrap();
        let config = store.config_for("nonexistent-machine-id");
        assert_eq!(GLOBAL, config.machine_id);
        assert_eq!(0, config.rules.len());
    }
}
