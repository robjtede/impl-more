# `impl-more`

> Concise trait implementations.

# Usage

```rust
struct MyNewTypeStruct(String);

impl_more::impl_as_ref!(MyNewTypeStruct, String);
impl_more::impl_as_mut!(MyNewTypeStruct, String);

impl_more::impl_deref!(MyNewTypeStruct, String);
impl_more::impl_deref_mut!(MyNewTypeStruct);

impl_more::impl_from!(MyNewTypeStruct, String);
impl_more::impl_into!(MyNewTypeStruct, String);

enum MyEnum {
    Bar,
    Qux,
}

impl_more::impl_display_enum!(MyEnum, Bar => "bar", Qux => "qux");
```
