/// Implement [`AsRef`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// wrapped type.
///
/// # Examples
/// ```
/// use impl_more::impl_as_ref;
///
/// struct Foo(String);
///
/// impl_as_ref!(Foo, String);
///
/// let foo = Foo("bar".to_owned());
/// assert_eq!(foo.as_ref(), "bar");
/// ```
#[macro_export]
macro_rules! impl_as_ref {
    ($ty:ty, $inner:ty) => {
        impl ::core::convert::AsRef<$inner> for $ty {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }
    };

    ($ty:ty, $inner:ty, $field:ident) => {
        impl ::core::convert::AsRef<$inner> for $ty {
            fn as_ref(&self) -> &$inner {
                &self.$field
            }
        }
    };
}

/// Implement [`AsMut`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// wrapped type.
///
/// # Examples
/// ```
/// use impl_more::{impl_as_ref, impl_as_mut};
///
/// struct Foo(String);
///
/// impl_as_ref!(Foo, String);
/// impl_as_mut!(Foo, String);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.as_mut().push('!');
///
/// assert_eq!(foo.as_ref(), "bar!");
/// ```
#[macro_export]
macro_rules! impl_as_mut {
    ($ty:ty, $inner:ty) => {
        impl ::core::convert::AsMut<$inner> for $ty {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };

    ($ty:ty, $inner:ty, $field:ident) => {
        impl ::core::convert::AsMut<$inner> for $ty {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.$field
            }
        }
    };
}

/// Implement [`Deref`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// wrapped type.
///
/// # Examples
/// ```
/// use impl_more::impl_deref;
///
/// struct Foo(String);
///
/// impl_deref!(Foo, String);
///
/// let mut foo = Foo("bar".to_owned());
/// assert_eq!(foo.len(), 3);
/// ```
///
/// [`Deref`]: std::ops::Deref
#[macro_export]
macro_rules! impl_deref {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }
    };
}

/// Implement [`DerefMut`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for. The type must also
/// implement [`Deref`] (se ).
///
/// # Examples
/// ```
/// use impl_more::{impl_deref, impl_deref_mut};
///
/// struct Foo(String);
///
/// impl_deref!(Foo, String);
/// impl_deref_mut!(Foo);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.push('!');
///
/// assert_eq!(*foo, "bar!");
/// ```
///
/// [`DerefMut`]: std::ops::DerefMut
/// [`Deref`]: std::ops::Deref
#[macro_export]
macro_rules! impl_deref_mut {
    ($ty:ty) => {
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $field:ident) => {
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_and_mut {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

/// Implement [`From`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// wrapped type.
///
/// # Examples
/// ```
/// use impl_more::impl_from;
/// struct Foo(String);
///
/// impl_from!(Foo, String);
/// let foo = Foo::from("bar".to_owned());
/// ```
#[macro_export]
macro_rules! impl_from {
    ($ty:ty, $inner:ty) => {
        impl ::core::convert::From<$inner> for $ty {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }
    };
}

/// Implement [`Into`] for a newtype struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// wrapped type.
///
/// # Examples
/// ```
/// use impl_more::impl_into;
/// struct Foo(String);
///
/// impl_into!(Foo, String);
///
/// let foo = Foo("bar".to_owned());
/// let foo_str: String = foo.into();
/// ```
#[macro_export]
macro_rules! impl_into {
    ($ty:ty, $inner:ty) => {
        impl ::core::convert::Into<$inner> for $ty {
            fn into(self) -> $inner {
                self.0
            }
        }
    };
}

/// Implement [`Display`] for enums using a static string or format args for each variant.
///
/// # Examples
/// ```
/// use impl_more::impl_display_enum;
///
/// enum Foo {
///     Bar,
///     Qux,
/// }
///
/// impl_display_enum!(Foo, Bar => "bar", Qux => "qux");
///
/// assert_eq!(Foo::Bar.to_string(), "bar");
/// assert_eq!(Foo::Qux.to_string(), "qux");
///
/// enum CoordOrMsg {
///     Coord(i64, i64),
///     Msg(&'static str),
/// }
///
/// impl_display_enum!(CoordOrMsg, Coord(x, y) => "{x}, {y}", Msg(msg) => "message: {msg}");
///
/// assert_eq!(CoordOrMsg::Coord(4, 2).to_string(), "4, 2");
/// assert_eq!(CoordOrMsg::Msg("hi").to_string(), "message: hi");
/// ```
///
/// [`Display`]: std::fmt::Display
#[macro_export]
macro_rules! impl_display_enum {
    ($ty:ty, $($variant:ident => $stringified:literal),+) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.write_str(match self {
                    $(
                        Self::$variant => $stringified,
                    )*
                })
            }
        }
    };

    ($ty:ty, $($variant:ident => $stringified:literal),+ ,) => {
        impl_display_enum!($ty, $($variant => $stringified),+)
    };

    ($ty:ty, $($variant:ident ($($inner:ident),+) => $format:literal),+) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::core::fmt::Write as _;

                // a more efficient method (format_args) is blocked by:
                // https://github.com/rust-lang/rust/issues/15023
                let mut buf = ::std::string::String::new();

                match self {
                    $(
                        Self::$variant($($inner),+) =>
                            ::core::write!(&mut buf, $format, $($inner = $inner),+)?,
                    )*
                };

                fmt.write_str(&buf)
            }
        }
    };

    ($ty:ty, $($variant:ident ($($inner:ident),+) => $format:literal),+ ,) => {
        impl_display_enum!($ty, $($variant ($($inner),+) => $format),+)
    };

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $format:literal),+) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::core::fmt::Write as _;

                // a more efficient method (format_args) is blocked by:
                // https://github.com/rust-lang/rust/issues/15023
                let mut buf = ::std::string::String::new();

                match self {
                    $(
                        Self::$variant { $($inner),+ } =>
                            ::core::write!(&mut buf, $format, $($inner = $inner),+)?,
                    )*
                };

                fmt.write_str(&buf)
            }
        }
    };

    // TODO: mixed named and positional variant support

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $format:literal),+ ,) => {
        impl_display_enum!($ty, $($variant ($($inner),+) => $format),+)
    };
}

