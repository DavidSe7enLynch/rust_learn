use std::cmp::{max, Ord};
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::marker::Copy;

pub mod traits;

pub struct Point<T>
where
    T: Ord + Hash + Add<Output = T> + Copy,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Ord + Hash + Add<Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    pub fn x(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn larger(&self) -> T {
        *max(&self.x, &self.y)
    }
}

impl<T> Hash for Point<T>
where
    T: Ord + Hash + Add<Output = T> + Copy,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> traits::Norm1d<T> for Point<T> 
where T: Ord + Hash + Add<Output = T> + Copy
{
    fn len(&self) -> T {
        self.x + self.y
    }
}