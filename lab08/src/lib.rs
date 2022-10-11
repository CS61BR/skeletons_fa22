pub mod chashmap;
pub mod myhashmap;
pub mod ohashmap;
mod std_types;
#[cfg(test)]
mod tests;
pub mod ullmap;

pub trait Map61B: IntoIterator<Item = (Self::Key, Self::Value)> {
    type Key;
    type Value;

    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn clear(&mut self);
    fn contains_key(&self, key: &Self::Key) -> bool;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;
}
