# JSmoke personal macros

This module provides JSmoke macros. It's intended to auto-implement
default behavior traits (traits that contains default functions).

It turns the coding a lot simpler:
```rust
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
