[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# intdate

A simple integer date library inspired by Python's `int_date`.

## Sample Usage
```rust
    let date = intdate::from_str("2020-01-01").unwrap();
    println!("{}", date.is_year_leap());
```

[ci]: https://github.com/Elinvynia/intdate/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/intdate/Rust/master?style=flat-square
[docs]: https://docs.rs/intdate
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/intdate
[crate-version]: https://img.shields.io/crates/v/intdate.svg?style=flat-square
