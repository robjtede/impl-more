//! Tests for formatting forwarding.

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
    impl_more::forward_debug!(Tuple,);
    impl_more::forward_debug!(Named => inner,);
    impl_more::forward_debug!(<T> in Generic<T>);
    impl_more::forward_debug!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_binary!(Tuple,);
    impl_more::forward_binary!(Named => inner,);
    impl_more::forward_binary!(<T> in Generic<T>);
    impl_more::forward_binary!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_octal!(Tuple,);
    impl_more::forward_octal!(Named => inner,);
    impl_more::forward_octal!(<T> in Generic<T>);
    impl_more::forward_octal!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_lower_hex!(Tuple,);
    impl_more::forward_lower_hex!(Named => inner,);
    impl_more::forward_lower_hex!(<T> in Generic<T>);
    impl_more::forward_lower_hex!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_upper_hex!(Tuple,);
    impl_more::forward_upper_hex!(Named => inner,);
    impl_more::forward_upper_hex!(<T> in Generic<T>);
    impl_more::forward_upper_hex!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_lower_exp!(Tuple,);
    impl_more::forward_lower_exp!(Named => inner,);
    impl_more::forward_lower_exp!(<T> in Generic<T>);
    impl_more::forward_lower_exp!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_upper_exp!(Tuple,);
    impl_more::forward_upper_exp!(Named => inner,);
    impl_more::forward_upper_exp!(<T> in Generic<T>);
    impl_more::forward_upper_exp!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_pointer!(Tuple,);
    impl_more::forward_pointer!(Named => inner,);
    impl_more::forward_pointer!(<T> in Generic<T>);
    impl_more::forward_pointer!(<T> in GenericNamed<T> => inner);
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
    impl_more::forward_debug!(<T> in Value<T>);
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
    impl_more::forward_lower_hex!(Value);
    let mut output = String::new();
    assert!(core::fmt::write(&mut output, format_args!("{:x}", Value(Fails))).is_err());
}
