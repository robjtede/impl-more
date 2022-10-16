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
