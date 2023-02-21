use stackes_js_rs::{rpc::Rpc, Js};

fn main() {
    let mut js = Js::new("./stacks-js-rs/js/mirror.ts").unwrap();
    js.call::<_, serde_json::Value>(&42).unwrap();
}
