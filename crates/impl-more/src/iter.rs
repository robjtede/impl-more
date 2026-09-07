/// Implement [`IntoIterator`] by forwarding to a field.
///
/// Select one or more modes: `owned` for `Self`, `ref` for `&Self`, and `ref_mut`
/// for `&mut Self`. Each mode uses the inner type's item and iterator types.
/// Only the selected modes are implemented. Emitted code supports `no_std`.
///
/// # Examples
///
/// ```
/// struct Items<T>(Vec<T>);
/// impl_more::forward_into_iterator!(<T> in Items<T> => Vec<T>; owned, ref, ref_mut);
/// let mut items = Items(vec![1, 2]);
/// for item in &mut items { *item += 1; }
/// assert_eq!((&items).into_iter().copied().collect::<Vec<_>>(), vec![2, 3]);
/// assert_eq!(items.into_iter().collect::<Vec<_>>(), vec![2, 3]);
/// ```
///
/// Select a named field with `=> field: InnerType`:
///
/// ```
/// struct Items { values: Vec<u8> }
/// impl_more::forward_into_iterator!(Items => values: Vec<u8>; ref);
/// assert_eq!((&Items { values: vec![1] }).into_iter().next(), Some(&1));
/// ```
#[macro_export]
macro_rules! forward_into_iterator {
    (<$($generic:ident),+> in $this:ty => $field:tt : $inner:ty; $($mode:ident),+ $(,)?) => {
        $crate::forward_into_iterator!(@modes [$($generic),+] $this => $field: $inner; $($mode),+);
    };
    (<$($generic:ident),+> in $this:ty => $inner:ty; $($mode:ident),+ $(,)?) => {
        $crate::forward_into_iterator!(@modes [$($generic),+] $this => 0: $inner; $($mode),+);
    };
    ($this:ty => $field:tt : $inner:ty; $($mode:ident),+ $(,)?) => {
        $crate::forward_into_iterator!(@modes [] $this => $field: $inner; $($mode),+);
    };
    ($this:ty => $inner:ty; $($mode:ident),+ $(,)?) => {
        $crate::forward_into_iterator!(@modes [] $this => 0: $inner; $($mode),+);
    };
    (@modes $generics:tt $this:ty => $field:tt : $inner:ty; $($mode:ident),+) => {
        $( $crate::forward_into_iterator!(@mode $generics $this => $field: $inner; $mode); )+
    };
    (@mode [$($generic:ident),*] $this:ty => $field:tt : $inner:ty; owned) => {
        impl<$($generic),*> ::core::iter::IntoIterator for $this
        where
            $inner: ::core::iter::IntoIterator,
        {
            type Item = <$inner as ::core::iter::IntoIterator>::Item;
            type IntoIter = <$inner as ::core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                <$inner as ::core::iter::IntoIterator>::into_iter(self.$field)
            }
        }
    };
    (@mode [$($generic:ident),*] $this:ty => $field:tt : $inner:ty; ref) => {
        impl<'__impl_more_iter, $($generic),*> ::core::iter::IntoIterator for &'__impl_more_iter $this
        where
            $inner: '__impl_more_iter,
            &'__impl_more_iter $inner: ::core::iter::IntoIterator,
        {
            type Item = <&'__impl_more_iter $inner as ::core::iter::IntoIterator>::Item;
            type IntoIter = <&'__impl_more_iter $inner as ::core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                <&'__impl_more_iter $inner as ::core::iter::IntoIterator>::into_iter(&self.$field)
            }
        }
    };
    (@mode [$($generic:ident),*] $this:ty => $field:tt : $inner:ty; ref_mut) => {
        impl<'__impl_more_iter, $($generic),*> ::core::iter::IntoIterator for &'__impl_more_iter mut $this
        where
            $inner: '__impl_more_iter,
            &'__impl_more_iter mut $inner: ::core::iter::IntoIterator,
        {
            type Item = <&'__impl_more_iter mut $inner as ::core::iter::IntoIterator>::Item;
            type IntoIter = <&'__impl_more_iter mut $inner as ::core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                <&'__impl_more_iter mut $inner as ::core::iter::IntoIterator>::into_iter(&mut self.$field)
            }
        }
    };
    (@mode $generics:tt $this:ty => $field:tt : $inner:ty; $mode:ident) => {
        ::core::compile_error!("expected iteration mode `owned`, `ref`, or `ref_mut`");
    };
}

