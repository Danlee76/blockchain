# Fees

Every write to this contract is a regular Soroban transaction and
pays the network's standard resource fee — there is no separate
"platform fee" charged by the contract itself. The only value the
contract ever moves beyond the caller's own transaction fee is the
`payment_token` amounts in `purchase_primary` and `buy_resale`
(primary sale proceeds and resale royalty/seller proceeds).

Callers should simulate before submitting (`simulateTransaction` /
`prepareTransaction` in the SDK) to get an accurate resource fee for
the specific operation, since fees scale with the footprint touched —
`issue_ticket` and `buy_resale` touch more storage than a simple
`verify_ticket` read.

## Test budget thresholds (issue #227)

The ticketing tests assert that representative operations stay under fixed
resource thresholds so cost regressions fail loudly instead of silently. The
limits below sit far above the values currently measured in the native-Rust
test environment — they are regression guards, not network limits.

| Operation        | CPU instruction cost limit | Memory bytes cost limit |
| ---------------- | -------------------------- | ------------------------|
| `issue_ticket`   | 10,000,000                 | 1,000,000               |
| `verify_ticket`  | 10,000,000                 | 1,000,000               |
| `buy_resale`     | 10,000,000                 | 1,000,000               |

The values are read from `env.cost_estimate().budget()` in
`contracts/ticketing/src/test.rs`. The SDK resets budget metering before every
top-level contract invocation, so each assertion meters exactly the one call
its test names.

Two caveats: the test environment executes contracts as native Rust, which
underestimates the CPU and memory a real WASM invocation consumes; and the
per-transaction limits actually enforced on-chain remain whatever the network
configuration sets, independent of these thresholds.
