use std::{collections::HashMap, ops::Deref};

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

pub struct OrderedMap {
    inner: HashMap<String, i32>,
    ordered_keys: Vec<String>,
}

pub struct OrderedMapIter<'a> {
    map: &'a OrderedMap,
    remainder_keys: &'a [String],
}

impl<'a> Iterator for OrderedMapIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder_keys.is_empty() {
            return None;
        }

        while let Some(key) = self.remainder_keys.first() {
            self.remainder_keys = &self.remainder_keys[1..self.remainder_keys.len()];
            if self.map.deref().contains_key(key) {
                return self.map.deref().get(key);
            }
        }
        None
    }
}
impl Deref for OrderedMap {
    type Target = HashMap<String, i32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl OrderedMap {
    pub fn new() -> Self {
        OrderedMap {
            inner: HashMap::new(),
            ordered_keys: Vec::new(),
        }
    }
    pub fn iter(&self) -> OrderedMapIter {
        OrderedMapIter {
            map: self,
            remainder_keys: &self.ordered_keys[..],
        }
    }

    pub fn insert(&mut self, key: &str, value: i32) -> Option<i32> {
        if !self.inner.contains_key(key) {
            self.ordered_keys.push(key.to_owned());
        }
        return self.inner.insert(key.to_owned(), value);
    }

    pub fn remove(&mut self, key: &str) -> Option<i32> {
        self.inner.remove(key)
    }
}

#[test]
fn it_works() {
    let map = OrderedMap::new();
    assert_eq!(map.iter().next(), None);
}
#[test]
fn it_works_deletion_with_no_element() {
    let mut map = OrderedMap::new();
    map.remove("somename");
    assert_eq!(map.iter().next(), None);
}

#[test]
fn it_works_for_one_element() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    let mut iterator = map.iter();
    assert_eq!(*iterator.next().unwrap(), 4);
    assert_eq!(iterator.next(), None);
}

#[test]
fn it_works_for_two_element() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("kumarmo2", 5);
    let mut iterator = map.iter();
    assert_eq!(*iterator.next().unwrap(), 4);
    assert_eq!(*iterator.next().unwrap(), 5);
    assert_eq!(iterator.next(), None);
}

#[test]
fn simple_deletion() {
    let mut map = OrderedMap::new();
    map.insert("name", 4);
    map.insert("kumarmo2", 5);
    map.remove("name");
    let mut iterator = map.iter();
    assert_eq!(*iterator.next().unwrap(), 5);
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
    assert_eq!(*iterator.next().unwrap(), 4);
    assert_eq!(*iterator.next().unwrap(), 10);
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
    assert_eq!(*iterator.next().unwrap(), 4);
    assert_eq!(*iterator.next().unwrap(), 5);
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
    assert_eq!(*iterator.next().unwrap(), 4);
    assert_eq!(*iterator.next().unwrap(), 10);
    assert_eq!(*iterator.next().unwrap(), 5);
    assert_eq!(iterator.next(), None);
}
