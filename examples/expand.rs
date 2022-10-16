#![allow(dead_code, clippy::from_over_into)]

struct MyNewTypeStruct(String);

impl_more::impl_as_ref!(MyNewTypeStruct, String);
impl_more::impl_as_mut!(MyNewTypeStruct, String);

impl_more::impl_deref!(MyNewTypeStruct, String);
impl_more::impl_deref_mut!(MyNewTypeStruct);
// or
// impl_more::forward_deref_and_mut!(MyNewTypeStruct, ref str);

impl_more::impl_from!(MyNewTypeStruct, String);
impl_more::impl_into!(MyNewTypeStruct, String);

enum MyEnum {
    Bar,
    Qux,
}

impl_more::impl_display_enum!(MyEnum, Bar => "bar", Qux => "qux");

enum Coords {
    Xy(i64, i64),
    Xyz(i64, i64, i64),
}

impl_more::impl_display_enum!(Coords, Xy(x, y) => "{x}, {y}", Xyz(x, y, z) => "{x}, {y}, {z}");

fn main() {}
