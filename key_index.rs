use std::collections::{BTreeMap, BTreeSet};

pub struct KeyIndex<K1, K2, K3, VI> {
    idx1: BTreeMap<K1, BTreeSet<VI>>,
    idx2: BTreeMap<K2, BTreeSet<VI>>,
    idx3: BTreeMap<K3, BTreeSet<VI>>,
}

pub struct InsertError<K1, K2, K3, VI> {
    pub ty: InsertErrorType,
    pub val: (K1, K2, K3, VI),
}

pub enum InsertErrorType {
    K1,
    K2,
    K3,
}

pub struct Query<K1, K2, K3> {
    pub k1: Option<K1>,
    pub k2: Option<K2>,
    pub k3: Option<K3>,
}

impl<K1, K2, K3, VI> KeyIndex<K1, K2, K3, VI>
where
    K1: Ord + Copy,
    K2: Ord + Copy,
    K3: Ord + Copy,
    VI: Ord + Copy,
{
    pub fn new() -> KeyIndex<K1, K2, K3, VI> {
        KeyIndex {
            idx1: BTreeMap::new(),
            idx2: BTreeMap::new(),
            idx3: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k1: K1, k2: K2, k3: K3, vi: VI) -> Result<(), InsertError<K1, K2, K3, VI>> {
        macro_rules! kn_impl {
            ($($kn:ident, $idxn:ident, $Kn:ident;)*) => {
                $(if !self.$idxn.entry($kn).or_default().insert(vi) {
                    return Err(InsertError {
                        ty: InsertErrorType::$Kn,
                        val: (k1, k2, k3, vi),
                    });
                })*
            };
        }
        kn_impl!(k1, idx1, K1; k2, idx2, K2; k3, idx3, K3;);
        Ok(())
    }

    pub fn query(&self, query: Query<K1, K2, K3>) -> BTreeSet<VI> {
        let Query { k1, k2, k3 } = query;
        let mut res = BTreeSet::new();
        macro_rules! kn_impl {
            ($($kn:ident, $idxn:ident;)*) => {
                $(if let Some(k) = $kn {
                    if let Some(s) = self.$idxn.get(&k) {
                        for vi in s {
                            assert!(res.insert(*vi));
                        }
                    }
                })*
            };
        }
        kn_impl!(k1, idx1; k2, idx2; k3, idx3;);
        res
    }
}

impl<K1, K2, K3> KeyIndex<K1, K2, K3, usize>
where
    K1: Ord + Copy,
    K2: Ord + Copy,
    K3: Ord + Copy,
{
    pub fn from_seq<V, S: AsRef<[(K1, K2, K3, V)]>>(seq: S) -> Result<KeyIndex<K1, K2, K3, usize>, InsertError<K1, K2, K3, usize>> {
        let mut res = KeyIndex::new();
        for (vi, (k1, k2, k3, _)) in seq.as_ref().iter().enumerate() {
            res.insert(*k1, *k2, *k3, vi)?;
        }
        Ok(res)
    }
}

#[cfg(feature = "map")]
mod map;
#[cfg(feature = "map")]
pub use map::*;
