

#[cfg(test)]
mod test_point {
    use crate::generics;

    #[test]
    fn test_larger() {
        let x = 13;
        let y = 9;
        let point1 = generics::Point::new(x, y);
        assert_eq!(point1.larger(), x, "should return larger");
        
        let point2 = generics::Point::new(y, x);
        assert_eq!(point2.larger(), x, "should return larger");
    }

    #[test]
    #[should_panic(expected = "intentionally")]
    fn test_panic() {
        let x = 13;
        let y = 9;
        let point1 = generics::Point::new(x, y);
        point1.panic();
    }
}