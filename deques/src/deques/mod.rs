use std::fmt::Display;


#[cfg(test)]
mod tests;

pub trait Deque: Display {
    type Item;
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn add_first(&mut self, item: Self::Item);
    fn add_last(&mut self, item: Self::Item);
    fn remove_first(&mut self) -> Option<Self::Item>;
    fn remove_last(&mut self) -> Option<Self::Item>;

    fn get_first(&self) -> Option<&Self::Item>;
    fn get_last(&self) -> Option<&Self::Item>;
    fn get_first_mut(&mut self) -> Option<&mut Self::Item>;
    fn get_last_mut(&mut self) -> Option<&mut Self::Item>;
}

pub trait Average {
    fn average(&self) -> Option<f64>;
}
