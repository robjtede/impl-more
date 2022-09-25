//! Concise trait implementations.

#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(future_incompatible)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Implement `AsRef` for a newtype struct.
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

/// Implement `AsMut` for a newtype struct.
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

/// Implement `Deref` for a newtype struct.
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
/// implement [`Deref`]
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

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Foo(String);

    impl_as_ref!(Foo, String);
    impl_as_mut!(Foo, String);
    impl_deref!(Foo, String);
    impl_deref_mut!(Foo);

    static_assertions::assert_impl_all!(
        Foo: AsRef<String>,
        AsMut<String>,
        Deref<Target = String>,
        DerefMut,
    );
}
