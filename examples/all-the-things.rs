#[derive(Debug, Clone)]
struct Foo(String);

impl_more::impl_as_ref!(Foo, String);
impl_more::impl_as_mut!(Foo, String);

impl_more::impl_deref!(Foo, String);
impl_more::impl_deref_mut!(Foo);

impl_more::impl_from!(Foo, String);
impl_more::impl_into!(Foo, String);

fn main() {
    let mut foo = Foo("bar".to_owned());

    foo.as_mut().push('!');
    assert_eq!(foo.as_ref(), "bar!");

    assert_eq!(foo.len(), 3);

    let mut foo = Foo("bar".to_owned());
    foo.push('!');
    assert_eq!(*foo, "bar!");

    let _foo = Foo::from("bar".to_owned());

    let foo = Foo("bar".to_owned());
    let _foo_str: String = foo.into();
}
