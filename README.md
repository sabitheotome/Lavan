# Lavan Parser - UNSTABLE

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![GPLv3 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/lavan.svg
[crates-url]: https://crates.io/crates/lavan
[docs-badge]: https://docs.rs/lavan/badge.svg
[docs-url]: https://docs.rs/lavan
[license-badge]: https://img.shields.io/crates/l/lavan.svg
[license-url]: https://github.com/sabitheotome/lavan/blob/master/LICENSE
[actions-badge]: https://github.com/sabitheotome/lavan/workflows/CI/badge.svg
[actions-url]: https://github.com/sabitheotome/lavan/actions

### *Lavan is a mean, lean, parsing machine!*  
A library for empowering users to build concise parsers, either for domain-specific, or general-purpose languages.

> [!WARNING]  
> ## This project is currently experimental. Literally, everything is subject to changes, even the repository link and the license.
>
> ## We STRONGLY ADVISE you to NOT USE this library in production.

# Example

## Parsing an email address

```rust
#![cfg(feature = "unstable-api-2021")]
use lavan::prelude::*;

fn main() {
    let input = "es4fbero15181@r65dgh51.com";

    let names_and_dots = any_if(char::is_ascii_alphanumeric)
        .discard()
        .repeat_min(1)
        .repeat_min(1)
        .separated_by(any_eq('.').discard())
        .slice();

    let email: Option<(&str, &str)> = names_and_dots
        .as_ref()
        .and(any_eq('@').discard())
        .and(names_and_dots.as_ref())
        .evaluate(input.chars());

    let (username, hostname) = email.unwrap();
    assert_eq!(username, "es4fbero15181");
    assert_eq!(hostname, "r65dgh51.com");
}
```

# Release cycle

- Lavan will reach v0.1.0-unstable once it's 2024 rust edition migration is complete.
- After a solid and stable API, Lavan will reach 1.0.0-pre-alpha. Minor refactoring and optimizations will be prioritized.
- Once benchmarks are made and code is consolidated, we will reach 1-0-0-alpha.
- Once unofficially ready-to-production versions are ready, we reached 1-0-0-beta.
- After several unit tests, integration tests, benchmarks, and auditions, we will reach 1-0-0-rc.
