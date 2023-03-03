#![allow(clippy::from_over_into, clippy::disallowed_names)]

use std::error::Error as _;

#[derive(Debug, Clone)]
struct Foo(String);

impl_more::impl_as_ref!(Foo => String);
impl_more::impl_as_mut!(Foo => String);

impl_more::impl_deref!(Foo => String);
impl_more::impl_deref_mut!(Foo);

impl_more::impl_from!(String => Foo);
impl_more::impl_into!(Foo => String);

impl_more::forward_display!(Foo);

#[derive(Debug, Clone)]
struct Bar {
    inner: String,
}

impl_more::forward_display!(Bar => inner);

#[derive(Debug)]
enum FooEnum {
    Bar,
    Qux,
}

impl_more::impl_display_enum!(FooEnum, Bar => "bar", Qux => "qux");

#[derive(Debug, Clone)]
struct Baz<T> {
    inner: T,
}

impl_more::forward_display!(<T> in Baz<T> => inner);

#[derive(Debug)]
enum Err {
    Io(std::io::Error),
    Generic(String),
}

impl_more::impl_display_enum!(Err, Io(err) => "{err}", Generic(msg) => "{msg}");
impl_more::impl_error_enum!(Err, Io(err) => err);

fn main() {
    let mut foo = Foo("bar".to_owned());

    foo.as_mut().push('!');
    assert_eq!(foo.as_ref(), "bar!");

    assert_eq!(foo.len(), 4);

    let mut foo = Foo("bar".to_owned());
    foo.push('!');
    assert_eq!(*foo, "bar!");

    let _foo = Foo::from("bar".to_owned());

    let foo = Foo("bar".to_owned());
    let _foo_str: String = foo.into();

    assert_eq!(FooEnum::Bar.to_string(), "bar");
    assert_eq!(FooEnum::Qux.to_string(), "qux");

    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
    assert!(Err::Io(io_err).source().is_some());
    assert!(Err::Generic("oops".to_owned()).source().is_none());
}
