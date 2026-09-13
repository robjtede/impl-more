/// Implement [`From`] for an enum variant with one named or unnamed field.
///
/// The destination is the enum path followed by the variant name. Each source
/// type can have only one conversion into a given enum. Generic conversions must
/// not overlap with other conversions for any choice of type arguments.
///
/// # Examples
///
/// With an error enum:
/// ```
/// enum Error {
///     Io(std::io::Error),
/// }
///
/// impl_more::impl_enum_from!(std::io::Error => Error::Io);
///
/// fn read() -> Result<String, Error> {
///     Ok(std::fs::read_to_string("config.txt")?)
/// }
/// ```
///
/// With a named field:
/// ```
/// enum Error {
///     Io { source: std::io::Error },
/// }
///
/// impl_more::impl_enum_from!(std::io::Error => Error::Io { source });
///
/// let error = Error::from(std::io::Error::from(std::io::ErrorKind::NotFound));
/// assert!(matches!(error, Error::Io { source } if source.kind() == std::io::ErrorKind::NotFound));
/// ```
///
/// With a generic enum and a module path:
/// ```
/// mod errors {
///     pub enum Error<T> {
///         Io(std::io::Error),
///         Other(Vec<T>),
///     }
/// }
///
/// impl_more::impl_enum_from!(<T> in std::io::Error => errors::Error<T>::Io);
/// impl_more::impl_enum_from!(<T> in Vec<T> => errors::Error<T>::Other);
///
/// let error = errors::Error::from(vec![42_u32]);
/// assert!(matches!(error, errors::Error::Other(values) if values == [42]));
/// ```
#[macro_export]
macro_rules! impl_enum_from {
    (<$($generic:ident),+> in $from:ty => $($target:tt)+) => {
        $crate::impl_enum_from!(@parse [<$($generic),+>] [$from] [] $($target)+);
    };

    ($from:ty => $($target:tt)+) => {
        $crate::impl_enum_from!(@parse [] [$from] [] $($target)+);
    };

    (@parse [$($generic:tt)*] [$from:ty] [$($path:tt)*]
        $enum:ident $(<$($arg:ty),+>)? :: $variant:ident $(,)?) => {
        impl $($generic)* ::core::convert::From<$from>
            for $($path)* $enum $(<$($arg),+>)?
        {
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };

    (@parse [$($generic:tt)*] [$from:ty] [$($path:tt)*]
        $enum:ident $(<$($arg:ty),+>)? :: $variant:ident { $field:ident $(,)? } $(,)?) => {
        impl $($generic)* ::core::convert::From<$from>
            for $($path)* $enum $(<$($arg),+>)?
        {
            fn from(value: $from) -> Self {
                Self::$variant { $field: value }
            }
        }
    };

    (@parse [$($generic:tt)*] [$from:ty] [$($path:tt)*]
        $segment:ident :: $($rest:tt)+) => {
        $crate::impl_enum_from!(@parse [$($generic)*] [$from]
            [$($path)* $segment ::] $($rest)+);
    };

    (@parse [$($generic:tt)*] [$from:ty] [] :: $($rest:tt)+) => {
        $crate::impl_enum_from!(@parse [$($generic)*] [$from] [::] $($rest)+);
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq)]
    enum Value<T> {
        Some(T),
    }

    impl_enum_from!(u32 => crate::enum_from::tests::Value<u32>::Some);

    #[test]
    fn error_conversion_with_question_mark() {
        enum Error {
            Io(std::io::Error),
            Parse(core::num::ParseIntError),
        }

        impl_enum_from!(std::io::Error => Error::Io);
        impl_enum_from!(core::num::ParseIntError => Error::Parse,);

        fn fail() -> Result<(), Error> {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))?;
            Ok(())
        }

        match fail() {
            Err(Error::Io(error)) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
            _ => panic!("expected an IO error"),
        }

        let source = "invalid".parse::<u32>().unwrap_err();
        match Error::from(source.clone()) {
            Error::Parse(error) => assert_eq!(error, source),
            _ => panic!("expected a parse error"),
        }
    }

    #[test]
    fn generic_enum_with_module_path() {
        mod errors {
            #[derive(Debug, PartialEq)]
            pub enum Error<T, U> {
                Code(u32),
                Other((T, U)),
            }
        }

        impl_enum_from!(<T, U> in u32 => errors::Error<T, U>::Code);
        impl_enum_from!(<T, U> in (T, U) => errors::Error<T, U>::Other,);

        assert_eq!(errors::Error::<u8, bool>::from(42), errors::Error::Code(42));
        assert_eq!(
            errors::Error::from((7_u8, true)),
            errors::Error::Other((7, true))
        );
    }

    #[test]
    fn named_error_conversion_with_question_mark() {
        enum Error {
            Io { source: std::io::Error },
        }

        impl_enum_from!(std::io::Error => Error::Io { source });

        fn fail() -> Result<(), Error> {
            Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))?;
            Ok(())
        }

        match fail() {
            Err(Error::Io { source }) => {
                assert_eq!(source.kind(), std::io::ErrorKind::PermissionDenied);
            }
            _ => panic!("expected an IO error"),
        }
    }

    #[test]
    fn named_generic_enum_with_module_path() {
        mod errors {
            #[derive(Debug, PartialEq)]
            pub enum Error<T> {
                Other { source: T },
            }
        }

        impl_enum_from!(<T> in T => errors::Error<T>::Other { source, },);

        assert_eq!(
            errors::Error::from(42_u32),
            errors::Error::Other { source: 42 }
        );
    }

    #[test]
    fn concrete_generic_arguments_with_crate_path() {
        assert_eq!(Value::<u32>::from(42), Value::Some(42));
    }
}
