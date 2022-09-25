#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(future_incompatible)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_export]
macro_rules! impl_as_ref {
    ($ty:ty, $inner:ty) => {
        impl ::std::convert::AsRef<$inner> for $ty {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_mut {
    ($ty:ty, $inner:ty) => {
        impl ::std::convert::AsMut<$inner> for $ty {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // use derive_more::AsRef;

    // use super::*;

    #[derive(Debug)]
    struct Foo(String);

    impl_as_ref!(Foo, String);
    impl_as_mut!(Foo, String);

    static_assertions::assert_impl_all!(Foo: AsRef<String>, AsMut<String>);
}
