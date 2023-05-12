## Pest-CSS-Parser

[![crates.io](https://img.shields.io/crates/v/pest-css-parser.svg)](https://crates.io/crates/pest-css-parser) [![docs.rs](https://docs.rs/pest-css-parser/badge.svg)](https://docs.rs/pest-css-parser)

Rust parser for **css** files using pest.

### Usage

```rust
let input = r#"
    span {
        display: inline;
    }
"#;
let stylesheet = StyleSheet::parse(input).unwrap();
assert_eq!(stylesheet.rules.len(), 1);
```

### Examples

- [Parse](../examples/parse.rs)

### Task list

- [ ] Write documentation
- [ ] Create a writer
