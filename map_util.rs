use std::collections::{BTreeMap, btree_map::Entry};

// remove when map_try_insert stable
pub fn first_insert<K: Ord, V>(map: &mut BTreeMap<K, V>, k: K, v: V) -> &mut V {
    match map.entry(k) {
        Entry::Vacant(entry) => entry.insert(v),
        Entry::Occupied(_) => unreachable!(),
    }
}
