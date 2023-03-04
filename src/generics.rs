use std::cmp::{max, Ord};
use std::hash::{Hash, Hasher};

pub struct Point<T>
where
    T: Ord + Hash,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Ord + Hash,
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

impl<T: Hash> Hash for Point<T>
where
    T: Ord + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
