/// Implement [`Debug`](core::fmt::Debug) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `Debug` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_debug!(Value);
///
/// assert_eq!(
///     format!("{:?}", Value(42)),
///     "42",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_debug!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:?}", Value(inner)),
///     "42",
/// );
/// ```
#[macro_export]
macro_rules! forward_debug {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::Debug),+> ::core::fmt::Debug for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::Debug),+> ::core::fmt::Debug for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::Debug for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::Debug for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`Binary`](core::fmt::Binary) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `Binary` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_binary!(Value);
///
/// assert_eq!(
///     format!("{:b}", Value(42)),
///     "101010",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_binary!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:b}", Value(inner)),
///     "101010",
/// );
/// ```
#[macro_export]
macro_rules! forward_binary {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::Binary),+> ::core::fmt::Binary for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::Binary),+> ::core::fmt::Binary for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::Binary for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::Binary for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`Octal`](core::fmt::Octal) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `Octal` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_octal!(Value);
///
/// assert_eq!(
///     format!("{:o}", Value(42)),
///     "52",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_octal!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:o}", Value(inner)),
///     "52",
/// );
/// ```
#[macro_export]
macro_rules! forward_octal {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::Octal),+> ::core::fmt::Octal for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::Octal),+> ::core::fmt::Octal for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::Octal for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::Octal for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`LowerHex`](core::fmt::LowerHex) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `LowerHex` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_lower_hex!(Value);
///
/// assert_eq!(
///     format!("{:x}", Value(42)),
///     "2a",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_lower_hex!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:x}", Value(inner)),
///     "2a",
/// );
/// ```
#[macro_export]
macro_rules! forward_lower_hex {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::LowerHex),+> ::core::fmt::LowerHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::LowerHex),+> ::core::fmt::LowerHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::LowerHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::LowerHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`UpperHex`](core::fmt::UpperHex) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `UpperHex` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_upper_hex!(Value);
///
/// assert_eq!(
///     format!("{:X}", Value(42)),
///     "2A",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_upper_hex!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:X}", Value(inner)),
///     "2A",
/// );
/// ```
#[macro_export]
macro_rules! forward_upper_hex {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::UpperHex),+> ::core::fmt::UpperHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::UpperHex),+> ::core::fmt::UpperHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::UpperHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::UpperHex for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`LowerExp`](core::fmt::LowerExp) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `LowerExp` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_lower_exp!(Value);
///
/// assert_eq!(
///     format!("{:e}", Value(42)),
///     "4.2e1",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_lower_exp!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:e}", Value(inner)),
///     "4.2e1",
/// );
/// ```
#[macro_export]
macro_rules! forward_lower_exp {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::LowerExp),+> ::core::fmt::LowerExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::LowerExp),+> ::core::fmt::LowerExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::LowerExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::LowerExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`UpperExp`](core::fmt::UpperExp) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `UpperExp` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(u32);
///
/// impl_more::forward_upper_exp!(Value);
///
/// assert_eq!(
///     format!("{:E}", Value(42)),
///     "4.2E1",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_upper_exp!(<T> in Value<T>);
///
/// let inner = 42u32;
///
/// assert_eq!(
///     format!("{:E}", Value(inner)),
///     "4.2E1",
/// );
/// ```
#[macro_export]
macro_rules! forward_upper_exp {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::UpperExp),+> ::core::fmt::UpperExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::UpperExp),+> ::core::fmt::UpperExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::UpperExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::UpperExp for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.0, fmt)
            }
        }
    };
}

