/// Implement [`AsRef`] for a struct.
///
/// The first argument is that of the struct to create the impl for and the second is the type to
/// produce a reference for.
///
/// # Examples
/// With a newtype struct:
/// ```
/// use impl_more::impl_as_ref;
///
/// struct Foo(String);
/// impl_as_ref!(Foo => String);
/// let foo = Foo("bar".to_owned());
/// assert_eq!(foo.as_ref(), "bar");
/// ```
///
/// With a named field struct and type parameters:
/// ```
/// use impl_more::impl_as_ref;
///
/// struct Foo<T> { inner: T }
/// impl_as_ref!(Foo<T> => inner: T);
/// let foo = Foo { inner: "bar".to_owned() };
/// assert_eq!(foo.as_ref().as_str(), "bar");
/// ```
#[macro_export]
macro_rules! impl_as_ref {
    ($this:ident $(<$($generic:ident),+>)? => $inner:ty) => {
        impl $(<$($generic),+>)? ::core::convert::AsRef<$inner> for $this $(<$($generic),+>)? {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }
    };

    ($this:ident $(<$($generic:ident),+>)? => $field:ident : $inner:ty) => {
        impl $(<$($generic),+>)? ::core::convert::AsRef<$inner> for $this $(<$($generic),+>)? {
            fn as_ref(&self) -> &$inner {
                &self.$field
            }
        }
    };
}

/// Implement [`AsRef`] by forwarding to a field's implementation.
///
/// The first argument is the struct to create the impl for and the second is the target type
/// produced by the field's [`AsRef`] implementation.
///
/// # Examples
/// With a newtype struct:
/// ```
/// use impl_more::forward_as_ref;
///
/// struct Foo(String);
/// forward_as_ref!(Foo => str);
///
/// let foo = Foo("bar".to_owned());
/// assert_eq!(foo.as_ref(), "bar");
/// ```
///
/// With a named field struct and type parameters:
/// ```
/// use impl_more::forward_as_ref;
///
/// struct Foo<T> { inner: Vec<T> }
/// forward_as_ref!(<T> in Foo<T> => inner: [T]);
///
/// let foo = Foo { inner: vec![1, 2, 3] };
/// assert_eq!(foo.as_ref(), &[1, 2, 3]);
/// ```
#[macro_export]
macro_rules! forward_as_ref {
    (<$($generic:ident),+> in $this:ty => $target:ty) => {
        impl <$($generic),+> ::core::convert::AsRef<$target> for $this {
            fn as_ref(&self) -> &$target {
                ::core::convert::AsRef::<$target>::as_ref(&self.0)
            }
        }
    };

    (<$($generic:ident),+> in $this:ty => $field:ident : $target:ty) => {
        impl <$($generic),+> ::core::convert::AsRef<$target> for $this {
            fn as_ref(&self) -> &$target {
                ::core::convert::AsRef::<$target>::as_ref(&self.$field)
            }
        }
    };

    ($this:ty => $target:ty) => {
        impl ::core::convert::AsRef<$target> for $this {
            fn as_ref(&self) -> &$target {
                ::core::convert::AsRef::<$target>::as_ref(&self.0)
            }
        }
    };

    ($this:ty => $field:ident : $target:ty) => {
        impl ::core::convert::AsRef<$target> for $this {
            fn as_ref(&self) -> &$target {
                ::core::convert::AsRef::<$target>::as_ref(&self.$field)
            }
        }
    };
}

/// Implement [`AsMut`] for a struct.
///
/// The first argument is that of the struct to create the impl for and the second is the type to
/// produce a reference for.
///
/// # Examples
/// With a newtype struct:
/// ```
/// use impl_more::{impl_as_ref, impl_as_mut};
///
/// struct Foo(String);
///
/// impl_as_ref!(Foo => String);
/// impl_as_mut!(Foo => String);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.as_mut().push('!');
///
/// assert_eq!(foo.as_ref(), "bar!");
/// ```
///
/// With a named field struct and type parameters:
/// ```
/// use impl_more::{impl_as_ref, impl_as_mut};
///
/// struct Foo<T> { inner: T }
///
/// impl_as_ref!(Foo<T> => inner: T);
/// impl_as_mut!(Foo<T> => inner: T);
///
/// let mut foo = Foo { inner: "bar".to_owned() };
/// foo.as_mut().push('!');
///
/// assert_eq!(foo.as_ref(), "bar!");
/// ```
#[macro_export]
macro_rules! impl_as_mut {
    ($this:ident $(<$($generic:ident),+>)? => $inner:ty) => {
        impl $(<$($generic),+>)? ::core::convert::AsMut<$inner> for $this $(<$($generic),+>)? {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };

    ($this:ident $(<$($generic:ident),+>)? => $field:ident : $inner:ty) => {
        impl $(<$($generic),+>)? ::core::convert::AsMut<$inner> for $this $(<$($generic),+>)? {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.$field
            }
        }
    };
}

