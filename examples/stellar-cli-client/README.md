# stellar-cli-client example

Standalone Rust example client (issue #226). Instead of pulling in an SDK
crate, it wraps the [stellar CLI]: every interaction is a `stellar contract
invoke` run as a subprocess, so the example itself has zero dependencies.

## Prerequisites

- The `stellar` CLI on `PATH`
- A funded testnet identity (`stellar keys generate --fund <name> --network testnet`)
- A deployed, initialized contract with a created event — see
  `scripts/examples/README.md` for the full setup

## Usage

```bash
cargo run --manifest-path examples/stellar-cli-client/Cargo.toml -- \
  <CONTRACT_ID> <organizer-identity>
```

Optional arguments: `[event-id]` (default `1`) and `[network]` (default
`testnet`).

## What it does

1. Resolves the organizer identity to its public key (`stellar keys address`)
   and uses it as the ticket buyer.
2. Submits `issue_ticket` on the given network, signed with the organizer
   identity, and prints the new ticket id.
3. Reads the ticket back with a simulated `verify_ticket` call
   (`--send no`) and prints the result.

[stellar CLI]: https://developers.stellar.org/docs/tools/cli/stellar-cli
