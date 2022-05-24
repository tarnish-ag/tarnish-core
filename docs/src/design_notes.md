# Package Structure
This repo is organized as a _Rust_ crate with a library target, which allows us to implement the `tarnish` language.

# Features
`Core` should be
- Explicitly Typed
- Future Conscious

#
> ___<span style="font-variant:small-caps;">notes on naming</span>___
> The repository is named `core`. However, as it conflicts with _Rust_'s [`core`](https://doc.rust-lang.org/core/) library, the root crate is named in _kebab-case_ as `tarnish-core` and the [_LALRPOP_](https://github.com/lalrpop/lalrpop) grammar and its corresponding module is named in _snake_case_ as `tarnish_core` per [_Rust_'s naming conventions](https://rust-lang.github.io/api-guidelines/naming.html).
> When referred to in the `tarnish` language, we will follow its conventions (TBD) instead, which is `Core` at time of writing.