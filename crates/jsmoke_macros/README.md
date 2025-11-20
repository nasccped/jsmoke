# JSmoke personal macros

[![crates io](https://img.shields.io/crates/v/jsmoke_macros.svg)](https://crates.io/crates/jsmoke_macros)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue?)](#)

This module provides JSmoke macros. It's intended to auto-implement
default behavior traits (traits that contains default functions).

It turns the coding a lot simpler:
```rust,compile_fail
trait SomeTrait {
    // code here ...
}

// Use this:
#[derive(SomeTrait)]
struct SomeStruct {
    // fields ...
}

// Instead of this:
impl SomeTrait for SomeStruct {
    // ...
}
```

## Currently added macros

This crate currently provides the:

### [`ErrorPrint`] proc-derive macro

Auto implements the [`jsmoke_utils::printing::error_print::ErrorPrint`]
trait on the following type (`enum`, `struct`, ...).

Note that the type must also implements the [`std::error::Error`]
trait since [`jsmoke_utils::printing::error_print::ErrorPrint`]
requires it:

```rust
// from jsmoke_utils::printing::error_print =>
pub trait ErrorPrint: std::error::Error {
    // ...
}
```

### [`WarningPrint`] proc-derive macro

Auto implements the [`jsmoke_utils::printing::warning_pring::WarningPrint`]
trait on the following type (`enum`, `struct`, ...).

Note that the type must also implements the [`std::error::Error`]
trait since [`jsmoke_utils::printing::warning_pring::WarningPrint`]
requires it:

```rust
// from jsmoke_utils::printing::warning_print =>
pub trait WarningPrint: std::error::Error {
    // ...
}
```

Even not being an error at all, [`WarningPrint`] implementers must
also implements [`std::error::Error`], and then, we can easily apply
the [`WarningPrint`] derive by also using the [`thiserror::Error`]
`proc-macro`.
