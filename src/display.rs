/// Implements [`Display`] for structs by forwarding to one of its field.
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
/// };
///
/// impl_more::forward_display!(Bar => inner);
///
/// assert_eq!(Bar { inner: 42 }.to_string(), "42");
/// ```
///
/// [`Display`]: std::fmt::Display
#[macro_export]
macro_rules! forward_display {
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
