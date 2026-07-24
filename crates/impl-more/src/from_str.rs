/// Implement [`FromStr`] by forwarding to a field's implementation.
///
/// The first argument is the struct to create the impl for and the second is the field type whose
/// [`FromStr`] implementation is used.
///
/// # Examples
/// With a newtype struct:
/// ```
/// use impl_more::forward_from_str;
///
/// #[derive(Debug, PartialEq)]
/// struct Port(u16);
/// forward_from_str!(Port => u16);
///
/// assert_eq!("8080".parse(), Ok(Port(8080)));
/// ```
///
/// With a named field struct and type parameters:
/// ```
/// use impl_more::forward_from_str;
///
/// #[derive(Debug, PartialEq)]
/// struct Value<T> { inner: T }
/// forward_from_str!(<T> in Value<T> => inner: T);
///
/// assert_eq!("true".parse(), Ok(Value { inner: true }));
/// ```
///
/// [`FromStr`]: core::str::FromStr
#[macro_export]
macro_rules! forward_from_str {
    (<$($generic:ident),+> in $this:ty => $inner:ty $(,)?) => {
        impl <$($generic),+> ::core::str::FromStr for $this
        where
            $inner: ::core::str::FromStr,
        {
            type Err = <$inner as ::core::str::FromStr>::Err;

            fn from_str(value: &str) -> ::core::result::Result<Self, Self::Err> {
                <$inner as ::core::str::FromStr>::from_str(value).map(Self)
            }
        }
    };

    (<$($generic:ident),+> in $this:ty => $field:ident : $inner:ty $(,)?) => {
        impl <$($generic),+> ::core::str::FromStr for $this
        where
            $inner: ::core::str::FromStr,
        {
            type Err = <$inner as ::core::str::FromStr>::Err;

            fn from_str(value: &str) -> ::core::result::Result<Self, Self::Err> {
                <$inner as ::core::str::FromStr>::from_str(value)
                    .map(|$field| Self { $field })
            }
        }
    };

    ($this:ty => $inner:ty $(,)?) => {
        impl ::core::str::FromStr for $this {
            type Err = <$inner as ::core::str::FromStr>::Err;

            fn from_str(value: &str) -> ::core::result::Result<Self, Self::Err> {
                <$inner as ::core::str::FromStr>::from_str(value).map(Self)
            }
        }
    };

    ($this:ty => $field:ident : $inner:ty $(,)?) => {
        impl ::core::str::FromStr for $this {
            type Err = <$inner as ::core::str::FromStr>::Err;

            fn from_str(value: &str) -> ::core::result::Result<Self, Self::Err> {
                <$inner as ::core::str::FromStr>::from_str(value)
                    .map(|$field| Self { $field })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use core::{num::ParseIntError, str::FromStr as _};

    #[derive(Debug, PartialEq)]
    struct Newtype(u16);
    forward_from_str!(Newtype => u16);

    #[derive(Debug, PartialEq)]
    struct Named {
        inner: bool,
    }
    forward_from_str!(Named => inner: bool);

    #[derive(Debug, PartialEq)]
    struct Generic<T>(T);
    forward_from_str!(<T> in Generic<T> => T);

    #[derive(Debug, PartialEq)]
    struct GenericNamed<T> {
        inner: T,
    }
    forward_from_str!(<T> in GenericNamed<T> => inner: T);

    static_assertions::assert_impl_all!(Newtype: core::str::FromStr<Err = ParseIntError>);
    static_assertions::assert_impl_all!(Named: core::str::FromStr<Err = core::str::ParseBoolError>);
    static_assertions::assert_impl_all!(Generic<u16>: core::str::FromStr<Err = ParseIntError>);
    static_assertions::assert_impl_all!(
        GenericNamed<bool>: core::str::FromStr<Err = core::str::ParseBoolError>
    );

    #[test]
    fn forwards_newtype() {
        assert_eq!(Newtype::from_str("8080"), Ok(Newtype(8080)));
        assert!(Newtype::from_str("invalid").is_err());
    }

    #[test]
    fn forwards_named_field() {
        assert_eq!(Named::from_str("true"), Ok(Named { inner: true }));
        assert!(Named::from_str("invalid").is_err());
    }

    #[test]
    fn forwards_generic_newtype() {
        assert_eq!(Generic::<u16>::from_str("8080"), Ok(Generic(8080)));
    }

    #[test]
    fn forwards_generic_named_field() {
        assert_eq!(
            GenericNamed::<bool>::from_str("true"),
            Ok(GenericNamed { inner: true })
        );
    }
}
