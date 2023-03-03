/// Implements [`Error`] for enums.
///
/// # Examples
///
/// ```
/// #[derive(Debug)]
/// enum Foo {
///     Bar,
///     Qux(std::io::Error),
/// }
///
/// impl_more::impl_display_enum!(Foo, Bar => "bar", Qux => "qux");
/// impl_more::impl_error_enum!(Foo, Qux(err) => err);
///
/// # let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
/// assert_eq!(Foo::Bar.source().is_none());
/// assert_eq!(Foo::Qux(io_err).source().is_some());
/// ```
///
/// [`Error`]: std::error::Error
#[macro_export]
macro_rules! impl_error_enum {
    ($ty:ty, $($variant:ident ($($inner:ident),+) => $format:literal),+ ,?) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$variant($($inner),+) =>
                            ::core::write!(&mut buf, $format, $($inner = $inner),+)?,
                    )*
                };
            }
        }
    };

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $format:literal),+ ,?) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        #[derive(Debug)]
        enum Foo {
            Bar,
            Qux(std::io::Error),
        }

        impl_display_enum!(Foo, Bar => "bar", Qux => "qux");
        impl_error_enum!(Foo, Qux(err) => err);

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert_eq!(Foo::Bar.source().is_none());
        assert_eq!(Foo::Qux(io_err).source().is_some());
    }
}
