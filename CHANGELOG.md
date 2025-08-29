# Changelog

## Unreleased

## 0.3.0

- Add `impl_newtype_from_into!` macro.
- Syntax for `impl_from_for_primitive!` macro now matches `impl_from!`.

## 0.2.0

- Add `impl_leaf_error!` macro.
- Allow `impl_display_enum!` to omit variant's bindings.
- Remove leaf error support from `impl_error_enum!`.
- Change delimiter after type of `impl_error_enum!` to `:`.
- Change delimiter after type of `impl_display!` and `impl_display_enum!` to `:`.
- Emit `core::error::Error` in `forward_error!` and `impl_error_enum!`.
- Minimum supported Rust version (MSRV) is now 1.58.

## 0.1.9

- No significant changes since `0.1.8`.

## 0.1.8

- Add `impl_display!` macro.

## 0.1.7

- Add `impl_from_for_primitive!` macro.

## 0.1.6

- Add `forward_error` derive for non-generic error wrappers.

## 0.1.5 - 2023-03-04

- Fix `impl_display_enum` comma handling.

## 0.1.4 - 2023-03-04

- Add basic `Error` derive for enums.

## 0.1.3 - 2023-02-24

- Support type parameters in `forward_display!`.

## 0.1.2 - 2023-02-24

- Add `forward_display!` macro.

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
