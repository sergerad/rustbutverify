# kv-rs

Concurrent, durable KV store written in Rust.

### Usage

```rust
use kv::DurableKv;

let file_path = std::path::Path::new("./kv.db");

let kv: DurableKv<String, i32> = DurableKv::new(file_path).unwrap();

kv.put("hello".to_string(), 1);
assert_eq!(Some(1), kv.get("hello".to_string()));
```

### Tests

Run `cargo test` and `cargo bench`.
