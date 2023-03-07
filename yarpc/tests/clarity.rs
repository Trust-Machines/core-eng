use clarity::vm::Value;

#[test]
fn test() {
    let f = |v: Value| v.to_string();
    assert_eq!(f(Value::Int(-0x1)), r#"-1"#);
    assert_eq!(
        f(Value::Int(-0x1_0000_0000_0000_0001)),
        r#"-18446744073709551617"#
    );
    assert_eq!(f(Value::Bool(true)), r#"true"#);
    assert_eq!(f(Value::Bool(false)), r#"false"#);
    assert_eq!(f(Value::UInt(34)), "u34");
}
