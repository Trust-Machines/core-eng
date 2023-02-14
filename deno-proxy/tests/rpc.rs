use std::io::Error;

use deno_proxy::new;
use serde_json::from_str;

fn test_wrap() -> Result<(), Error> {
    let mut js = new(".")?;
    {
        let result = js.call(from_str("{\"b\":[],\"a\":2}")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"object\",{\"a\":2,\"b\":[]}]]");
    }
    {
        let result = js.call(from_str("[54,null]")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"array\",[54,null]]]");
    }
    {
        let result = js.call(from_str("42")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"number\",42]]");
    }
    {
        let result = js.call(from_str("\"Hello!\"")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"string\",\"Hello!\"]]");
    }
    {
        let result = js.call(from_str("true")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"boolean\",true]]");
    }
    {
        let result = js.call(from_str("null")?)?;
        assert_eq!(result.to_string(), "[\"ok\",[\"null\"]]");
    }
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}
