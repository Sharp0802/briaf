# Conventions

These conventions apply throughout the repository and take precedence over
conflicting guidance. For everything else, follow
[Rust best practices](https://github.com/apollographql/rust-best-practices).

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
Use `feat` for new features, `fix` for bug fixes, and other appropriate types
such as `docs`, `refactor`, or `chore`.

## Comments and documentation

Keep comments minimal. Use `std`-style documentation: a short action summary and
applicable panic or undefined behaviour conditions. Avoid redundant inline
comments, implementation narratives, and documentation examples.

Use `///` doc comments with `# Panics` and `# Safety` sections where applicable.
Omit sections that do not apply. Do not add an `# Errors` section;
`clippy::missing_errors_doc` is explicitly allowed in the lint policy.

Use **method** for a callable with any `self` receiver, including `self`,
`&self`, and `&mut self`. Use **function** for free functions and associated
functions without a `self` receiver. Substitute the appropriate term in these
templates:

| Case | Template |
| --- | --- |
| Panic on a named state | ``Panics on `<state>`.`` |
| Panic with one condition | `Panics if <condition>.` |
| Panic with multiple conditions | `Panics if any of the following conditions are met:` |
| UB on a named state | ``Calling this method on `<state>` is *[undefined behaviour]*.`` |
| UB with one condition | `Calling this method when <condition> is *[undefined behaviour]*.` |
| UB with multiple conditions | `Calling this method under any of the following conditions is *[undefined behaviour]*:` |

Use `on` for a named state such as `None`, `if` or `when` for a condition
clause, and `under` for the UB introduction to multiple conditions. Follow each
multiple-condition introduction with a blank line and a bullet list containing
one condition per bullet.

```rust
/// Removes the specified range.
///
/// # Panics
///
/// Panics if any of these conditions are met:
///
/// - `start` is greater than `end`.
/// - `end` exceeds the collection's length.
```

Preserve the spelling `undefined behaviour`, the emphasis, and the reference
link. Include its definition in the same doc comment:

```rust
/// Returns the contained value.
///
/// # Safety
///
/// Calling this method on `None` is *[undefined behaviour]*.
///
/// [undefined behaviour]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
```

## Code layout and checks

Spacing, line breaks, logical grouping, and column alignment are part of code
readability. Follow rustfmt for ordinary code and use blank lines to separate
logical groups.

Preserve deliberate column alignment and row grouping in large data arrays,
such as UV, vertex, and index arrays. Apply `#[rustfmt::skip]` to the smallest
declaration containing the array. Keep its formatting consistent by hand;
Clippy still applies. Do not skip an entire function or module to preserve one
array.

The following checks must pass with the repository's configured toolchain:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```
