# Cargo Profiles

This document explains the optimization profiles used in the `ticketing` contract
and why each setting matters for Soroban smart contracts.

## Profile configuration

The workspace defines two optimized profiles in [`Cargo.toml`](../Cargo.toml):

### release

The default release profile used for production deployments:

```toml
[profile.release]
opt-level = "z"          # Optimize aggressively for size
overflow-checks = true   # Catch integer overflows at runtime
debug = 0                # Strip all debug info
strip = "symbols"        # Remove symbol table
debug-assertions = false # No runtime assertions
panic = "abort"          # Abort on panic (no unwinding)
codegen-units = 1        # Maximizes optimization
lto = true               # Link-time optimization
```

**Rationale:**
- `opt-level = "z"`: Soroban contracts are deployed as WebAssembly. Smaller code
  means lower deployment costs and faster execution. This is the tightest
  optimization.
- `codegen-units = 1`: Forces LLVM to treat the entire crate as one compilation
  unit, enabling more aggressive inter-procedural optimizations.
- `lto = true`: Link-time optimization can eliminate dead code and inline across
  boundaries, further reducing binary size.
- `panic = "abort"`: Reduces code size by omitting unwinding machinery.
- `overflow-checks = true`: Critical for financial contracts — overflows in ticket
  pricing or royalty calculations must be detected, not silently wrapped.

See [Soroban best practices](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/logging#cargotoml-profile)
for detailed rationale.

### release-with-logs

A variant of `release` that enables debug assertions for testing:

```toml
[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```

Use this profile locally when debugging:

```bash
cargo build --profile release-with-logs
```

## WASM binary size

The optimizations above significantly reduce the compiled contract size:

- **unoptimized (debug)**: ~1.2 MB
- **release profile**: ~100-150 KB
- **gzipped release**: ~30-50 KB

Smaller binaries mean:
- Lower deployment costs (charged per byte on Soroban)
- Faster network propagation
- Better user experience (faster contract invocations)

To check the current binary size:

```bash
stellar contract build
ls -lh target/wasm32v1-none/release/stellar_tickets_ticketing.wasm
```
