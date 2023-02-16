use deno_proxy::new;

fn main() {
    let _ = new("./deno-proxy").unwrap();
}
