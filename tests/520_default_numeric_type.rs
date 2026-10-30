use ron::{
    options::{DefaultNumericType, UntypedFloatType, UntypedIntegerType},
    Options, Value,
};

#[test]
fn default_numeric_type_smallest_is_unchanged() {
    let value: Value = Options::default().from_str("42").unwrap();
    assert_eq!(value, Value::Number(ron::Number::U8(42)));
}

#[test]
fn default_numeric_type_minimum_u32_f64() {
    let options = Options::default().with_default_numeric_type(DefaultNumericType::Minimum {
        integer: UntypedIntegerType::U32,
        float: UntypedFloatType::F64,
    });

    let value: Value = options.from_str("42").unwrap();
    assert_eq!(value, Value::Number(ron::Number::U32(42)));

    let value: Value = options.from_str("3.14").unwrap();
    assert_eq!(
        value,
        Value::Number(ron::Number::F64(ron::value::F64(3.14)))
    );

    let value: Value = options.from_str("inf").unwrap();
    assert_eq!(
        value,
        Value::Number(ron::Number::F64(ron::value::F64(f64::INFINITY)))
    );
}

#[test]
fn default_numeric_type_minimum_falls_back_when_out_of_range() {
    let options = Options::default().with_default_numeric_type(DefaultNumericType::Minimum {
        integer: UntypedIntegerType::U32,
        float: UntypedFloatType::F64,
    });

    let value: Value = options.from_str("5000000000").unwrap();
    assert_eq!(value, Value::Number(ron::Number::U64(5_000_000_000)));
}
