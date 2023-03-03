/// Implements [`Error`] for enums.
///
/// Emitted code is not compatible with `#[no_std]`.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// use std::error::Error as _;
///
/// #[derive(Debug)]
/// enum Err {
///     Io(std::io::Error),
///     Generic(String),
/// }
///
/// impl_more::impl_display_enum!(Err, Io(err) => "{err}", Generic(msg) => "{msg}");
/// impl_more::impl_error_enum!(Err, Io(err) => err);
///
/// # let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
/// assert!(Err::Io(io_err).source().is_some());
/// assert!(Err::Generic("oops".to_owned()).source().is_none());
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

    ($ty:ty, $($variant:ident ($($inner:ident),+) => $source:expr),+) => {
        $crate::impl_error_enum!($ty, $($variant ($($inner),+) => $source),+ ,);
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

    ($ty:ty, $($variant:ident { $($inner:ident),+ } => $source:expr),+) => {
        $crate::impl_error_enum!($ty, $($variant { $($inner),+ } => $source),+ ,);
    };

    ($ty:ty,) => {
        impl ::std::error::Error for $ty {}
    };

    ($ty:ty) => {
        $crate::impl_error_enum!($ty,);
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use std::error::Error as _;

    #[test]
    fn with_trailing_comma() {
        #![allow(unused)]

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
