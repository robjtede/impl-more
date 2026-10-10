use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Debug, Clone)]
struct Foo(String);

impl_more::impl_as_ref!(Foo => String);
impl_more::impl_as_mut!(Foo => String);
impl_more::forward_as_ref!(Foo => str);
impl_more::forward_as_mut!(Foo => str);

impl_more::impl_deref!(Foo => String);
impl_more::impl_deref_mut!(Foo);

struct Array<T, const N: usize>([T; N]);

impl_more::impl_deref_and_mut!(<T, const N: usize> in Array<T, N> => [T; N]);
impl_more::impl_as_ref!(<T, const N: usize> in Array<T, N> => [T; N]);
impl_more::impl_as_mut!(<T, const N: usize> in Array<T, N> => [T; N]);
impl_more::forward_as_ref!(<T, const N: usize> in Array<T, N> => [T]);
impl_more::forward_as_mut!(<T, const N: usize> in Array<T, N> => [T]);
impl_more::impl_from!(<T, const N: usize> in [T; N] => Array<T, N>);
impl_more::impl_into!(<T, const N: usize> in Array<T, N> => [T; N]);
impl_more::forward_into_iterator!(
    <T, const N: usize> in Array<T, N> => [T; N]; owned, ref, ref_mut
);
impl_more::forward_debug!(<T, const N: usize> in Array<T, N>);

struct BoxArray<T, const N: usize>(Box<[T; N]>);

impl_more::forward_deref_and_mut!(<T, const N: usize> in BoxArray<T, N> => [T; N]);
impl_more::forward_as_ref_and_mut!(<T, const N: usize> in BoxArray<T, N> => [T; N]);

struct Buffer<const COUNT: usize> {
    items: [u8; COUNT],
}

impl_more::impl_deref!(<const COUNT: usize> in Buffer<COUNT> => items: [u8; COUNT]);
impl_more::impl_deref_mut!(<const COUNT: usize> in Buffer<COUNT> => items);

impl_more::impl_from!(String => Foo);
impl_more::impl_into!(Foo => String);

impl_more::forward_display!(Foo);
impl_more::forward_from_str!(Foo => String);

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

enum DisplayEvent {
    Idle,
    Items(&'static [u8]),
    Progress { completed: usize, total: usize },
}

impl_more::impl_display_enum! {
    DisplayEvent:
    Idle => "idle",
    Items(items) => ("{} items", items.len()),
    Progress { completed: count, .. } => "{count} complete",
}

#[derive(Debug, Clone)]
struct Baz<T> {
    inner: T,
}

impl_more::forward_display!(<T> in Baz<T> => inner);
impl_more::forward_from_str!(<T> in Baz<T> => inner: T);

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
impl_more::impl_enum_from!(LeafErr => Errors::Wrapped);

enum GenericError<T, const N: usize> {
    Other { source: [T; N] },
}

impl_more::impl_enum_from!(<T, const N: usize> in [T; N] => GenericError<T, N>::Other { source });

#[derive(Debug, Clone)]
struct Checked(bool);

impl_more::impl_newtype_from_into!(Checked [<=>] bool);

impl_more::forward_into_iterator!(<T> in Qux<T> => Vec<T>; owned, ref, ref_mut);
impl_more::forward_from_iterator!(<T> in Qux<T> => Vec<T>);
impl_more::forward_extend!(<T> in Qux<T> => Vec<T>);

struct Bounded<T, const N: usize> {
    values: Vec<T>,
}

impl<T, const N: usize> core::iter::FromIterator<T> for Bounded<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            values: iter.into_iter().take(N).collect(),
        }
    }
}

impl<T, const N: usize> Extend<T> for Bounded<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let remaining = N - self.values.len();

        self.values.extend(iter.into_iter().take(remaining));
    }
}

struct Collection<T, const N: usize>(Bounded<T, N>);

impl_more::forward_from_iterator!(<T, const N: usize> in Collection<T, N> => Bounded<T, N>);
impl_more::forward_extend!(<T, const N: usize> in Collection<T, N> => Bounded<T, N>);

struct FormatValue<T>(T);

impl_more::forward_debug!(<T> in FormatValue<T>);
impl_more::forward_binary!(<T> in FormatValue<T>);
impl_more::forward_octal!(<T> in FormatValue<T>);
impl_more::forward_lower_hex!(<T> in FormatValue<T>);
impl_more::forward_upper_hex!(<T> in FormatValue<T>);
impl_more::forward_lower_exp!(<T> in FormatValue<T>);
impl_more::forward_upper_exp!(<T> in FormatValue<T>);
impl_more::forward_pointer!(<T> in FormatValue<T>);
