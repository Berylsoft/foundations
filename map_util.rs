use std::collections::{BTreeMap, btree_map::Entry};

pub fn get_or_insert<K: Ord, V, F: FnOnce() -> V>(map: &mut BTreeMap<K, V>, k: K, init_v: F) -> &mut V {
    match map.entry(k) {
        Entry::Vacant(entry) => entry.insert(init_v()),
        Entry::Occupied(entry) => entry.into_mut(),
    }
}

pub fn first_insert<K: Ord, V>(map: &mut BTreeMap<K, V>, k: K, v: V) -> &mut V {
    match map.entry(k) {
        Entry::Vacant(entry) => entry.insert(v),
        Entry::Occupied(_) => unreachable!(),
    }
}
