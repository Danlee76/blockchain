# Deployment checklist

1. `cargo test --workspace` — full suite green
2. `cargo clippy --all-targets -- -D warnings` — no lint warnings
3. `stellar contract build` — optimized wasm builds cleanly
4. Deploy to **testnet** first; run through the full ticket lifecycle
   manually (create event, issue, verify, check in, list, buy resale)
   against a real testnet SEP-41 token
5. Only after a testnet soak period, deploy to **mainnet** with a
   production payment token and a hardware-backed admin key
6. Record the deployed contract ID in the backend's
   `TICKETING_CONTRACT_ID` environment variable

## Multisig admin

On mainnet the contract admin should be a multisig account rather than a
single key. Configure it before calling `initialize`:

```
scripts/setup-multisig-admin.sh <admin-identity> <network> <threshold> <signer-address>...
```

Run it on testnet first and confirm an admin call (for example
`set_purchase_throttle`) needs `<threshold>` signatures before doing the same
on mainnet.

## deploy.sh safety prompts (issue #228)

`scripts/deploy.sh <identity> <network>` guards against accidental use:

- **Dry run** — pass `--dry-run` to print the unsigned deployment
  transaction (base64 XDR, via `stellar contract deploy --build-only`)
  without submitting anything. Run it first to sanity-check the identity,
  network and wasm before a real deploy on any network:

  ```
  scripts/deploy.sh <identity> mainnet --dry-run
  ```

- **Mainnet confirmation** — a real mainnet deploy pauses and requires
  typing `confirm` at the prompt. Testnet/futurenet deployments submit
  without a prompt.

- **Scripted deploys** — pass `--yes` to skip the mainnet confirmation
  once the deployment has been reviewed (e.g. in automation).

The wasm path, identity and network are unchanged from the original script.
