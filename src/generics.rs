use std::cmp::{max, Ord};
use std::hash::{Hash, Hasher};
use std::marker::Copy;
use std::ops::Add;

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

    pub fn panic(&self) {
        panic!("intentionally panic for test");
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
where
    T: Ord + Hash + Add<Output = T> + Copy,
{
    fn len(&self) -> T {
        self.x + self.y
    }
}

#[cfg(test)]
mod test_point {
    use super::*;

    #[test]
    fn test_larger() {
        let x = 13;
        let y = 9;
        let point1 = Point::new(x, y);
        assert_eq!(point1.larger(), x, "should return larger");

        let point2 = Point::new(y, x);
        assert_eq!(point2.larger(), x, "should return larger");
    }

    #[test]
    #[should_panic(expected = "intentionally")]
    fn test_panic() {
        let x = 13;
        let y = 9;
        let point1 = Point::new(x, y);
        point1.panic();
    }
}
