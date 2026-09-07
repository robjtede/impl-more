# `impl-more`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/impl-more?label=latest)](https://crates.io/crates/impl-more)
[![Documentation](https://docs.rs/impl-more/badge.svg?version=0.3.5)](https://docs.rs/impl-more/0.3.5)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/impl-more.svg)
<br />
[![dependency status](https://deps.rs/crate/impl-more/0.3.5/status.svg)](https://deps.rs/crate/impl-more/0.3.5)
[![Download](https://img.shields.io/crates/d/impl-more.svg)](https://crates.io/crates/impl-more)
[![CI](https://github.com/robjtede/impl-more/actions/workflows/ci.yml/badge.svg)](https://github.com/robjtede/impl-more/actions/workflows/ci.yml)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

Concise, declarative trait implementation macros.

Forward formatting traits such as `Debug`, `LowerHex`, and `Binary` to an inner value:

```rust
struct Mask(u32);

impl_more::forward_debug!(Mask);
impl_more::forward_lower_hex!(Mask);

assert_eq!(format!("{:#06x}", Mask(42)), "0x002a");
```

## `#[no_std]`

Where possible, these macros emit `#[no_std]`-compatible code.

## Usage

```rust
struct MyNewTypeStruct(String);

impl_more::impl_as_ref!(MyNewTypeStruct => String);
impl_more::impl_as_mut!(MyNewTypeStruct => String);
impl_more::forward_as_ref!(MyNewTypeStruct => str);
impl_more::forward_as_mut!(MyNewTypeStruct => str);

impl_more::impl_deref!(MyNewTypeStruct => String);
impl_more::impl_deref_mut!(MyNewTypeStruct);
// or, to deref through String too:
// impl_more::forward_deref_and_mut!(MyNewTypeStruct => ref str);

impl_more::impl_from!(String => MyNewTypeStruct);
impl_more::impl_into!(MyNewTypeStruct => String);

impl_more::forward_display!(MyNewTypeStruct);
impl_more::forward_from_str!(MyNewTypeStruct => String);

struct Items<T>(Vec<T>);

impl_more::forward_into_iterator!(<T> in Items<T> => Vec<T>; owned, ref, ref_mut);
impl_more::forward_from_iterator!(<T> in Items<T> => Vec<T>);
impl_more::forward_extend!(<T> in Items<T> => Vec<T>);

enum MyEnum {
    Bar,
    Qux,
}

impl_more::impl_display_enum!(MyEnum: Bar => "bar", Qux => "qux");

enum Coords {
    Xy(i64, i64),
    Xyz(i64, i64, i64),
    Uv(i64, i64),
}

impl_more::impl_display_enum!(
    Coords:
    Xy(x, y) => "{x}, {y}",
    Xyz(x, y, z) => "{x}, {y}, {z}",
    Uv(u, _) => "{u}",
);

#[derive(Debug)]
struct MyError(eyre::Report);

impl_more::forward_display!(MyError);
impl_more::forward_error!(MyError);

let err = MyError(eyre::eyre!("something went wrong"));
assert_eq!(err.source().unwrap().to_string(), "something went wrong");

#[derive(Debug)]
struct MyFieldError {
    cause: eyre::Report,
}

impl_more::forward_display!(MyFieldError => cause);
impl_more::forward_error!(MyFieldError => cause);

let err = MyFieldError {
    cause: eyre::eyre!("something else went wrong"),
};
assert_eq!(
    err.source().unwrap().to_string(),
    "something else went wrong"
);

#[derive(Debug)]
enum Err {
    Io(std::io::Error),
    Generic(String),
}

impl_more::impl_display_enum! {
    Err:
    Io(err) => "{err}",
    Generic(msg) => "{msg}",
}
impl_more::impl_error_enum!(Err: Io(err) => err);
```

<!-- cargo-rdme end -->
