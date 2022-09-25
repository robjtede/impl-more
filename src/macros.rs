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
        impl ::std::convert::AsRef<$inner> for $ty {
            fn as_ref(&self) -> &$inner {
                &self.0
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
        impl ::std::convert::AsMut<$inner> for $ty {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
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
    ($ty:ty, $inner:ty) => {
        impl ::std::ops::Deref for $ty {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
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
        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
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
        impl ::std::convert::From<$inner> for $ty {
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
        impl ::std::convert::Into<$inner> for $ty {
            fn into(self) -> $inner {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_enum {
    ($ty:ty, $($variant:ident => $stringified:literal),+) => {
        impl ::std::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.write_str(match self {
                    $(
                        Self::$variant => $stringified,
                    )*
                })
            }
        }
    };

    ($ty:ty, $($variant:ident => $stringified:literal),+,) => {
        impl_display_enum!($ty, $($variant => $stringified),+)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::from_over_into)]

    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Foo(String);

    impl_as_ref!(Foo, String);
    impl_as_mut!(Foo, String);

    impl_deref!(Foo, String);
    impl_deref_mut!(Foo);

    impl_from!(Foo, String);
    impl_into!(Foo, String);

    static_assertions::assert_impl_all!(
        Foo: AsRef<String>,
        AsMut<String>,
        Deref<Target = String>,
        DerefMut,
        From<String>,
        Into<String>,
    );

    #[test]
    fn impl_display() {
        #[derive(Debug)]
        enum Foo {
            Bar,
            Qux,
        }

        impl_display_enum!(Foo, Bar => "bar", Qux => "qux");

        assert_eq!(Foo::Bar.to_string(), "bar");
        assert_eq!(Foo::Qux.to_string(), "qux");

        #[derive(Debug)]
        enum FooComma {
            Bar,
            Qux,
        }

        impl_display_enum!(FooComma, Bar => "bar", Qux => "qux",);
    }
}
