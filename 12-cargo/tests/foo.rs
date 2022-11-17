#[cfg(test)]
mod foo_tests {

    #[test]
    fn test_a() {
        fn add<T>(x: T, y: T) -> T
        where
            T: std::ops::Add<Output = T>,
        {
            x + y
        }

        assert_eq!(add(2, 3), 5);
    }
}
