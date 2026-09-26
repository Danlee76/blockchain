# Clippy Pedantic Lints

This project enables Rust's [`clippy::pedantic`](https://docs.rs/clippy/latest/clippy/index.html#pedantic-lints)
lint group to maintain high code quality standards. Pedantic lints catch
stylistic issues and potential logic errors that go beyond the default lints.

## Policy

All `clippy::pedantic` warnings must be either:
1. **Fixed** — adopt the suggested change
2. **Allowed** — document the exception with a `#[allow(...)]` attribute or
   `clippy.toml` configuration

## Rationale

Pedantic lints help prevent:
- Silent logic errors (e.g., unchecked assumptions about data layout)
- Cognitive overhead (unnecessarily complex patterns)
- Maintenance burden (patterns that confuse future readers)
- Performance issues (suboptimal patterns that clippy can flag)

For a financial smart contract, these are especially important — a single
oversight in accounting logic or authorization could have severe consequences.

## Enabling pedantic lints

The project enables pedantic lints in two places:

**CI and local checks** (`.github/workflows/ci.yml` and `Makefile`):
```bash
cargo clippy --all-targets -- -D warnings -W clippy::pedantic
```

**VS Code** (`.vscode/settings.json`, if configured):
See your editor's clippy integration for enabling pedantic lints locally.

## Common pedantic lints

Some pedantic lints frequently applied in this project:

| Lint | Reason | Typical fix |
|------|--------|------------|
| `must_use_candidate` | Function should return `Result` or `Option` | Add `#[must_use]` attribute |
| `missing_errors_doc` | Public function can return error | Add doc comment explaining errors |
| `match_same_arms` | Match arms are identical | Combine with `\|` operator |
| `doc_markdown` | Doc comment formatting | Use proper markdown (backticks, etc.) |
| `implicit_hasher` | Function uses `HashMap` without parameterizing over hasher | Add hasher type parameter (or allow if not applicable) |

## How to allow an exception

If a pedantic lint is not applicable to specific code, use a targeted allow:

```rust
#[allow(clippy::pedantic_lint_name)]
pub fn my_function() { ... }
```

**Always** include a comment explaining why:

```rust
// ALLOW: This pattern is correct for Soroban contracts, which don't support
// async. clippy::async_yields_async is not applicable here.
#[allow(clippy::async_yields_async)]
pub fn my_function() { ... }
```

If many functions need the same exception, configure it in `Cargo.toml`:

```toml
# In contracts/ticketing/Cargo.toml
[package]
# ...

[lints.clippy]
# Soroban doesn't support these patterns
pedantic = "allow"
```

However, prefer targeted exceptions (`#[allow(...)]`) over blanket disables.
