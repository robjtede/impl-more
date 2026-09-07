//! Concise, declarative trait implementation macros.
//!
//! Forward formatting traits such as `Debug`, `LowerHex`, and `Binary` to an inner value:
//!
//! ```
//! struct Mask(u32);
//! impl_more::forward_debug!(Mask);
//! impl_more::forward_lower_hex!(Mask);
//! assert_eq!(format!("{:#06x}", Mask(42)), "0x002a");
//! ```
//!
//! # `#[no_std]`
//!
//! Where possible, these macros emit `#[no_std]`-compatible code.
//!
//! # Usage
//!
//! ```
//! # use std::error::Error as _;
//! struct MyNewTypeStruct(String);
//!
//! impl_more::impl_as_ref!(MyNewTypeStruct => String);
//! impl_more::impl_as_mut!(MyNewTypeStruct => String);
//! impl_more::forward_as_ref!(MyNewTypeStruct => str);
//! impl_more::forward_as_mut!(MyNewTypeStruct => str);
//!
//! impl_more::impl_deref!(MyNewTypeStruct => String);
//! impl_more::impl_deref_mut!(MyNewTypeStruct);
//! // or, to deref through String too:
//! // impl_more::forward_deref_and_mut!(MyNewTypeStruct => ref str);
//!
//! impl_more::impl_from!(String => MyNewTypeStruct);
//! impl_more::impl_into!(MyNewTypeStruct => String);
//!
//! impl_more::forward_display!(MyNewTypeStruct);
//! impl_more::forward_from_str!(MyNewTypeStruct => String);
//!
//! struct Items<T>(Vec<T>);
//!
//! impl_more::forward_into_iterator!(<T> in Items<T> => Vec<T>; owned, ref, ref_mut);
//! impl_more::forward_from_iterator!(<T> in Items<T> => Vec<T>);
//! impl_more::forward_extend!(<T> in Items<T> => Vec<T>);
//!
//! enum MyEnum {
//!     Bar,
//!     Qux,
//! }
//!
//! impl_more::impl_display_enum!(MyEnum: Bar => "bar", Qux => "qux");
//!
//! enum Coords {
//!     Xy(i64, i64),
//!     Xyz(i64, i64, i64),
//!     Uv(i64, i64),
//! }
//!
//! impl_more::impl_display_enum!(
//!     Coords:
//!     Xy(x, y) => "{x}, {y}",
//!     Xyz(x, y, z) => "{x}, {y}, {z}",
//!     Uv(u, _) => "{u}",
//! );
//!
//! #[derive(Debug)]
//! struct MyError(eyre::Report);
//!
//! impl_more::forward_display!(MyError);
//! impl_more::forward_error!(MyError);
//!
//! let err = MyError(eyre::eyre!("something went wrong"));
//! assert_eq!(err.source().unwrap().to_string(), "something went wrong");
//!
//! #[derive(Debug)]
//! struct MyFieldError {
//!     cause: eyre::Report,
//! }
//!
//! impl_more::forward_display!(MyFieldError => cause);
//! impl_more::forward_error!(MyFieldError => cause);
//!
//! let err = MyFieldError {
//!     cause: eyre::eyre!("something else went wrong"),
//! };
//! assert_eq!(
//!     err.source().unwrap().to_string(),
//!     "something else went wrong"
//! );
//!
//! #[derive(Debug)]
//! enum Err {
//!     Io(std::io::Error),
//!     Generic(String),
//! }
//!
//! impl_more::impl_display_enum! {
//!     Err:
//!     Io(err) => "{err}",
//!     Generic(msg) => "{msg}",
//! }
//! impl_more::impl_error_enum!(Err: Io(err) => err);
//! ```

#![cfg_attr(not(docsrs), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

#[macro_use]
mod as_ref;
#[macro_use]
mod deref;
#[macro_use]
mod display;
#[macro_use]
mod error;
#[macro_use]
mod fmt;
#[macro_use]
mod from;
#[macro_use]
mod from_str;
#[macro_use]
mod iter;

#[cfg(test)]
mod tests {
    #![allow(dead_code, clippy::from_over_into)]

    use alloc::string::{String, ToString as _};
    use core::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Foo(String);

    crate::impl_as_ref!(Foo => String);
    crate::impl_as_mut!(Foo => String);

    crate::impl_deref!(Foo => String);
    crate::impl_deref_mut!(Foo);

    static_assertions::assert_impl_all!(
        Foo:
        // impls
        AsRef<String>,
        AsMut<String>,
        Deref<Target = String>,
        DerefMut,
    );

    #[derive(Debug)]
    struct Bar {
        foo: Foo,
    }

    crate::impl_as_ref!(Bar => foo: Foo);
    crate::impl_as_mut!(Bar => foo: Foo);

    crate::impl_deref!(Bar => foo: String);
    crate::impl_deref_mut!(Bar => foo);

    static_assertions::assert_impl_all!(
        Bar:
        // impls
        AsRef<Foo>,
        AsMut<Foo>,
        Deref<Target = String>,
        DerefMut,
    );

    #[allow(dead_code)]
    #[test]
    fn impl_display() {
        enum Foo {
            Bar,
            Qux,
        }
        crate::impl_display_enum!(Foo: Bar => "bar", Qux => "qux");
        assert_eq!(Foo::Bar.to_string(), "bar");
        assert_eq!(Foo::Qux.to_string(), "qux");

        enum FooComma {
            Bar,
            Qux,
        }
        crate::impl_display_enum!(FooComma: Bar => "bar", Qux => "qux",);

        enum FooContents {
            Bar(u64, u64),
        }
        crate::impl_display_enum!(FooContents: Bar (x, y) => "x: {x}; y: {y}");
        assert_eq!(FooContents::Bar(4, 2).to_string(), "x: 4; y: 2");

        enum FooContents2 {
            Qux { msg: &'static str },
        }
        crate::impl_display_enum!(FooContents2: Qux { msg } => "msg: {msg}");
        assert_eq!(FooContents2::Qux { msg: "foo" }.to_string(), "msg: foo");

        // not supported yet
        // enum FooContents3 {
        //     Bar(u64, u64),
        //     Qux { msg: &'static str },
        // }
        // impl_display_enum!(FooContents3, Bar (x, y) => "x: {x}; y: {y}", Qux { msg } => "{msg}");
        // assert_eq!(FooContents3::Bar(4, 2).to_string(), "x: 4; y: 2");
        // assert_eq!(FooContents3::Qux { msg: "foo" }.to_string(), "x: 4; y: 2");
    }
}
