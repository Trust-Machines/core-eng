use std::io;

use yarpc::rpc::{js::Js, Rpc};

fn to_value(s: &str) -> io::Result<serde_json::Value> {
    let x = serde_json::from_str(s)?;
    Ok(x)
}

fn json_call(js: &mut Js, input: &str) -> io::Result<String> {
    Ok(js
        .call::<_, serde_json::Value>(&to_value(input)?)?
        .to_string())
}

fn test_wrap() -> io::Result<()> {
    let mut js = Js::new("./js/tests/mirror.ts")?;
    assert_eq!(
        json_call(&mut js, "{\"b\":[],\"a\":2}")?,
        "{\"a\":2,\"b\":[]}"
    );
    assert_eq!(json_call(&mut js, "[54,null]")?, "[54,null]");
    assert_eq!(json_call(&mut js, "42")?, "42");
    assert_eq!(json_call(&mut js, "\"Hello!\"")?, "\"Hello!\"");
    assert_eq!(json_call(&mut js, "true")?, "true");
    assert_eq!(json_call(&mut js, "null")?, "null");
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}

#[test]
fn test_err() {
    let mut js = Js::new("./js/tests/throw.ts").unwrap();
    let error = js.call::<_, serde_json::Value>(&42);
    assert!(error.is_err());
}

#[test]
fn test_async_err() {
    let mut js = Js::new("./js/tests/async_throw.ts").unwrap();
    let error = js.call::<_, serde_json::Value>(&42);
    assert!(error.is_err());
}
