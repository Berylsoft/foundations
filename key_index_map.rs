use std::collections::VecDeque;
use crate::*;

pub struct KeyIndexedMap<K1, K2, K3, V> {
    idx: KeyIndex<K1, K2, K3, usize>,
    seq: Vec<(K1, K2, K3, V)>,
}

pub struct QueryIter<'a, K1, K2, K3, V> {
    vi: VecDeque<usize>,
    seq: &'a Vec<(K1, K2, K3, V)>,
}

impl<K1, K2, K3, V> KeyIndexedMap<K1, K2, K3, V>
where
    K1: Ord + Copy,
    K2: Ord + Copy,
    K3: Ord + Copy,
{
    pub fn new() -> KeyIndexedMap<K1, K2, K3, V> {
        KeyIndexedMap {
            idx: KeyIndex::new(),
            seq: Vec::new(),
        }
    }

    pub fn get_idx(&self) -> &KeyIndex<K1, K2, K3, usize> {
        &self.idx
    }

    pub fn get_seq(&self) -> &Vec<(K1, K2, K3, V)> {
        &self.seq
    }

    pub fn from_seq<S: IntoIterator<Item = (K1, K2, K3, V)>>(seq: S) -> Result<KeyIndexedMap<K1, K2, K3, V>, InsertError<K1, K2, K3, usize>> {
        let seq = seq.into_iter().collect();
        let idx = KeyIndex::from_seq(&seq)?;
        Ok(KeyIndexedMap { idx, seq })
    }

    pub fn into_parts(self) -> (KeyIndex<K1, K2, K3, usize>, Vec<(K1, K2, K3, V)>) {
        let KeyIndexedMap { idx, seq } = self;
        (idx, seq)
    }

    pub fn insert(&mut self, k1: K1, k2: K2, k3: K3, v: V) -> Result<(), InsertError<K1, K2, K3, usize>> {
        let vi = self.seq.len();
        self.seq.push((k1, k2, k3, v));
        self.idx.insert(k1, k2, k3, vi)
    }

    pub fn query<'a>(&'a mut self, query: Query<K1, K2, K3>) -> QueryIter<'a, K1, K2, K3, V> {
        QueryIter {
            vi: self.idx.query(query).into_iter().collect(),
            seq: &self.seq,
        }
    }

    pub fn get(&self, k1: K1, k2: K2, k3: K3) -> Option<&(K1, K2, K3, V)> {
        let query: Vec<_> = self.idx.query(Query { k1: Some(k1), k2: Some(k2), k3: Some(k3) }).into_iter().collect();
        if query.len() == 1 {
            let [res]: [_; 1] = query.try_into().unwrap();
            let res = self.seq.get(res).unwrap();
            Some(res)
        } else if query.is_empty() {
            None
        } else {
            unreachable!()
        }
    }
}

impl<'a, K1, K2, K3, V> Iterator for QueryIter<'a, K1, K2, K3, V> {
    type Item = &'a (K1, K2, K3, V);

    fn next(&mut self) -> Option<Self::Item> {
        let vi = self.vi.pop_front()?;
        Some(self.seq.get(vi).unwrap())
    }
}
