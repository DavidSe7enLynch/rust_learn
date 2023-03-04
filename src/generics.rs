use std::cmp::{max, Ord};
use std::hash::{Hash, Hasher};
use std::ops::Add;

mod traits;

pub struct Point<T>
where
    T: Ord + Hash + Add,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Ord + Hash + Add,
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

    pub fn larger(&self) -> &T {
        max(&self.x, &self.y)
    }
}

impl<T> Hash for Point<T>
where
    T: Ord + Hash + Add,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> traits::Norm2d<T> for Point<T> 
where T: Ord + Hash + Add
{
    fn len(&self) -> T {
        self.x + self.y
    }
}