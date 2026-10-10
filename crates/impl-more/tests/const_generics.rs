//! Tests for const parameters in trait implementation macros.

#[test]
fn direct_array_references() {
    struct Array<T, const N: usize = 3>([T; N]);

    impl_more::impl_as_ref!(<T, const N: usize> in Array<T, N> => [T; N]);
    impl_more::impl_as_mut!(<T, const N: usize,> in Array<T, N> => [T; N]);

    let mut value = Array::<u8>([1, 2, 3]);

    assert_eq!(AsRef::<[u8; 3]>::as_ref(&value), &[1, 2, 3]);

    AsMut::<[u8; 3]>::as_mut(&mut value).reverse();

    assert_eq!(AsRef::<[u8; 3]>::as_ref(&value), &[3, 2, 1]);
}

#[test]
fn direct_named_array_references() {
    struct Buffer<const N: usize, const ENABLED: bool> {
        items: [u8; N],
    }

    impl_more::impl_as_ref!(
        <const N: usize, const ENABLED: bool,> in Buffer<N, ENABLED> => items: [u8; N]
    );
    impl_more::impl_as_mut!(
        <const N: usize, const ENABLED: bool> in Buffer<N, ENABLED> => items: [u8; N]
    );

    let mut value = Buffer::<2, true> { items: [1, 2] };
    AsMut::<[u8; 2]>::as_mut(&mut value)[0] = 3;

    assert_eq!(AsRef::<[u8; 2]>::as_ref(&value), &[3, 2]);
}

#[test]
fn forwarded_array_slices() {
    struct Array<const N: usize, T>([T; N]);

    impl_more::forward_as_ref!(<const N: usize, T,> in Array<N, T> => [T]);
    impl_more::forward_as_mut!(<const N: usize, T> in Array<N, T> => [T]);

    let mut value = Array([1, 2, 3]);
    AsMut::<[i32]>::as_mut(&mut value).reverse();

    assert_eq!(AsRef::<[i32]>::as_ref(&value), &[3, 2, 1]);
}

#[test]
fn combined_named_array_slices() {
    struct Array<T, const N: usize> {
        items: [T; N],
    }

    impl_more::forward_as_ref_and_mut!(<T, const N: usize> in Array<T, N> => items: [T]);

    let mut value = Array { items: [1, 2, 3] };
    AsMut::<[i32]>::as_mut(&mut value)[1] = 4;

    assert_eq!(AsRef::<[i32]>::as_ref(&value), &[1, 4, 3]);
}

#[test]
#[allow(clippy::from_over_into)]
fn array_conversions() {
    struct Array<T, const N: usize>([T; N]);

    impl_more::impl_from!(<T, const N: usize,> in [T; N] => Array<T, N>);
    impl_more::impl_into!(<T, const N: usize> in Array<T, N> => [T; N]);

    let value = Array::from([String::from("a"), String::from("b")]);

    assert_eq!(Into::<[String; 2]>::into(value), ["a", "b"]);
}

#[test]
#[allow(clippy::from_over_into)]
fn named_array_conversions() {
    struct Array<const N: usize, T, const ENABLED: bool> {
        items: [T; N],
    }

    impl_more::impl_from!(
        <const N: usize, T, const ENABLED: bool> in [T; N] => Array<N, T, ENABLED>: items,
    );
    impl_more::impl_into!(
        <const N: usize, T, const ENABLED: bool,> in Array<N, T, ENABLED> => [T; N]: items
    );

    let value = Array::<2, _, true>::from([1, 2]);

    assert_eq!(Into::<[i32; 2]>::into(value), [1, 2]);
}

#[test]
fn enum_array_conversions() {
    mod values {
        #[derive(Debug, PartialEq)]
        pub enum Tuple<T, const N: usize> {
            Items([T; N]),
        }

        #[derive(Debug, PartialEq)]
        pub enum Named<const N: usize, T, const ENABLED: bool> {
            Items { items: [T; N] },
        }
    }

    impl_more::impl_enum_from!(<T, const N: usize> in [T; N] => values::Tuple<T, N>::Items);
    impl_more::impl_enum_from!(
        <const N: usize, T, const ENABLED: bool,> in [T; N]
        => values::Named<N, T, ENABLED>::Items { items, },
    );

    assert_eq!(values::Tuple::from([1, 2]), values::Tuple::Items([1, 2]));
    assert_eq!(
        values::Named::<2, _, true>::from([1, 2]),
        values::Named::Items { items: [1, 2] }
    );
}

