# ICA Rust client

WIP, only GDS v1 supported, highly alpha/experimental code

# Re-generate client stubs

This crate will use upstream OpenAPI Codegen:

```shell
$ brew install openapi-generator
$ openapi-generator generate -i openapi/gds.json -g rust -o src/gds
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

# OpenAPI vs Progenitor

I wish Oxide Computer progenitor worked with this [cursed multi-MediaType OpenAPI definition](https://converter.swagger.io/api/convert?url=https%3A%2F%2Faps2.platform.illumina.com%2Fgds%2Fswagger%2Fv1%2Fswagger.json), [but it does not right now](https://github.com/oxidecomputer/progenitor/issues/76) :-S
