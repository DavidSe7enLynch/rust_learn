use std::ops::Add;

pub struct MilliMeter(pub u32);
struct Meter(u32);

impl Add<Meter> for MilliMeter {
    type Output = MilliMeter;
    fn add(self, rhs: Meter) -> Self::Output {
        MilliMeter(self.0 + rhs.0 * 1000)
    }
}

/// MilliMeter + MilliMeter = MilliMeter
/// 
/// # Example
/// ```
/// use library::overload::MilliMeter;
/// 
/// let a = MilliMeter(1);
/// let b = MilliMeter(2);
/// let c = a + b;
/// assert_eq!(c.0, 3);
/// ```
/// 
impl Add for MilliMeter {
    type Output = MilliMeter;
    fn add(self, rhs: Self) -> Self::Output {
        MilliMeter(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let milli1 = MilliMeter(1);
        let milli2 = MilliMeter(2);
        let meter = Meter(1);
        assert_eq!((milli1 + milli2).0, 3);
        assert_eq!((MilliMeter(1) + meter).0, 1001);
    }
}