/// Implement [`Pointer`](core::fmt::Pointer) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `Pointer` bound.
///
/// # Examples
///
/// With a newtype struct:
///
/// ```
/// struct Value(*const i32);
///
/// impl_more::forward_pointer!(Value);
///
/// assert_eq!(
///     format!("{:p}", Value(core::ptr::null())),
///     "0x0",
/// );
/// ```
///
/// With a generic newtype struct:
///
/// ```
/// struct Value<T>(T);
///
/// impl_more::forward_pointer!(<T> in Value<T>);
///
/// let inner = core::ptr::null::<i32>();
///
/// assert_eq!(
///     format!("{:p}", Value(inner)),
///     "0x0",
/// );
/// ```
#[macro_export]
macro_rules! forward_pointer {
    (<$($generic:ident),+> in $this:ty => $field:tt $(,)?) => {
        impl <$($generic: ::core::fmt::Pointer),+> ::core::fmt::Pointer for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Pointer::fmt(&self.$field, fmt)
            }
        }
    };
    (<$($generic:ident),+> in $this:ty $(,)?) => {
        impl <$($generic: ::core::fmt::Pointer),+> ::core::fmt::Pointer for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Pointer::fmt(&self.0, fmt)
            }
        }
    };
    ($this:ty => $field:tt $(,)?) => {
        impl ::core::fmt::Pointer for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Pointer::fmt(&self.$field, fmt)
            }
        }
    };
    ($this:ty $(,)?) => {
        impl ::core::fmt::Pointer for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Pointer::fmt(&self.0, fmt)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::String, vec};

    #[test]
    fn debug_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_debug!(Tuple,);
        crate::forward_debug!(Named => inner,);
        crate::forward_debug!(<T> in Generic<T>);
        crate::forward_debug!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012?}", inner);
        assert_eq!(format!("{:#012?}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012?}", Named { inner }), expected);
        assert_eq!(format!("{:#012?}", Generic(inner)), expected);
        assert_eq!(format!("{:#012?}", GenericNamed { inner }), expected);
    }

    #[test]
    fn binary_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_binary!(Tuple,);
        crate::forward_binary!(Named => inner,);
        crate::forward_binary!(<T> in Generic<T>);
        crate::forward_binary!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012b}", inner);
        assert_eq!(format!("{:#012b}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012b}", Named { inner }), expected);
        assert_eq!(format!("{:#012b}", Generic(inner)), expected);
        assert_eq!(format!("{:#012b}", GenericNamed { inner }), expected);
    }

    #[test]
    fn octal_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_octal!(Tuple,);
        crate::forward_octal!(Named => inner,);
        crate::forward_octal!(<T> in Generic<T>);
        crate::forward_octal!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012o}", inner);
        assert_eq!(format!("{:#012o}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012o}", Named { inner }), expected);
        assert_eq!(format!("{:#012o}", Generic(inner)), expected);
        assert_eq!(format!("{:#012o}", GenericNamed { inner }), expected);
    }

    #[test]
    fn lower_hex_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_lower_hex!(Tuple,);
        crate::forward_lower_hex!(Named => inner,);
        crate::forward_lower_hex!(<T> in Generic<T>);
        crate::forward_lower_hex!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012x}", inner);
        assert_eq!(format!("{:#012x}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012x}", Named { inner }), expected);
        assert_eq!(format!("{:#012x}", Generic(inner)), expected);
        assert_eq!(format!("{:#012x}", GenericNamed { inner }), expected);
    }

    #[test]
    fn upper_hex_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_upper_hex!(Tuple,);
        crate::forward_upper_hex!(Named => inner,);
        crate::forward_upper_hex!(<T> in Generic<T>);
        crate::forward_upper_hex!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012X}", inner);
        assert_eq!(format!("{:#012X}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012X}", Named { inner }), expected);
        assert_eq!(format!("{:#012X}", Generic(inner)), expected);
        assert_eq!(format!("{:#012X}", GenericNamed { inner }), expected);
    }

    #[test]
    fn lower_exp_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_lower_exp!(Tuple,);
        crate::forward_lower_exp!(Named => inner,);
        crate::forward_lower_exp!(<T> in Generic<T>);
        crate::forward_lower_exp!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012e}", inner);
        assert_eq!(format!("{:#012e}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012e}", Named { inner }), expected);
        assert_eq!(format!("{:#012e}", Generic(inner)), expected);
        assert_eq!(format!("{:#012e}", GenericNamed { inner }), expected);
    }

    #[test]
    fn upper_exp_forwards_formatter_and_fields() {
        struct Tuple(u32);
        struct Named {
            inner: u32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_upper_exp!(Tuple,);
        crate::forward_upper_exp!(Named => inner,);
        crate::forward_upper_exp!(<T> in Generic<T>);
        crate::forward_upper_exp!(<T> in GenericNamed<T> => inner);
        let inner = 42u32;
        let expected = format!("{:#012E}", inner);
        assert_eq!(format!("{:#012E}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012E}", Named { inner }), expected);
        assert_eq!(format!("{:#012E}", Generic(inner)), expected);
        assert_eq!(format!("{:#012E}", GenericNamed { inner }), expected);
    }

    #[test]
    fn pointer_forwards_formatter_and_fields() {
        struct Tuple(*const i32);
        struct Named {
            inner: *const i32,
        }
        struct Generic<T>(T);
        struct GenericNamed<T> {
            inner: T,
        }
        crate::forward_pointer!(Tuple,);
        crate::forward_pointer!(Named => inner,);
        crate::forward_pointer!(<T> in Generic<T>);
        crate::forward_pointer!(<T> in GenericNamed<T> => inner);
        let inner = &42 as *const i32;
        let expected = format!("{:#012p}", inner);
        assert_eq!(format!("{:#012p}", Tuple(inner)), expected);
        assert_eq!(format!("{:#012p}", Named { inner }), expected);
        assert_eq!(format!("{:#012p}", Generic(inner)), expected);
        assert_eq!(format!("{:#012p}", GenericNamed { inner }), expected);
    }

    #[test]
    fn debug_preserves_pretty_layout_and_precision() {
        struct Value<T>(T);
        crate::forward_debug!(<T> in Value<T>);
        assert_eq!(
            format!("{:#?}", Value(vec![1, 2])),
            format!("{:#?}", vec![1, 2])
        );
        assert_eq!(
            format!("{:>12.3?}", Value(1.23456)),
            format!("{:>12.3?}", 1.23456)
        );
    }

    #[test]
    fn forwards_format_errors() {
        struct Fails;
        impl core::fmt::LowerHex for Fails {
            fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                Err(core::fmt::Error)
            }
        }
        struct Value(Fails);
        crate::forward_lower_hex!(Value);
        let mut output = String::new();
        assert!(core::fmt::write(&mut output, format_args!("{:x}", Value(Fails))).is_err());
    }
}
