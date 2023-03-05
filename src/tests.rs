

#[cfg(test)]
mod test_point {
    use crate::generics;

    #[test]
    fn test_larger() {
        let x = 13;
        let y = 9;
        let point1 = generics::Point::new(x, y);
        assert_eq!(point1.larger(), x);
        
        let point2 = generics::Point::new(y, x);
        assert_eq!(point2.larger(), x);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let x = 13;
        let y = 9;
        let point1 = generics::Point::new(x, y);
        point1.panic();
    }
}