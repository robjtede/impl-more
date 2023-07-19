pub const DEFAULT_JSON_LIMIT: usize = 2_097_152;

#[derive(Debug)]
// #[derive(Display)]
pub struct Either<L, R> {
    left: L,
    right: R,
}

impl_more::impl_deref_and_mut!(<L, R> in Either<L, R> => left:L);

#[derive(Debug)]
// #[derive(Display)]
pub struct Json<T, const LIMIT: usize = DEFAULT_JSON_LIMIT>(pub T);

impl_more::impl_deref_and_mut!(<T, const LIMIT: usize> in Json<T, LIMIT> => T);

static_assertions::assert_impl_all!(Json<T, const LIMIT: usize>: Deref, DerefMut);
