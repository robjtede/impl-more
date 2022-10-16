#![allow(dead_code)]

enum FooContents3 {
    Bar(u64, u64),
    Qux { msg: &'static str },
}

// impl_more::impl_display_enum!(FooContents3, Bar (x, y) => "x: {x}; y: {y}", Qux { msg } => "{msg}");

fn main() {}
