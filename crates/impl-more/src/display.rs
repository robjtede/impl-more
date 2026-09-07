/// Implements [`Display`] for structs by forwarding to one of its field.
///
/// Emitted code is not compatible with `#[no_std]`.
///
/// Newtype structs can omit the field identifier.
///
/// # Examples
///
/// For newtype struct:
///
/// ```
/// # use impl_more::forward_display;
/// struct Foo(String);
///
/// impl_more::forward_display!(Foo);
///
/// assert_eq!(Foo("hello world".to_owned()).to_string(), "hello world");
/// ```
///
/// For struct with named field:
///
/// ```
/// # use impl_more::forward_display;
/// struct Bar {
///     inner: u64,
/// }
///
/// impl_more::forward_display!(Bar => inner);
///
/// assert_eq!(Bar { inner: 42 }.to_string(), "42");
/// ```
///
/// For generic newtype struct (note that `Display` bounds are applied to all type parameters):
///
/// ```
/// # use impl_more::forward_display;
/// struct Baz<T>(T);
///
/// impl_more::forward_display!(<T> in Baz<T>);
///
/// assert_eq!(Baz(42u64).to_string(), "42");
/// ```
///
/// [`Display`]: core::fmt::Display
#[macro_export]
macro_rules! forward_display {
    (<$($generic:ident),+> in $this:ty => $field:ident) => {
        impl <$($generic: ::core::fmt::Display),+> ::core::fmt::Display for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.$field, fmt)
            }
        }
    };

    (<$($generic:ident),+> in $this:ty) => {
        impl <$($generic: ::core::fmt::Display),+> ::core::fmt::Display for $this {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, fmt)
            }
        }
    };

    ($ty:ty) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, fmt)
            }
        }
    };

    ($ty:ty => $field:ident) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.$field, fmt)
            }
        }
    };
}

/// Implements [`Display`] for structs using a `format!`-like string constructor.
///
/// # Examples
///
/// Display implementation can be just a string literal.
///
/// ```
/// # use impl_more::forward_display;
/// struct Hello;
/// impl_more::impl_display!(Hello: "hello world");
/// assert_eq!(Hello.to_string(), "hello world");
/// ```
///
/// Explicit and inline format args are supported.
///
/// ```
/// # use impl_more::forward_display;
/// struct Hello2;
/// impl_more::impl_display!(Hello2: "hello world {}", 2);
/// assert_eq!(Hello2.to_string(), "hello world 2");
///
/// const HI: &str = "hello";
///
/// struct Hello3;
/// impl_more::impl_display!(Hello3: "{HI} world");
/// assert_eq!(Hello3.to_string(), "hello world");
/// ```
///
/// [`Display`]: core::fmt::Display
#[macro_export]
macro_rules! impl_display {
    // no format args
    ($ty:ty: $format:literal) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(fmt, $format)
            }
        }
    };

    // with explicit format args
    ($ty:ty: $format:literal, $($args:expr),+) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(fmt, $format, $($args),+)
            }
        }
    };

    // strip trailing comma and forward to format args branch
    ($ty:ty: $format:literal, $($args:expr),+ ,) => {
        $crate::impl_display!($ty: $format, $($args),+);
    };
}