/// Implement [`FromIterator`](core::iter::FromIterator) for a single-field wrapper.
///
/// Accept any item type supported by the inner collection's `FromIterator`
/// implementation. The inner type does not need to implement `IntoIterator`.
/// Emitted code supports `no_std`.
///
/// # Examples
///
/// ```
/// struct Items<T>(Vec<T>);
/// impl_more::forward_from_iterator!(<T> in Items<T> => Vec<T>);
/// let items = (1..4).collect::<Items<_>>();
/// assert_eq!(items.0, vec![1, 2, 3]);
/// ```
///
/// ```
/// struct Text { value: String }
/// impl_more::forward_from_iterator!(Text => value: String);
/// let text = ["hello", " world"].iter().copied().collect::<Text>();
/// assert_eq!(text.value, "hello world");
/// ```
#[macro_export]
macro_rules! forward_from_iterator {
    (<$($generic:ident),+> in $this:ty => $field:tt : $inner:ty $(,)?) => {
        $crate::forward_from_iterator!(@impl [$($generic),+] $this => $field: $inner);
    };
    (<$($generic:ident),+> in $this:ty => $inner:ty $(,)?) => {
        $crate::forward_from_iterator!(@impl [$($generic),+] $this => 0: $inner);
    };
    ($this:ty => $field:tt : $inner:ty $(,)?) => {
        $crate::forward_from_iterator!(@impl [] $this => $field: $inner);
    };
    ($this:ty => $inner:ty $(,)?) => {
        $crate::forward_from_iterator!(@impl [] $this => 0: $inner);
    };
    (@impl [$($generic:ident),*] $this:ty => $field:tt : $inner:ty) => {
        impl<__ImplMoreItem, $($generic),*> ::core::iter::FromIterator<__ImplMoreItem> for $this
        where
            $inner: ::core::iter::FromIterator<__ImplMoreItem>,
        {
            fn from_iter<__ImplMoreIter: ::core::iter::IntoIterator<Item = __ImplMoreItem>>(iter: __ImplMoreIter) -> Self {
                Self { $field: <$inner as ::core::iter::FromIterator<__ImplMoreItem>>::from_iter(iter) }
            }
        }
    };
}

/// Implement [`Extend`] by forwarding to a field.
///
/// Accept any item type supported by the inner collection's `Extend`
/// implementation. Emitted code supports `no_std`.
///
/// # Examples
///
/// ```
/// struct Items<T>(Vec<T>);
/// impl_more::forward_extend!(<T> in Items<T> => Vec<T>);
/// let mut items = Items(vec![1]);
/// items.extend([2, 3]);
/// assert_eq!(items.0, vec![1, 2, 3]);
/// ```
///
/// ```
/// struct Text { value: String }
/// impl_more::forward_extend!(Text => value: String);
/// let mut text = Text { value: String::new() };
/// text.extend(["hello", " world"]);
/// assert_eq!(text.value, "hello world");
/// ```
#[macro_export]
macro_rules! forward_extend {
    (<$($generic:ident),+> in $this:ty => $field:tt : $inner:ty $(,)?) => {
        $crate::forward_extend!(@impl [$($generic),+] $this => $field: $inner);
    };
    (<$($generic:ident),+> in $this:ty => $inner:ty $(,)?) => {
        $crate::forward_extend!(@impl [$($generic),+] $this => 0: $inner);
    };
    ($this:ty => $field:tt : $inner:ty $(,)?) => {
        $crate::forward_extend!(@impl [] $this => $field: $inner);
    };
    ($this:ty => $inner:ty $(,)?) => {
        $crate::forward_extend!(@impl [] $this => 0: $inner);
    };
    (@impl [$($generic:ident),*] $this:ty => $field:tt : $inner:ty) => {
        impl<__ImplMoreItem, $($generic),*> ::core::iter::Extend<__ImplMoreItem> for $this
        where
            $inner: ::core::iter::Extend<__ImplMoreItem>,
        {
            fn extend<__ImplMoreIter: ::core::iter::IntoIterator<Item = __ImplMoreItem>>(&mut self, iter: __ImplMoreIter) {
                <$inner as ::core::iter::Extend<__ImplMoreItem>>::extend(&mut self.$field, iter);
            }
        }
    };
}
