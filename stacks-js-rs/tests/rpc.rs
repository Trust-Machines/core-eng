use std::io::Error;

use serde_json::{from_str, to_string, Value};
use stackes_js_rs::Js;

fn to_value(s: &str) -> Result<Value, Error> {
    let x = from_str(s)?;
    Ok(x)
}

fn test_wrap() -> Result<(), Error> {
    let mut js = Js::new(".")?;
    {
        let result = js.call(&to_value("{\"b\":[],\"a\":2}")?)?;
        assert_eq!(result.to_string(), "[\"object\",{\"a\":2,\"b\":[]}]");
    }
    {
        let result = js.call(&to_value("[54,null]")?)?;
        assert_eq!(result.to_string(), "[\"array\",[54,null]]");
    }
    {
        let result = js.call(&to_value("42")?)?;
        assert_eq!(result.to_string(), "[\"number\",42]");
    }
    {
        let result = js.call(&to_value("\"Hello!\"")?)?;
        assert_eq!(result.to_string(), "[\"string\",\"Hello!\"]");
    }
    {
        let result = js.call(&to_value("true")?)?;
        assert_eq!(result.to_string(), "[\"boolean\",true]");
    }
    {
        let result = js.call(&to_value("null")?)?;
        assert_eq!(result.to_string(), "[\"null\"]");
    }
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}
