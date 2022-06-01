# ICA Rust client

WIP, only GDS v1 supported, highly alpha/experimental code

# Re-generate client stubs

This crate will use upstream OpenAPI Codegen:

```shell
$ brew install openapi-generator
$ openapi-generator generate -i openapi/v1/gds.json -g rust -o gds
```

Note: Currently it sets the version wrongly on `Cargo.toml`, needs to be fixed upstream or locally.

# Examples

```shell
$ cd examples/get_presigned_url
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/ica-get-presigned-url`
https://stratus-gds-aps2.s3.ap-southeast-2.amazonaws.com/1f412... <-- presigned url
```
