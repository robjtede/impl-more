# Changelog

## Unreleased


## 0.1.1 - 2022-10-17
- No significant changes since `0.1.0`.


## 0.1.0 - 2022-10-17

- Change form of `impl_as_{ref,mut}!`, `impl_deref[_mut]!`, and `impl_{from,into}!` to be clearer when using named fields.
- Add support for type parameters when using `impl_as_{ref,mut}!`, `impl_deref[_mut]!`, and `impl_{from,into}!`.

## 0.0.4 - 2022-10-16

- No significant changes since `0.0.3`.

## 0.0.3 - 2022-10-16

- Add combined `impl_as_deref_and_mut!` macro.
- Add `forward_deref_and_mut!` macro.
- Add `impl_as_ref!`, `impl_as_mut!`, `impl_deref!`, and `impl_deref_mut!` invocations for structs other than newtypes.
- Add support in `impl_display_enum!` for variables interpolation.

## 0.0.2 - 2022-09-25

- Add basic `impl_display!` macro for enums.
- Add `impl_from!` and `impl_into!` macros.

## 0.0.1 - 2022-09-25

- Initial release with `impl_as_ref!`, `impl_as_mut!`, `impl_deref!`, and `impl_deref_mut!` macros.
