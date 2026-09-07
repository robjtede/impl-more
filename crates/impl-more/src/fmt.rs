/// Implement [`Debug`](core::fmt::Debug) by forwarding to a field.
///
/// The formatter is passed through unchanged. Emitted code supports `no_std`.
/// Omit the field for a tuple newtype. All type parameters receive a `Debug` bound.
///
/// # Examples
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
