use alloc::string::String;

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
