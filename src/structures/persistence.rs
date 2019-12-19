use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use rayon::prelude::*;
use serde::Serialize;

use crate::structures::MapKey;

pub trait PersistData<V> {
    fn persist(&self, prefix: &str, data: &HashMap<MapKey, HashSet<V>>);
}

pub struct PersistDataToDisk {}

impl PersistDataToDisk {
    pub fn open_file(&self, k: &MapKey, prefix: &str) -> File {
        let path = Path::new(prefix).join(format!("{}.json", k));
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .expect("failed to open file")
    }
}

impl<V> PersistData<V> for PersistDataToDisk
    where V: Serialize + Send + Sync {
    fn persist(&self, prefix: &str, data: &HashMap<MapKey, HashSet<V>>) {
        fs::create_dir_all(prefix).expect("Failed to create data directory");
        data.par_iter()
            .for_each(|(k, v): (&MapKey, &HashSet<V>)| {
                let mut file = self.open_file(k, prefix);
                for entry in v {
                    match serde_json::to_string(&entry) {
                        Ok(json_value) => writeln!(file, "{}", json_value).expect("Failed to write to file"),
                        Err(err) => eprintln!("Failed to serialize entry {:?}", err),
                    }
                }
            });
    }
}