/// Implements [`Display`] for enums using a static string or format args for each variant.
///
/// Unit, tuple, and named-field variants can be mixed. Field patterns use Rust syntax,
/// including renamed fields, `_`, and `..`. The patterns must cover every variant.
///
/// Data variants accept format strings with captured fields and explicit positional or
/// named arguments. Unit variants without explicit arguments write their string verbatim;
/// braces are not interpreted. Add explicit arguments to format a unit variant's output.
///
/// Emitted code supports `no_std` and writes directly to the formatter without a temporary
/// allocation. Outer width, precision, and alignment are not applied to the complete output.
/// Generic parameter declarations and match guards are not supported.
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// use impl_more::impl_display_enum;
///
/// enum Foo {
///     Bar,
///     Qux,
/// }
///
/// impl_display_enum!(Foo: Bar => "bar", Qux => "qux");
///
/// assert_eq!(Foo::Bar.to_string(), "bar");
/// assert_eq!(Foo::Qux.to_string(), "qux");
///
/// enum CoordOrMsg {
///     Coord(i64, i64),
///     Msg(&'static str),
/// }
///
/// impl_display_enum!(CoordOrMsg: Coord(x, y) => "{x}, {y}", Msg(msg) => "message: {msg}");
///
/// assert_eq!(CoordOrMsg::Coord(4, 2).to_string(), "4, 2");
/// assert_eq!(CoordOrMsg::Msg("hi").to_string(), "message: hi");
/// ```
///
/// Mix variant shapes and compute format arguments:
///
/// ```
/// use impl_more::impl_display_enum;
///
/// enum Event {
///     Idle,
///     Items(Vec<u8>),
///     Progress { completed: usize, total: usize },
/// }
///
/// impl_display_enum! {
///     Event:
///     Idle => "idle",
///     Items(items) => "{} items", items.len(),
///     Progress { completed: count, .. } => "{count} complete",
/// }
///
/// assert_eq!(Event::Items(vec![1, 2]).to_string(), "2 items");
/// assert_eq!(Event::Progress { completed: 2, total: 3 }.to_string(), "2 complete");
/// ```
///
/// [`Display`]: core::fmt::Display
#[macro_export]
macro_rules! impl_display_enum {
    // Keep the common unit-only form free of recursive parsing.
    ($ty:ty: $($variant:ident => $text:literal),+ $(,)?) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.write_str(match self {
                    $(Self::$variant => $text,)+
                })
            }
        }
    };

    ($ty:ty: $($variants:tt)+) => {
        $crate::impl_display_enum!(@arms [$ty] [fmt] [] $($variants)+);
    };

    // Collect complete match arms because macros cannot expand to individual arms.
    (@arms [$ty:ty] [$fmt:ident] [$($arms:tt)*]) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, $fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $($arms)*
                }
            }
        }
    };

    (@arms [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        $variant:ident ($($fields:tt)*) => $format:literal $($rest:tt)*
    ) => {
        $crate::impl_display_enum!(@args [$ty] [$fmt] [$($arms)*]
            [Self::$variant($($fields)*)] [format] [$format] [] $($rest)*);
    };

    (@arms [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        $variant:ident { $($fields:tt)* } => $format:literal $($rest:tt)*
    ) => {
        $crate::impl_display_enum!(@args [$ty] [$fmt] [$($arms)*]
            [Self::$variant { $($fields)* }] [format] [$format] [] $($rest)*);
    };

    (@arms [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        $variant:ident => $format:literal $($rest:tt)*
    ) => {
        $crate::impl_display_enum!(@args [$ty] [$fmt] [$($arms)*]
            [Self::$variant] [literal] [$format] [] $($rest)*);
    };

    // Recognize the next variant before treating a comma as a format argument.
    (@args [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        [$($pattern:tt)*] [$mode:ident] [$format:literal] [$($args:tt)*]
        , $variant:ident $(($($tuple:tt)*))? $({$($named:tt)*})? => $($rest:tt)*
    ) => {
        $crate::impl_display_enum!(@arms [$ty] [$fmt] [
            $($arms)*
            $($pattern)* => $crate::impl_display_enum!(@write $fmt $mode $format $($args)*),
        ] $variant $(($($tuple)*))? $({$($named)*})? => $($rest)*);
    };

    (@args [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        [$($pattern:tt)*] [$mode:ident] [$format:literal] [$($args:tt)*]
        , $name:ident = $value:expr $(, $($rest:tt)*)?
    ) => {
        $crate::impl_display_enum!(@args [$ty] [$fmt] [$($arms)*]
            [$($pattern)*] [format] [$format] [$($args)*, $name = $value] $(, $($rest)*)?);
    };

    (@args [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        [$($pattern:tt)*] [$mode:ident] [$format:literal] [$($args:tt)*]
        , $value:expr $(, $($rest:tt)*)?
    ) => {
        $crate::impl_display_enum!(@args [$ty] [$fmt] [$($arms)*]
            [$($pattern)*] [format] [$format] [$($args)*, $value] $(, $($rest)*)?);
    };

    (@args [$ty:ty] [$fmt:ident] [$($arms:tt)*]
        [$($pattern:tt)*] [$mode:ident] [$format:literal] [$($args:tt)*] $(,)?
    ) => {
        $crate::impl_display_enum!(@arms [$ty] [$fmt] [
            $($arms)*
            $($pattern)* => $crate::impl_display_enum!(@write $fmt $mode $format $($args)*),
        ]);
    };

    (@write $fmt:ident literal $text:literal) => {
        $fmt.write_str($text)
    };

    (@write $fmt:ident format $format:literal $($args:tt)*) => {
        ::core::write!($fmt, $format $($args)*)
    };
}

#[cfg(test)]
mod tests {
    use alloc::{
        borrow::ToOwned as _,
        string::{String, ToString as _},
    };

    #[test]
    fn impl_forward_for_newtype_struct() {
        struct Foo(String);

        forward_display!(Foo);

        assert_eq!(Foo("hello world".to_owned()).to_string(), "hello world");
    }

    #[test]
    fn impl_forward_newtype_named_struct() {
        struct Foo {
            inner: u64,
        }

        forward_display!(Foo => inner);

        assert_eq!(Foo { inner: 42 }.to_string(), "42");
    }

    #[test]
    fn impl_forward_generic_newtype_struct() {
        struct Foo<T>(T);

        forward_display!(<T> in Foo<T>);

        assert_eq!(Foo(42).to_string(), "42");
    }

    #[test]
    fn impl_forward_generic_named_struct() {
        struct Foo<T> {
            inner: T,
        }

        forward_display!(<T> in Foo<T> => inner);

        assert_eq!(Foo { inner: 42 }.to_string(), "42");
    }

    #[test]
    fn impl_basic_for_unit_struct() {
        struct Foo;
        impl_display!(Foo: "foo");
        assert_eq!(Foo.to_string(), "foo");
    }

    #[test]
    fn impl_basic_with_args() {
        struct Foo;
        impl_display!(Foo: "foo {} {}", 2, 3);
        assert_eq!(Foo.to_string(), "foo 2 3");
    }

    #[test]
    fn impl_basic_with_inline_args() {
        const HI: &str = "hello";

        struct Hello3;
        impl_display!(Hello3: "{HI} world");
        assert_eq!(Hello3.to_string(), "hello world");
    }

    #[test]
    fn impl_enum_named_variant_with_trailing_comma() {
        enum Foo {
            Bar { value: u64 },
        }

        impl_display_enum!(Foo: Bar { value } => "{value}",);

        assert_eq!(Foo::Bar { value: 42 }.to_string(), "42");
    }

    #[test]
    fn enum_mixed_patterns_and_format_arguments() {
        #[allow(dead_code)] // These fields test patterns that ignore data.
        enum Event {
            Idle,
            Message(&'static str, bool),
            Progress { completed: usize, total: usize },
            Items(&'static [u8]),
            Done,
        }

        impl_display_enum! {
            Event:
            Idle => "{{idle}}",
            Message(msg, ..) => "{{{msg}}}",
            Progress { completed: count, .. } => "{count} complete",
            Items(items) => "{} items ({label})", items.len(), label = "ready",
            Done => "done: {}", 42,
        }

        assert_eq!(Event::Idle.to_string(), "{{idle}}");
        assert_eq!(Event::Message("hi", true).to_string(), "{hi}");
        assert_eq!(
            Event::Progress {
                completed: 2,
                total: 3
            }
            .to_string(),
            "2 complete"
        );
        assert_eq!(Event::Items(&[1, 2]).to_string(), "2 items (ready)");
        assert_eq!(Event::Done.to_string(), "done: 42");
    }

    #[test]
    fn enum_format_arguments_without_trailing_comma() {
        #[allow(dead_code)] // The first tuple field tests the wildcard pattern.
        enum Value {
            Tuple(u8, u8),
            Named { value: u8 },
        }

        impl_display_enum!(Value:
            Tuple(_, value) => "{value:02x}",
            Named { value } => "{number:02x}", number = value
        );

        assert_eq!(Value::Tuple(0, 10).to_string(), "0a");
        assert_eq!(Value::Named { value: 11 }.to_string(), "0b");

        enum Count {
            Items(&'static [u8]),
        }
        impl_display_enum!(Count: Items(items) => "{}", items.len());
        assert_eq!(Count::Items(&[1, 2]).to_string(), "2");
    }

    #[test]
    fn enum_propagates_format_errors() {
        struct Fails;
        impl core::fmt::Display for Fails {
            fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                Err(core::fmt::Error)
            }
        }

        enum Value {
            Tuple(Fails),
            Named { value: Fails },
        }
        impl_display_enum!(Value: Tuple(value) => "{value}", Named { value } => "{value}");

        let mut output = String::new();
        assert!(core::fmt::write(&mut output, format_args!("{}", Value::Tuple(Fails))).is_err());
        assert!(core::fmt::write(
            &mut output,
            format_args!("{}", Value::Named { value: Fails })
        )
        .is_err());
    }
}
