#![allow(dead_code)]

struct Foo2(String);

impl_more::forward_deref_and_mut!(Foo2, ref str);

fn main() {}
