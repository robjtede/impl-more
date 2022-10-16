/// Implement [`Deref`] for a struct.
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
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }
    };
}

/// Implement [`DerefMut`] for a struct.
///
/// The first argument is that of the newtype struct to create the impl for. The type must also
/// implement [`Deref`].
///
/// Also see [`impl_deref`] and [`impl_deref_and_mut`].
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
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $field:ident) => {
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

// TODO: docs
#[macro_export]
macro_rules! impl_deref_and_mut {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

// TODO: docs
#[macro_export]
macro_rules! forward_deref_and_mut {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }
    };

    ($ty:ty, ref $target:ty) => {
        impl<'__impl_more_a> ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl<'__impl_more_a> ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.$field)
            }
        }
    };

    ($ty:ty, ref $target:ty, $field:ident) => {
        impl<'__impl_more_a> ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }

        impl<'__impl_more_a> ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.$field)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    fn accepts_string_slice(_: &str) {}
    fn accepts_mut_string_slice(_: &mut str) {}

    struct Foo1(String);
    impl_deref_and_mut!(Foo1, String);
    static_assertions::assert_impl_all!(
        Foo1:
        // impls
        Deref<Target = String>,
        DerefMut<Target = String>,
    );

    struct Foo2(String);
    forward_deref_and_mut!(Foo2, ref str);
    static_assertions::assert_impl_all!(
        Foo2:
        // impls
        Deref,
        DerefMut,
    );

    #[test]
    fn foo2_impls() {
        let mut foo = Foo2("".to_owned());
        accepts_string_slice(&foo);
        accepts_mut_string_slice(&mut foo);
    }

    struct Foo3 {
        msg: String,
    }
    forward_deref_and_mut!(Foo3, ref str, msg);
    static_assertions::assert_impl_all!(
        Foo3:
        // impls
        Deref,
        DerefMut,
    );

    #[test]
    fn foo3_impls() {
        let mut foo = Foo3 { msg: "".to_owned() };
        accepts_string_slice(&foo);
        accepts_mut_string_slice(&mut foo);
    }
}
