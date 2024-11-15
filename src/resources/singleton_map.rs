use std::fmt::Debug;
use anymap::AnyMap;

// Wrapper type
#[derive(Default, Debug)]
pub struct SingletonMap {
    map: AnyMap
}

impl SingletonMap {
    pub fn insert(&mut self, value: impl Debug + Clone + 'static) {
        self.map.insert(value);
    }
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map.remove::<T>()
    }
    pub fn get<T: Clone + Debug + 'static>(&self) -> Option<T> {
        self.map.get::<T>().cloned()
    }
}