#[test]
fn enum_concrete_const_arguments() {
    #[derive(Debug, PartialEq)]
    enum Value<T, const N: usize, const ENABLED: bool> {
        Item(T),
    }

    impl_more::impl_enum_from!(Vec<Vec<u8>> => Value<Vec<Vec<u8>>, 2, true>::Item);
    impl_more::impl_enum_from!([u8; 3] => Value<[u8; 3], { 1 + 2 }, false>::Item);

    assert_eq!(
        Value::<_, 2, true>::from(vec![vec![1]]),
        Value::Item(vec![vec![1]])
    );
    assert_eq!(
        Value::<_, 3, false>::from([1, 2, 3]),
        Value::Item([1, 2, 3])
    );
}

#[test]
fn array_iteration_modes() {
    struct Array<T, const N: usize>([T; N]);

    impl_more::forward_into_iterator!(
        <T, const N: usize> in Array<T, N> => [T; N]; owned, ref, ref_mut,
    );

    let mut value = Array([String::from("a"), String::from("b")]);

    for item in &mut value {
        item.push('!');
    }

    assert_eq!(
        (&value).into_iter().map(String::as_str).collect::<Vec<_>>(),
        ["a!", "b!"]
    );
    assert_eq!(value.into_iter().collect::<Vec<_>>(), ["a!", "b!"]);
}

#[test]
fn named_const_only_array_iteration() {
    struct Array<const N: usize> {
        items: [u8; N],
    }

    impl_more::forward_into_iterator!(
        <const N: usize,> in Array<N> => items: [u8; N]; owned, ref, ref_mut,
    );

    let mut value = Array { items: [1, 2] };

    for item in &mut value {
        *item += 1;
    }

    assert_eq!((&value).into_iter().copied().collect::<Vec<_>>(), [2, 3]);
    assert_eq!(value.into_iter().collect::<Vec<_>>(), [2, 3]);
}

#[test]
fn collection_capacity_controls_collect_and_extend() {
    struct Bounded<T, const N: usize> {
        values: Vec<T>,
    }

    impl<T, const N: usize> FromIterator<T> for Bounded<T, N> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self {
                values: iter.into_iter().take(N).collect(),
            }
        }
    }

    impl<T, const N: usize> Extend<T> for Bounded<T, N> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            let remaining = N - self.values.len();

            self.values.extend(iter.into_iter().take(remaining));
        }
    }

    struct Items<T, const N: usize>(Bounded<T, N>);

    struct Named<const N: usize, T> {
        items: Bounded<T, N>,
    }

    impl_more::forward_from_iterator!(<T, const N: usize> in Items<T, N> => Bounded<T, N>);
    impl_more::forward_extend!(<T, const N: usize> in Items<T, N> => Bounded<T, N>);
    impl_more::forward_from_iterator!(
        <const N: usize, T,> in Named<N, T> => items: Bounded<T, N>,
    );
    impl_more::forward_extend!(<const N: usize, T,> in Named<N, T> => items: Bounded<T, N>,);

    let mut tuple = [String::from("a")].into_iter().collect::<Items<_, 2>>();
    tuple.extend([String::from("b"), String::from("c")]);

    assert_eq!(tuple.0.values, ["a", "b"]);

    let mut named = [String::from("a"), String::from("b")]
        .into_iter()
        .collect::<Named<1, _>>();
    named.extend([String::from("c")]);

    assert_eq!(named.items.values, ["a"]);

    let mut empty = [String::from("a")].into_iter().collect::<Items<_, 0>>();
    empty.extend([String::from("b")]);

    assert!(empty.0.values.is_empty());
}

#[test]
fn debug_forwards_array_formatting() {
    struct Array<T, const N: usize>([T; N]);

    struct Named<const N: usize, T> {
        items: [T; N],
    }

    impl_more::forward_debug!(<T, const N: usize> in Array<T, N>);
    impl_more::forward_debug!(<const N: usize, T,> in Named<N, T> => items,);

    let items = [String::from("a"), String::from("b")];
    let expected = format!("{items:#?}");

    assert_eq!(format!("{:#?}", Array(items.clone())), expected);
    assert_eq!(
        format!("{:>12.3?}", Named { items }),
        format!("{:>12.3?}", [String::from("a"), String::from("b")])
    );
}

#[test]
fn const_only_formatting() {
    struct Buffer<const N: usize>([u8; N]);

    impl_more::forward_debug!(<const N: usize> in Buffer<N>);

    assert_eq!(format!("{:?}", Buffer([1, 2])), "[1, 2]");
}
