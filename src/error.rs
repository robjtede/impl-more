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
    ($ty:ty, $($variant:ident ($($inner:ident),+) => $source:expr),+ ,) => {
        impl ::std::error::Error for $ty {
            fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)> {
                match self {
                    $(
                        Self::$variant($($inner),+) => ::core::option::Option::Some($source),
                    )*
                    _ => ::core::option::Option::None,
                }
            }
        }
    };

    ($ty:ty, $($variant:ident => $stringified:literal),+) => {
        impl_display_enum!($ty, $($variant => $stringified),+ ,)
    };

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $source:expr),+ ,) => {
        impl ::std::error::Error for $ty {
            fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)> {
                match self {
                    $(
                        Self::$variant($($inner),+) => ::core::option::Option::Some($source),
                    )*
                    _ => ::core::option::Option::None,
                }
            }
        }
    };

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $format:literal),+) => {
        impl_display_enum!($ty, $($variant ($($inner),+) => $format),+ ,)
    };

    ($ty:ty,) => {
        impl ::std::error::Error for $ty {}
    };

    ($ty:ty) => {
        impl_error_enum!($ty,)
    };
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    #[test]
    fn with_trailing_comma() {
        #[derive(Debug)]
        enum Foo {
            Bar,
        }

        impl_display_enum!(Foo, Bar => "bar");
        impl_error_enum!(Foo,);
    }

    #[test]
    fn no_inner_data() {
        #[derive(Debug)]
        enum Foo {
            Bar,
            Baz,
        }

        impl_display_enum!(Foo, Bar => "bar", Baz => "qux");
        impl_error_enum!(Foo);

        assert!(Foo::Bar.source().is_none());
        assert!(Foo::Baz.source().is_none());
    }

    #[test]
    fn uniform_enum() {
        #[derive(Debug)]
        enum Foo {
            Bar(String),
            Baz(std::io::Error),
            Qux(String, std::io::Error),
        }

        impl_display_enum!(
            Foo,
            Bar(desc) => "{desc}",
            Baz(err) => "{err}",
            Qux(desc, err) => "{desc}: {err}"
        );
        impl_error_enum!(Foo, Baz(err) => err, Qux(_desc, err) => err);

        assert!(Foo::Bar(String::new()).source().is_none());

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert!(Foo::Baz(io_err).source().is_some());

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert!(Foo::Qux(String::new(), io_err).source().is_some());
    }
}
