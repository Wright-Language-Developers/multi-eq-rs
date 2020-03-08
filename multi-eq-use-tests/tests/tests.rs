use multi_eq::*;
use multi_eq_derive_tests::*;

multi_eq_make_trait!(TestEq, test_eq);

#[test]
fn test_basic_enum() {
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

#[test]
fn test_unit() {
    #[derive(TestEq)]
    struct TestUnit;

    assert!(TestUnit.test_eq(&TestUnit));
}

#[test]
fn test_struct_attr_cmp() {
    #[derive(TestEq)]
    struct TestStruct {
	a: u32,
	b: bool,
	c: (),
    }
}