#[cfg(test)]
mod tests {
    #![allow(clippy::from_over_into)]

    use alloc::string::{String, ToString as _};
    use core::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Foo(String);

    impl_as_ref!(Foo, String);
    impl_as_mut!(Foo, String);

    impl_deref!(Foo, String);
    impl_deref_mut!(Foo);

    impl_from!(Foo, String);
    impl_into!(Foo, String);

    static_assertions::assert_impl_all!(
        Foo:
        // impls
        AsRef<String>,
        AsMut<String>,
        Deref<Target = String>,
        DerefMut,
        From<String>,
        Into<String>,
    );

    #[derive(Debug)]
    struct Bar {
        foo: Foo,
    }

    impl_as_ref!(Bar, Foo, foo);
    impl_as_mut!(Bar, Foo, foo);

    impl_deref!(Bar, String, foo);
    impl_deref_mut!(Bar, foo);

    static_assertions::assert_impl_all!(
        Bar:
        // impls
        AsRef<Foo>,
        AsMut<Foo>,
        Deref<Target = String>,
        DerefMut,
    );

    #[allow(dead_code)]
    #[test]
    fn impl_display() {
        enum Foo {
            Bar,
            Qux,
        }
        impl_display_enum!(Foo, Bar => "bar", Qux => "qux");
        assert_eq!(Foo::Bar.to_string(), "bar");
        assert_eq!(Foo::Qux.to_string(), "qux");

        enum FooComma {
            Bar,
            Qux,
        }
        impl_display_enum!(FooComma, Bar => "bar", Qux => "qux",);

        enum FooContents {
            Bar(u64, u64),
        }
        impl_display_enum!(FooContents, Bar (x, y) => "x: {x}; y: {y}");
        assert_eq!(FooContents::Bar(4, 2).to_string(), "x: 4; y: 2");

        enum FooContents2 {
            Qux { msg: &'static str },
        }
        impl_display_enum!(FooContents2, Qux { msg } => "msg: {msg}");
        assert_eq!(FooContents2::Qux { msg: "foo" }.to_string(), "msg: foo");

        // enum FooContents3 {
        //     Bar(u64, u64),
        //     Qux { msg: &'static str },
        // }
        // impl_display_enum!(FooContents3, Bar (x, y) => "x: {x}; y: {y}", Qux { msg } => "{msg}");
        // assert_eq!(FooContents3::Bar(4, 2).to_string(), "x: 4; y: 2");
        // assert_eq!(FooContents3::Qux { msg: "foo" }.to_string(), "x: 4; y: 2");
    }
}
