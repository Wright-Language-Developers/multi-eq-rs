#[test]
fn it_works() {
    use multi_eq::*;
    use multi_eq_derive_tests::*;

    multi_eq_make_trait!(TestEq, test_eq);

    #[derive(TestEq)]
    enum TestEnum {
        A,
        B,
        C,
    }

    assert!(TestEnum::A.test_eq(&TestEnum::A));
    assert!(TestEnum::B.test_eq(&TestEnum::B));
    assert!(TestEnum::C.test_eq(&TestEnum::C));
    assert!(!TestEnum::A.test_eq(&TestEnum::B));
    assert!(!TestEnum::B.test_eq(&TestEnum::A));
    assert!(!TestEnum::C.test_eq(&TestEnum::A));
}
