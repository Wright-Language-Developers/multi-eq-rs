#[test]
fn it_works() {
    use multi_eq_derive_tests::*;

    #[derive(TestEq)]
    enum TestEnum {
        A,
        B,
        C,
    }

    assert!(TestEnum::A.test_eq(TestEnum::A));
}
