// Keep const declarations unchanged. Apply an optional trait bound to type parameters only.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_more_parse_generics {
    (@parse $macro:ident [$($bound:tt)*] [$($generic:tt)*] > in $($rest:tt)+) => {
        $crate::$macro!(@impl [$($generic)*] $($rest)+);
    };

    (@parse $macro:ident [$($bound:tt)*] [$($generic:tt)*]
        const $name:ident : $ty:ty, $($rest:tt)+) => {
        $crate::__impl_more_parse_generics!(
            @parse $macro [$($bound)*] [$($generic)* const $name: $ty,] $($rest)+
        );
    };

    (@parse $macro:ident [$($bound:tt)*] [$($generic:tt)*]
        const $name:ident : $ty:ty > in $($rest:tt)+) => {
        $crate::$macro!(@impl [$($generic)* const $name: $ty,] $($rest)+);
    };

    (@parse $macro:ident [$($bound:tt)*] [$($generic:tt)*]
        $name:ident, $($rest:tt)+) => {
        $crate::__impl_more_parse_generics!(
            @parse $macro [$($bound)*] [$($generic)* $name $($bound)*,] $($rest)+
        );
    };

    (@parse $macro:ident [$($bound:tt)*] [$($generic:tt)*]
        $name:ident > in $($rest:tt)+) => {
        $crate::$macro!(@impl [$($generic)* $name $($bound)*,] $($rest)+);
    };
}
