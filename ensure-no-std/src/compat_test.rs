use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone)]
struct Foo(String);

impl_more::impl_as_ref!(Foo => String);
impl_more::impl_as_mut!(Foo => String);
impl_more::forward_as_ref!(Foo => str);
impl_more::forward_as_mut!(Foo => str);

impl_more::impl_deref!(Foo => String);
impl_more::impl_deref_mut!(Foo);

impl_more::impl_from!(String => Foo);
impl_more::impl_into!(Foo => String);
impl_more::forward_from_str!(Foo => String);

impl_more::forward_display!(Foo);

#[derive(Debug, Clone)]
struct Bar {
    inner: String,
}

impl_more::forward_as_ref_and_mut!(Bar => inner: str);
impl_more::forward_display!(Bar => inner);

#[derive(Debug)]
enum FooEnum {
    Bar,
    Qux,
}

impl_more::impl_display_enum!(FooEnum: Bar => "bar", Qux => "qux");

#[derive(Debug, Clone)]
struct Baz<T> {
    inner: T,
}

impl_more::forward_from_str!(<T> in Baz<T> => inner: T);
impl_more::forward_display!(<T> in Baz<T> => inner);

#[derive(Debug, Clone)]
struct Qux<T>(Vec<T>);

impl_more::forward_as_ref_and_mut!(<T> in Qux<T> => [T]);

#[derive(Debug)]
struct LeafErr;

impl_more::impl_display!(LeafErr: "leaf");
impl_more::impl_error_enum!(LeafErr);

#[derive(Debug)]
enum Errors {
    Wrapped(LeafErr),
}

impl_more::impl_display!(Errors: "wrapped");
impl_more::impl_error_enum!(Errors: Wrapped(err) => err);

#[derive(Debug, Clone)]
struct Checked(bool);

impl_more::impl_newtype_from_into!(Checked [<=>] bool);
