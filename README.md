# Basic Rust Enumeration
Use the `ParseEnum` derive to add `to_number` and `from_number`.
The first item is always the default if the value was not found.

```rust
#[repr(u8)]
#[derive(ParseEnum, Debug, PartialEq)]
pub enum TestEnum {
    Unknown = 0,
    Case1 = 1,
    Case3 = 3,
}

fn main() {
    assert_eq!(TestEnum::to_number(TestEnumOne::Case3), 3);
    assert_eq!(TestEnumTwo::from_number(1), TestEnumTwo::Case1);
    assert_eq!(TestEnumTwo::from_number(2), TestEnumTwo::Unknown);
    assert_eq!(TestEnum::Case1, 1.into());
    assert_eq!(TestEnum::Case3.into(), 3);
}
```