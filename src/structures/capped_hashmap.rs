use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use serde::Serialize;

use crate::structures::MapKey;
use crate::structures::PersistData;

pub struct CappedDumpableHashMap<V> {
    max_size: usize,
    prefix: String,
    data: HashMap<MapKey, HashSet<V>>,
    counter: usize,
    persistence: Box<dyn PersistData<V>>,
}

impl<V> CappedDumpableHashMap<V>
    where V: Eq + Hash + Serialize {
    pub fn new(
        max_size: usize,
        prefix: &str,
        persistence: Box<dyn PersistData<V>>,
    ) -> Self {
        CappedDumpableHashMap {
            max_size,
            prefix: prefix.to_owned(),
            data: HashMap::new(),
            counter: 0,
            persistence,
        }
    }

    fn cumulative_size(&self) -> usize {
        self.data
            .iter()
            .map(|(_k, v)| v.len())
            .sum()
    }

    pub fn add(&mut self, key: MapKey, value: V) {
        self.data
            .entry(key)
            .or_insert(HashSet::new())
            .insert(value);

        self.counter += 1;
        if self.counter >= self.max_size {
            println!("dumping {} entries to disk", self.cumulative_size());
            self.dump_to_disk();
            self.data.clear();
            self.counter = 0;
            println!("dump finished");
        }
    }

    pub fn dump_to_disk(&mut self) {
        self.persistence.persist(&self.prefix, &self.data);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use serde::de::Unexpected::Map;

    use super::*;

    struct Dummy<V> {
        dirty: Rc<Cell<bool>>,
        data: Rc<Cell<Option<HashMap<MapKey, HashSet<V>>>>>,
    }

    impl<V> PersistData<V> for Dummy<V>
        where V: Clone {
        fn persist(&self, _prefix: &str, data: &HashMap<MapKey, HashSet<V>>) {
            self.dirty.set(true);
            self.data.set(Some(data.clone()));
        }
    }


    #[test]
    fn capped_hashmap_dumps_calls_persist_when_full() {
        let dirty = Rc::new(Cell::new(false));
        let data = Rc::new(Cell::new(None));
        let dummy: Dummy<String> = Dummy { dirty: dirty.clone(), data: Rc::clone(&data) };

        let mut capped_map: CappedDumpableHashMap<String> = CappedDumpableHashMap::new(
            3,
            "",
            Box::new(dummy),
        );

        capped_map.add(MapKey::NumKey { key: 1 }, "".to_owned());
        capped_map.add(MapKey::NumKey { key: 1 }, "".to_owned());
        assert!(!dirty.get());
        capped_map.add(MapKey::NumKey { key: 1 }, "".to_owned());
        assert!(dirty.get());
    }

    #[test]
    fn data_is_split_by_key() {
        let data = Rc::new(Cell::new(None));
        let dummy: Dummy<String> = Dummy {
            dirty: Rc::new(Cell::new(false)),
            data: Rc::clone(&data),
        };

        let mut capped_map: CappedDumpableHashMap<String> = CappedDumpableHashMap::new(
            3,
            "",
            Box::new(dummy),
        );

        capped_map.add(MapKey::NumKey { key: 1 }, "".to_owned());
        capped_map.add(MapKey::NumKey { key: 2 }, "".to_owned());
        capped_map.add(MapKey::NumKey { key: 2 }, "".to_owned());

        let data = data.take().unwrap();
        assert_eq!(data.len(), 2);

        let key = MapKey::NumKey { key: 2 };
        assert_eq!(data.get(&key).unwrap().len(), 1);
    }
}