/// Implement [`AsMut`] by forwarding to a field's implementation.
///
/// The first argument is the struct to create the impl for and the second is the target type
/// produced by the field's [`AsMut`] implementation.
///
/// # Examples
/// With a newtype struct:
/// ```
/// use impl_more::forward_as_mut;
///
/// struct Foo(String);
/// forward_as_mut!(Foo => str);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.as_mut().make_ascii_uppercase();
///
/// assert_eq!(foo.0, "BAR");
/// ```
///
/// With a named field struct and type parameters:
/// ```
/// use impl_more::forward_as_mut;
///
/// struct Foo<T> { inner: Vec<T> }
/// forward_as_mut!(<T> in Foo<T> => inner: [T]);
///
/// let mut foo = Foo { inner: vec![1, 2, 3] };
/// foo.as_mut().reverse();
///
/// assert_eq!(foo.inner, [3, 2, 1]);
/// ```
#[macro_export]
macro_rules! forward_as_mut {
    (<$($generic:ident),+> in $this:ty => $target:ty) => {
        impl <$($generic),+> ::core::convert::AsMut<$target> for $this {
            fn as_mut(&mut self) -> &mut $target {
                ::core::convert::AsMut::<$target>::as_mut(&mut self.0)
            }
        }
    };

    (<$($generic:ident),+> in $this:ty => $field:ident : $target:ty) => {
        impl <$($generic),+> ::core::convert::AsMut<$target> for $this {
            fn as_mut(&mut self) -> &mut $target {
                ::core::convert::AsMut::<$target>::as_mut(&mut self.$field)
            }
        }
    };

    ($this:ty => $target:ty) => {
        impl ::core::convert::AsMut<$target> for $this {
            fn as_mut(&mut self) -> &mut $target {
                ::core::convert::AsMut::<$target>::as_mut(&mut self.0)
            }
        }
    };

    ($this:ty => $field:ident : $target:ty) => {
        impl ::core::convert::AsMut<$target> for $this {
            fn as_mut(&mut self) -> &mut $target {
                ::core::convert::AsMut::<$target>::as_mut(&mut self.$field)
            }
        }
    };
}

/// Implement [`AsRef`] and [`AsMut`] by forwarding to a field's implementations.
///
/// This macro has the same type parameter support and format as [`forward_as_ref`].
///
/// # Examples
/// ```
/// use impl_more::forward_as_ref_and_mut;
///
/// struct Foo(String);
/// forward_as_ref_and_mut!(Foo => str);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.as_mut().make_ascii_uppercase();
///
/// assert_eq!(foo.as_ref(), "BAR");
/// ```
///
/// [`forward_as_ref`]: crate::forward_as_ref
#[macro_export]
macro_rules! forward_as_ref_and_mut {
    (<$($generic:ident),+> in $this:ty => $target:ty) => {
        $crate::forward_as_ref!(<$($generic),+> in $this => $target);
        $crate::forward_as_mut!(<$($generic),+> in $this => $target);
    };

    (<$($generic:ident),+> in $this:ty => $field:ident : $target:ty) => {
        $crate::forward_as_ref!(<$($generic),+> in $this => $field: $target);
        $crate::forward_as_mut!(<$($generic),+> in $this => $field: $target);
    };

    ($this:ty => $target:ty) => {
        $crate::forward_as_ref!($this => $target);
        $crate::forward_as_mut!($this => $target);
    };

    ($this:ty => $field:ident : $target:ty) => {
        $crate::forward_as_ref!($this => $field: $target);
        $crate::forward_as_mut!($this => $field: $target);
    };
}

#[cfg(test)]
mod tests {
    use alloc::{string::String, vec, vec::Vec};

    struct Newtype(String);
    forward_as_ref!(Newtype => str);
    forward_as_mut!(Newtype => str);

    struct Generic<T>(Vec<T>);
    forward_as_ref!(<T> in Generic<T> => [T]);
    forward_as_mut!(<T> in Generic<T> => [T]);

    struct GenericNamed<T> {
        inner: Vec<T>,
    }
    forward_as_ref_and_mut!(<T> in GenericNamed<T> => inner: [T]);

    static_assertions::assert_impl_all!(Newtype: AsRef<str>, AsMut<str>);
    static_assertions::assert_impl_all!(Generic<usize>: AsRef<[usize]>, AsMut<[usize]>);
    static_assertions::assert_impl_all!(GenericNamed<usize>: AsRef<[usize]>, AsMut<[usize]>);

    #[test]
    fn forwards_newtype() {
        let mut value = Newtype("hello".into());
        AsMut::<str>::as_mut(&mut value).make_ascii_uppercase();

        assert_eq!(AsRef::<str>::as_ref(&value), "HELLO");
    }

    #[test]
    fn forwards_generic_newtype() {
        let mut value = Generic(vec![1, 2, 3]);
        AsMut::<[usize]>::as_mut(&mut value).reverse();

        assert_eq!(AsRef::<[usize]>::as_ref(&value), &[3, 2, 1]);
    }

    #[test]
    fn forwards_generic_named_field() {
        let mut value = GenericNamed {
            inner: vec![1, 2, 3],
        };
        AsMut::<[usize]>::as_mut(&mut value).reverse();

        assert_eq!(AsRef::<[usize]>::as_ref(&value), &[3, 2, 1]);
    }
}
