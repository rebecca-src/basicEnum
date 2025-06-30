use basic_enum::ParseEnum;

#[repr(u8)]
#[derive(ParseEnum, Debug, PartialEq)]
pub enum TestEnumOne {
    Unknown,
    Start,
    Middle,
    End,
}

#[repr(u16)]
#[derive(ParseEnum, Debug, PartialEq)]
pub enum TestEnumTwo {
    Unknown = 0,
    Case1 = 1,
    Case3 = 3,
}

#[test]
fn test_macro() {
    assert_eq!(TestEnumOne::from_number(0), TestEnumOne::Unknown);
    assert_eq!(TestEnumOne::from_number(1), TestEnumOne::Start);
    assert_eq!(TestEnumOne::from_number(2), TestEnumOne::Middle);
    assert_eq!(TestEnumOne::from_number(6), TestEnumOne::Unknown);

    assert_eq!(TestEnumOne::to_number(TestEnumOne::Unknown), 0);
    assert_eq!(TestEnumOne::to_number(TestEnumOne::Start), 1);
    assert_eq!(TestEnumOne::to_number(TestEnumOne::Middle), 2);

    assert_eq!(TestEnumOne::Middle, 2.into());
    assert_eq!(TestEnumOne::Start.into(), 1);
}

#[test]
fn test_macro2() {
    assert_eq!(TestEnumTwo::from_number(0), TestEnumTwo::Unknown);
    assert_eq!(TestEnumTwo::from_number(1), TestEnumTwo::Case1);
    assert_eq!(TestEnumTwo::from_number(2), TestEnumTwo::Unknown);
    assert_eq!(TestEnumTwo::from_number(3), TestEnumTwo::Case3);
    assert_eq!(TestEnumTwo::from_number(6), TestEnumTwo::Unknown);

    assert_eq!(TestEnumTwo::to_number(TestEnumTwo::Unknown), 0);
    assert_eq!(TestEnumTwo::to_number(TestEnumTwo::Case1), 1);
    assert_eq!(TestEnumTwo::to_number(TestEnumTwo::Case3), 3);

    assert_eq!(TestEnumTwo::Case3, 3.into());
    assert_eq!(TestEnumTwo::Case1.into(), 1);
}
