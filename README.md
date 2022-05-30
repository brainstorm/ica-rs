# ICA Rust client

WIP, only GDS v1 supported, highly alpha/experimental code

# Re-generate client stubs

This crate will use upstream OpenAPI Codegen:

```
brew install openapi-generator
openapi-generator generate -i openapi/gds.json -g rust -o src/gds
```

I wish Oxide Computer progenitor worked with this [cursed multi-MediaType OpenAPI definition](https://converter.swagger.io/api/convert?url=https%3A%2F%2Faps2.platform.illumina.com%2Fgds%2Fswagger%2Fv1%2Fswagger.json), [but it does not right now](https://github.com/oxidecomputer/progenitor/issues/76) :-S
