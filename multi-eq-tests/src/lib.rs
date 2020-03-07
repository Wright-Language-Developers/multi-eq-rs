use multi_eq::*;

multi_eq_make_trait!(TestEq, test_eq);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        #[derive(TestEq)]
        enum TestEnum {
            A,
            B,
            C,
        }

        assert!(TestEnum::A.test_eq(TestEnum::A));
    }
}
