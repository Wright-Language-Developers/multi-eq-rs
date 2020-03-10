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
fn test_tuple_enum() {
    #[derive(TestEq)]
    enum TestEnum {
        A(
            #[test_eq(cmp = "eq")] u8,
            #[test_eq(cmp = "eq")] i8,
            #[test_eq(cmp = "eq")] bool,
        ),
        B(
            #[test_eq(cmp = "eq")] u16,
            #[test_eq(cmp = "eq")] i16,
            #[test_eq(cmp = "eq")] (),
        ),
    }

    assert!(TestEnum::A(0, 0, false).test_eq(&TestEnum::A(0, 0, false)));
    assert!(TestEnum::A(255, 100, true).test_eq(&TestEnum::A(255, 100, true)));
    assert!(TestEnum::B(0, 0, ()).test_eq(&TestEnum::B(0, 0, ())));
    assert!(TestEnum::B(255, 100, ()).test_eq(&TestEnum::B(255, 100, ())));

    assert!(!TestEnum::A(0, 0, false).test_eq(&TestEnum::B(0, 0, ())));
    assert!(!TestEnum::B(0, 0, ()).test_eq(&TestEnum::A(0, 0, false)));
    assert!(!TestEnum::A(0, 100, true).test_eq(&TestEnum::A(255, 100, true)));
    assert!(!TestEnum::A(255, 100, true).test_eq(&TestEnum::A(255, 0, true)));
    assert!(!TestEnum::A(255, 100, true).test_eq(&TestEnum::A(255, 100, false)));
    assert!(!TestEnum::B(255, 0, ()).test_eq(&TestEnum::B(255, 100, ())));
    assert!(!TestEnum::B(255, 100, ()).test_eq(&TestEnum::B(0, 100, ())));
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
        #[test_eq(cmp = "eq")]
        a: u32,

        #[test_eq(cmp = "eq")]
        b: bool,

        #[test_eq(cmp = "eq")]
        c: (),
    }

    impl TestStruct {
        fn new(a: u32, b: bool, c: ()) -> Self {
            Self { a, b, c }
        }
    }

    assert!(TestStruct::new(0, false, ()).test_eq(&TestStruct::new(0, false, ())));
    assert!(TestStruct::new(20, false, ()).test_eq(&TestStruct::new(20, false, ())));
    assert!(TestStruct::new(928, true, ()).test_eq(&TestStruct::new(928, true, ())));

    assert!(!TestStruct::new(0, true, ()).test_eq(&TestStruct::new(0, false, ())));
    assert!(!TestStruct::new(20, false, ()).test_eq(&TestStruct::new(22, false, ())));
    assert!(!TestStruct::new(928, true, ()).test_eq(&TestStruct::new(908, false, ())));
}

#[test]
fn test_struct_attr_ignore() {
    #[derive(TestEq)]
    struct TestStruct {
        #[test_eq(cmp = "eq")]
        a: u32,

        #[test_eq(ignore)]
        b: bool,
    }

    impl TestStruct {
        fn new(a: u32, b: bool) -> Self {
            Self { a, b }
        }
    }

    assert!(TestStruct::new(0, false).test_eq(&TestStruct::new(0, false)));
    assert!(TestStruct::new(20, false).test_eq(&TestStruct::new(20, false)));
    assert!(TestStruct::new(928, true).test_eq(&TestStruct::new(928, true)));
    assert!(TestStruct::new(0, false).test_eq(&TestStruct::new(0, true)));
    assert!(TestStruct::new(20, true).test_eq(&TestStruct::new(20, false)));
    assert!(TestStruct::new(928, false).test_eq(&TestStruct::new(928, true)));

    assert!(!TestStruct::new(1, true).test_eq(&TestStruct::new(0, false)));
    assert!(!TestStruct::new(20, false).test_eq(&TestStruct::new(22, false)));
    assert!(!TestStruct::new(928, true).test_eq(&TestStruct::new(908, false)));

    println!("{}", TestStruct::new(0, false).b);
}
