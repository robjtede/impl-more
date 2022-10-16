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
