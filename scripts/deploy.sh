#!/usr/bin/env bash
# Deploys the ticketing contract's wasm to the given network.
#
# Usage: scripts/deploy.sh <identity> <network> [--dry-run] [--yes]
#
# Safety (issue #228):
#   --dry-run  prints the unsigned deployment transaction instead of
#              submitting it; useful on any network, submits nothing.
#   mainnet    requires an explicit typed confirmation unless --yes is
#              passed for scripted deployments.
set -euo pipefail
cd "$(dirname "$0")/.."

IDENTITY="${1:?identity required}"
NETWORK="${2:?network required (testnet|futurenet|mainnet)}"
shift 2

DRY_RUN=false
ASSUME_YES=false
while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run) DRY_RUN=true ;;
    --yes) ASSUME_YES=true ;;
    *)
      echo "unknown option: $1" >&2
      echo "usage: scripts/deploy.sh <identity> <network> [--dry-run] [--yes]" >&2
      exit 1
      ;;
  esac
  shift
done

WASM=target/wasm32v1-none/release/stellar_tickets_ticketing.wasm

if [ "$DRY_RUN" = true ]; then
  echo "Dry run: printing the unsigned deployment transaction (nothing is submitted)." >&2
  stellar contract deploy \
    --wasm "$WASM" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    --build-only
  exit 0
fi

if [ "$NETWORK" = mainnet ] && [ "$ASSUME_YES" = false ]; then
  printf 'You are about to deploy to MAINNET. This cannot be undone.\nType "confirm" to continue: '
  read -r ANSWER || ANSWER=""
  if [ "$ANSWER" != "confirm" ]; then
    echo "Aborted: mainnet deployment was not confirmed." >&2
    exit 1
  fi
fi

stellar contract deploy \
  --wasm "$WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK"
