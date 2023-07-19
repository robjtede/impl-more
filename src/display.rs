/// Implements [`Display`] for structs by forwarding to one of its field.
///
/// Emitted code is not compatible with `#[no_std]`.
///
/// Newtype structs can omit the field identifier.
///
/// # Examples
///
/// For newtype struct:
///
/// ```
/// # use impl_more::forward_display;
/// struct Foo(String);
///
/// impl_more::forward_display!(Foo);
///
/// assert_eq!(Foo("hello world".to_owned()).to_string(), "hello world");
/// ```
///
/// For struct with named field:
///
/// ```
/// # use impl_more::forward_display;
/// struct Bar {
///     inner: u64,
/// }
///
/// impl_more::forward_display!(Bar => inner);
///
/// assert_eq!(Bar { inner: 42 }.to_string(), "42");
/// ```
///
/// For generic newtype struct (note that `Display` bounds are applied to all type parameters):
///
/// ```
/// # use impl_more::forward_display;
/// struct Baz<T>(T);
///
/// impl_more::forward_display!(<T> in Baz<T>);
///
/// assert_eq!(Baz(42u64).to_string(), "42");
/// ```
///
/// [`Display`]: std::fmt::Display
#[macro_export]
macro_rules! forward_display {
    (<$($generic:ident),+> in $this:ty => $field:ident) => {
        impl <$($generic: ::core::fmt::Display),+> ::core::fmt::Display for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.$field, fmt)
            }
        }
    };

    (<$($generic:ident),+> in $this:ty) => {
        impl <$($generic: ::core::fmt::Display),+> ::core::fmt::Display for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, fmt)
            }
        }
    };

    ($ty:ty) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, fmt)
            }
        }
    };

    ($ty:ty => $field:ident) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.$field, fmt)
            }
        }
    };
}

/// Implements [`Display`] for enums using a static string or format args for each variant.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
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
        $crate::impl_display_enum!($ty, $($variant => $stringified),+);
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
        $crate::impl_display_enum!($ty, $($variant ($($inner),+) => $format),+);
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

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $format:literal),+ ,) => {
        $crate::impl_display_enum!($ty, $($variant ($($inner),+) => $format),+);
    };

    // TODO: mixed named and positional variant support
}

#[cfg(test)]
mod tests {
    use alloc::{
        borrow::ToOwned as _,
        string::{String, ToString as _},
    };

    #[test]
    fn impl_for_newtype_struct() {
        struct Foo(String);

        forward_display!(Foo);

        assert_eq!(Foo("hello world".to_owned()).to_string(), "hello world");
    }

    #[test]
    fn impl_for_newtype_named_struct() {
        struct Foo {
            inner: u64,
        }

        forward_display!(Foo => inner);

        assert_eq!(Foo { inner: 42 }.to_string(), "42");
    }

    #[test]
    fn impl_for_generic_newtype_struct() {
        struct Foo<T>(T);

        forward_display!(<T> in Foo<T>);

        assert_eq!(Foo(42).to_string(), "42");
    }

    #[test]
    fn impl_for_generic_named_struct() {
        struct Foo<T> {
            inner: T,
        }

        forward_display!(<T> in Foo<T> => inner);

        assert_eq!(Foo { inner: 42 }.to_string(), "42");
    }
}
