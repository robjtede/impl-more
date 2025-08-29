/// Implements [`Error`] for structs and forwards the `source` implementation to one of its fields.
///
/// Emitted code is not compatible with `#[no_std]`.
///
/// Newtype structs can omit the field identifier.
///
/// # Examples
///
/// For newtype struct:
///
/// ```ignore
/// use std::error::Error as _;
///
/// #[derive(Debug)]
/// struct MyError(eyre::Report);
///
/// impl_more::forward_display!(MyError);
/// impl_more::forward_error!(MyError);
///
/// let err = MyError(eyre::eyre!("something went wrong"));
/// assert_eq!(err.source().unwrap().to_string(), "something went wrong");
/// ```
///
/// For struct with named field:
///
/// ```ignore
/// use std::error::Error as _;
///
/// #[derive(Debug)]
/// struct MyError {
///     cause: eyre::Report,
/// }
///
/// impl_more::forward_display!(MyError => cause);
/// impl_more::forward_error!(MyError => cause);
///
/// let err = MyError { cause: eyre::eyre!("something went wrong") };
/// assert_eq!(err.source().unwrap().to_string(), "something went wrong");
/// ```
///
/// This macro does not yet support use with generic error wrappers.
///
/// [`Error`]: core::error::Error
#[macro_export]
macro_rules! forward_error {
    ($ty:ty) => {
        impl ::core::error::Error for $ty {
            fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
                Some(::core::ops::Deref::deref(&self.0))
            }
        }
    };

    ($ty:ty => $field:ident) => {
        impl ::core::error::Error for $ty {
            fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
                Some(::core::ops::Deref::deref(&self.$field))
            }
        }
    };
}

/// Implements [`Error`] for enums.
///
/// Emitted code is compatible with `#[no_std]` after Rust v1.81.
///
/// # Examples
///
/// ```ignore
/// # extern crate alloc;
/// use core::error::Error as _;
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
/// [`Error`]: core::error::Error
#[macro_export]
macro_rules! impl_error_enum {
    ($ty:ty: $($variant:ident ($($inner:ident),+) => $source:expr),+ ,) => {
        impl ::core::error::Error for $ty {
            fn source(&self) -> ::core::option::Option<&(dyn ::core::error::Error + 'static)> {
                match self {
                    $(
                        Self::$variant($($inner),+) => ::core::option::Option::Some($source),
                    )*
                    _ => ::core::option::Option::None,
                }
            }
        }
    };

    ($ty:ty: $($variant:ident ($($inner:ident),+) => $source:expr),+) => {
        $crate::impl_error_enum!($ty: $($variant ($($inner),+) => $source),+ ,);
    };

    ($ty:ty: $($variant:ident { $($inner:ident),+ } => $source:expr),+ ,) => {
        impl ::core::error::Error for $ty {
            fn source(&self) -> ::core::option::Option<&(dyn ::core::error::Error + 'static)> {
                match self {
                    $(
                        Self::$variant($($inner),+) => ::core::option::Option::Some($source),
                    )*
                    _ => ::core::option::Option::None,
                }
            }
        }
    };

    ($ty:ty: $($variant:ident { $($inner:ident),+ } => $source:expr),+) => {
        $crate::impl_error_enum!($ty, $($variant { $($inner),+ } => $source),+ ,);
    };

    ($ty:ty) => {
        impl ::core::error::Error for $ty {}
    };
}

/// Implements leaf [`Error`]s.
///
/// Emitted code is compatible with `#[no_std]` after Rust v1.81.
///
/// # Examples
///
/// ```ignore
/// #[derive(Debug)]
/// struct LeafError;
///
/// impl_more::impl_display!(LeafError; "leaf");
/// impl_more::impl_error_enum!(LeafError);
/// ```
///
/// [`Error`]: core::error::Error
#[macro_export]
macro_rules! impl_leaf_error {
    ($ty:ty) => {
        impl ::core::error::Error for $ty {}
    };
}

#[cfg(test)]
#[rustversion::since(1.81)]
mod tests {
    use alloc::string::String;
    use core::error::Error as _;

    #[test]
    fn with_trailing_comma() {
        #![allow(unused)]

        #[derive(Debug)]
        enum Foo {
            Bar,
        }

        impl_display_enum!(Foo: Bar => "bar");
        impl_leaf_error!(Foo);
    }

    #[test]
    fn no_inner_data() {
        #[derive(Debug)]
        enum Foo {
            Bar,
            Baz,
        }

        impl_display_enum!(Foo: Bar => "bar", Baz => "qux");
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

        impl_display_enum! {
            Foo:
            Bar(desc) => "{desc}",
            Baz(err) => "{err}",
            Qux(desc, err) => "{desc}: {err}"
        };
        impl_error_enum! {
            Foo:
            Baz(err) => err,
            Qux(_desc, err) => err
        };

        assert!(Foo::Bar(String::new()).source().is_none());

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert!(Foo::Baz(io_err).source().is_some());

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert!(Foo::Qux(String::new(), io_err).source().is_some());
    }
}
