//! Tests for Deref macro syntax and behavior.

#[test]
fn forward_const_generic_newtype() {
    struct Array<T, const N: usize>(Box<[T; N]>);

    impl_more::forward_deref_and_mut!(<T, const N: usize> in Array<T, N> => [T; N]);

    let mut value = Array(Box::new([1, 2, 3]));
    value.reverse();

    assert_eq!(&*value, &[3, 2, 1]);
}

#[test]
fn forward_const_generic_named_ref() {
    struct Array<const N: usize> {
        items: Box<[u8; N]>,
    }

    impl_more::forward_deref_and_mut!(<const N: usize,> in Array<N> => items: ref [u8; N]);

    let mut value = Array {
        items: Box::new([1, 2]),
    };
    value.reverse();

    assert_eq!(&*value, &[2, 1]);
}

#[test]
fn const_generic_newtype() {
    struct Array<T, const N: usize = 3>([T; N]);

    impl_more::impl_deref_and_mut!(<T, const N: usize> in Array<T, N> => [T; N]);

    let mut array = Array::<_, 2>([1, 2]);

    assert_eq!(&*array, &[1, 2]);

    array.reverse();

    assert_eq!(&*array, &[2, 1]);

    let mut default = Array::<u8>([1, 2, 3]);
    default.reverse();

    assert_eq!(&*default, &[3, 2, 1]);
}

#[test]
fn const_generic_named_field() {
    struct Array<T, const COUNT: usize> {
        items: [T; COUNT],
    }

    impl_more::impl_deref!(<T, const COUNT: usize> in Array<T, COUNT> => items: [T; COUNT]);
    impl_more::impl_deref_mut!(<T, const COUNT: usize> in Array<T, COUNT> => items);

    let mut value = Array { items: [1, 2, 3] };

    assert_eq!(&*value, &[1, 2, 3]);

    value.reverse();

    assert_eq!(&*value, &[3, 2, 1]);
}

#[test]
fn const_only_newtype() {
    struct Buffer<const COUNT: usize, const ENABLED: bool>([u8; COUNT]);

    impl_more::impl_deref!(
        <const COUNT: usize, const ENABLED: bool,> in Buffer<COUNT, ENABLED> => [u8; COUNT]
    );
    impl_more::impl_deref_mut!(
        <const COUNT: usize, const ENABLED: bool,> in Buffer<COUNT, ENABLED>
    );

    let mut value = Buffer::<3, true>([1, 2, 3]);

    assert_eq!(&*value, &[1, 2, 3]);

    value[1] = 42;

    assert_eq!(&*value, &[1, 42, 3]);
}

#[test]
fn combined_const_generic_named_field() {
    struct Array<const COUNT: usize, T, U> {
        items: [T; COUNT],
        marker: core::marker::PhantomData<U>,
    }

    impl_more::impl_deref_and_mut!(
        <const COUNT: usize, T, U> in Array<COUNT, T, U> => items: [T; COUNT]
    );

    let mut value = Array::<3, _, bool> {
        items: [1, 2, 3],
        marker: core::marker::PhantomData,
    };

    assert_eq!(&*value, &[1, 2, 3]);

    value.reverse();

    assert_eq!(&*value, &[3, 2, 1]);
}

// Older compilers reject associated type projections in trait impls.
#[rustversion::since(1.81)]
#[test]
fn qualified_self_type() {
    trait Identity {
        type Value;
    }

    struct Newtype(String);

    impl Identity for Newtype {
        type Value = Self;
    }

    impl_more::impl_deref_and_mut!(<Newtype as Identity>::Value => String);

    let mut newtype = Newtype(String::from("hello"));
    newtype.push('!');

    assert_eq!(&*newtype, "hello!");

    struct Named {
        value: String,
    }

    impl Identity for Named {
        type Value = Self;
    }

    impl_more::impl_deref!(<Named as Identity>::Value => value: String);
    impl_more::impl_deref_mut!(<Named as Identity>::Value => value);

    let mut named = Named {
        value: String::from("hello"),
    };
    named.push('!');

    assert_eq!(&*named, "hello!");
}
