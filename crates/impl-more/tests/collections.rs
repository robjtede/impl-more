//! Tests for collection forwarding.

struct Items<T>(Vec<T>);
impl_more::forward_into_iterator!(<T> in Items<T> => Vec<T>; owned, ref, ref_mut,);
impl_more::forward_from_iterator!(<T> in Items<T> => Vec<T>,);
impl_more::forward_extend!(<T> in Items<T> => Vec<T>,);

#[test]
fn collect_extend_and_iterate_non_copy_items() {
    let mut items = IntoIterator::into_iter([String::from("a")]).collect::<Items<_>>();
    items.extend([String::from("b")]);
    for item in &mut items {
        item.push('!');
    }
    assert_eq!(
        (&items).into_iter().map(String::as_str).collect::<Vec<_>>(),
        ["a!", "b!"]
    );
    assert_eq!(items.into_iter().collect::<Vec<_>>(), ["a!", "b!"]);
}

#[test]
fn generic_named_field_and_borrowed_items() {
    struct Named<T> {
        values: Vec<T>,
    }
    impl_more::forward_into_iterator!(<T> in Named<T> => values: Vec<T>; ref_mut, owned, ref);
    impl_more::forward_from_iterator!(<T> in Named<T> => values: Vec<T>);
    impl_more::forward_extend!(<T> in Named<T> => values: Vec<T>);
    let source = vec![String::from("a")];
    let mut items = source.iter().collect::<Named<_>>();
    items.extend(source.iter());
    for item in &mut items {
        assert_eq!(*item, &source[0]);
    }
    assert_eq!((&items).into_iter().count(), 2);
    assert_eq!(
        items.into_iter().collect::<Vec<_>>(),
        [&source[0], &source[0]]
    );
}

#[test]
fn concrete_named_field() {
    struct Named {
        values: Vec<u8>,
    }
    impl_more::forward_into_iterator!(Named => values: Vec<u8>; owned, ref, ref_mut);
    impl_more::forward_from_iterator!(Named => values: Vec<u8>);
    impl_more::forward_extend!(Named => values: Vec<u8>);
    let mut items = (0..2).collect::<Named>();
    items.extend([2]);
    for item in &mut items {
        *item += 1;
    }
    assert_eq!((&items).into_iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq!(items.into_iter().collect::<Vec<_>>(), [1, 2, 3]);
}

#[test]
fn concrete_tuple_and_empty_input() {
    struct Bytes(Vec<u8>);
    impl_more::forward_into_iterator!(Bytes => Vec<u8>; owned, ref, ref_mut);
    impl_more::forward_from_iterator!(Bytes => Vec<u8>);
    impl_more::forward_extend!(Bytes => Vec<u8>);
    let mut bytes = core::iter::empty::<u8>().collect::<Bytes>();
    bytes.extend(core::iter::empty::<u8>());
    assert_eq!((&bytes).into_iter().next(), None);
    assert_eq!((&mut bytes).into_iter().next(), None);
    assert_eq!(bytes.into_iter().next(), None);
}

#[test]
fn collection_without_into_iterator_accepts_multiple_item_types() {
    struct Text(String);
    impl_more::forward_from_iterator!(Text => String);
    impl_more::forward_extend!(Text => String);
    let mut text = ['a', 'b'].iter().copied().collect::<Text>();
    text.extend(["cd", "ef"]);
    text.extend(['g']);
    assert_eq!(text.0, "abcdefg");
    assert_eq!(["ab", "cd"].iter().copied().collect::<Text>().0, "abcd");
}

#[test]
fn modes_are_opt_in_and_bounds_apply_to_the_collection() {
    struct Shared<T>(T);
    impl_more::forward_into_iterator!(<T> in Shared<T> => T; ref);
    static_assertions::assert_not_impl_any!(Shared<Vec<u8>>: IntoIterator);
    static_assertions::assert_not_impl_any!(&mut Shared<Vec<u8>>: IntoIterator);
    assert_eq!(
        (&Shared(vec![1, 2]))
            .into_iter()
            .copied()
            .collect::<Vec<_>>(),
        [1, 2]
    );

    struct Owned<T>(T);
    impl_more::forward_into_iterator!(<T> in Owned<T> => T; owned);
    static_assertions::assert_not_impl_any!(&Owned<Vec<u8>>: IntoIterator);
    static_assertions::assert_not_impl_any!(&mut Owned<Vec<u8>>: IntoIterator);
    assert_eq!(Owned(0..3).into_iter().sum::<u8>(), 3);
}
