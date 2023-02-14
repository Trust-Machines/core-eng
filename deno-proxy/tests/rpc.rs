use std::io::Error;

use deno_proxy::new;
use serde_json::from_str;

fn test_wrap() -> Result<(), Error> {
    let mut js = new(".")?;
    {
        let result = js.call(from_str("{\"b\":[],\"a\":2}")?)?;
        assert_eq!(result.to_string(), "[\"object\",{\"a\":2,\"b\":[]}]");
    }
    {
        let result = js.call(from_str("[54,null]")?)?;
        assert_eq!(result.to_string(), "[\"array\",[54,null]]");
    }
    {
        let result = js.call(from_str("42")?)?;
        assert_eq!(result.to_string(), "[\"number\",42]");
    }
    {
        let result = js.call(from_str("\"Hello!\"")?)?;
        assert_eq!(result.to_string(), "[\"string\",\"Hello!\"]");
    }
    {
        let result = js.call(from_str("true")?)?;
        assert_eq!(result.to_string(), "[\"boolean\",true]");
    }
    {
        let result = js.call(from_str("null")?)?;
        assert_eq!(result.to_string(), "[\"null\"]");
    }
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}
