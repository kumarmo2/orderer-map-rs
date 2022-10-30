use std::{borrow::Borrow, collections::HashMap, hash::Hash, ops::Deref};

/*
* - the actions that mutate the map, are the potential clients for implementing those methods.
* - for actions that only require getting the values in map, i am thinking to implement the deref
*   trait and use the actual map's method. So the consumer of the OrderedMap, will need to call the
*   deref method and then use those methods. This is likely to change in next phases.
*
* - Mutating Actions
*   1. Insert Entry
*       - If same key is inserted multiple times, do we update the order of the Key ?
*           - For simplicity sake, lets not update the order if the key is re-inserted.
*   2. Delete Entry
*       - once an entry is deleted from the map and if we don't delete the entry from the
*         datastructure maintaing the order,  when we will iterate on the orderedMap,
*         will need to check if the key exists in the under lying map or not.
* */

pub struct OrderedMap<K, V>
where
    K: Eq + Hash + Clone,
{
    // I am keeping Clone trait for now, because If I don't keep the clone trait, and the `inner`
    // map  is the owner of tke key, then ordered_keys will need to be a reference to the `inner`,
    // which would make this OrderedMap a self-referential struct, which is not possible as of
    // today in safe rust.
    inner: HashMap<K, V>,
    ordered_keys: Vec<K>,
}

pub struct OrderedMapIter<'a, K, V>
where
    K: Eq + Hash + Clone,
{
    map: &'a OrderedMap<K, V>,
    remainder_keys: &'a [K],
}

impl<'a, K, V> Iterator for OrderedMapIter<'a, K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder_keys.is_empty() {
            return None;
        }

        while let Some(key) = self.remainder_keys.first() {
            self.remainder_keys = &self.remainder_keys[1..self.remainder_keys.len()];
            if self.map.deref().contains_key(key) {
                return Some((key, self.map.deref().get(key).unwrap()));
            }
        }
        None
    }
}
impl<K, V> Deref for OrderedMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, K, V> OrderedMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        OrderedMap {
            inner: HashMap::new(),
            ordered_keys: Vec::new(),
        }
    }
    pub fn iter(&'a self) -> OrderedMapIter<'a, K, V> {
        OrderedMapIter {
            map: self,
            remainder_keys: &self.ordered_keys[..],
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if !self.inner.contains_key(&key) {
            self.ordered_keys.push(key.to_owned());
        }
        return self.inner.insert(key.to_owned(), value);
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.remove(key)
    }
}

#[test]
fn it_works() {
    let map: OrderedMap<String, i32> = OrderedMap::new();
    assert_eq!(map.iter().next(), None);
}
#[test]
fn it_works_deletion_with_no_element() {
    let mut map: OrderedMap<String, i32> = OrderedMap::new();
    map.remove("somename");
    assert_eq!(map.iter().next(), None);
}

#[test]
fn it_works_for_one_element() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(iterator.next(), None);
}
#[test]
fn it_works_for_one_element_with_int_key() {
    let mut map = OrderedMap::new();
    map.insert(1, 4);
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(iterator.next(), None);
}

#[test]
fn it_works_for_two_element() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("kumarmo2", 5);
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(*(iterator.next().unwrap().1), 5);
    assert_eq!(iterator.next(), None);
}

#[test]
fn simple_deletion() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("kumarmo2", 5);
    map.remove("name");
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 5);
    assert_eq!(iterator.next(), None);
}

#[test]
fn deletion_at_end() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("name2", 10);
    map.insert("kumarmo2", 5);
    map.remove("kumarmo2");
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(*(iterator.next().unwrap().1), 10);
    assert_eq!(iterator.next(), None);
}

#[test]
fn deletion_at_middle() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("name2", 10);
    map.insert("kumarmo2", 5);
    map.remove("name2");
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(*(iterator.next().unwrap().1), 5);
    assert_eq!(iterator.next(), None);
}

#[test]
fn it_works_for_updating_key() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("name2", 3);
    map.insert("kumarmo2", 5);
    map.insert("name2", 10);
    let mut iterator = map.iter();
    assert_eq!(*(iterator.next().unwrap().1), 4);
    assert_eq!(*(iterator.next().unwrap().1), 10);
    assert_eq!(*(iterator.next().unwrap().1), 5);
    assert_eq!(iterator.next(), None);
}

#[test]
fn simple_iteration_works() {
    let mut map = OrderedMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.insert(4, 4);
    let mut iterator = map.iter();
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (1, 1));
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (2, 2));
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (3, 3));
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (4, 4));
}
#[test]
fn iteration_works_with_deletion() {
    let mut map = OrderedMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.insert(4, 4);
    map.remove(&2);
    map.remove(&3);
    let mut iterator = map.iter();
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (1, 1));
    // assert_eq!((*tuple.0, *tuple.1), (2, 2));
    // assert_eq!((*tuple.0, *tuple.1), (3, 3));
    let tuple = iterator.next().unwrap();
    assert_eq!((*tuple.0, *tuple.1), (4, 4));
}
