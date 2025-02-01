# Polkadot Remark Commit

Minimal CLI to remark commit hashes on-chain to permanently persist them in the historical data of Polkadot.

## Usage

Installation happens through crates.io and arguments are passed as environment variables.

```sh
cargo install polkadot-remark-commit --locked
RPC="wss://polkadot-collectives-rpc.polkadot.io" SEED_OR_URI="//Alice"
```

## Key Derivation

Note that this is using Substrate Key derivation for ED25519. Advised approach to configuring a new
key, mnenonic or URI is to dry-run the tool once to check that the output public key is what you
expect. Polkadot supports multiple crypto schemata and different wallets use not only different
crypto schemata but also different key-expansion schemata.

Using `//Alice` as example here:
```pre
remark-commit --rpc ws://127.0.0.1:8000 --seed "//Alice" --org "JamBrains" --repo "graymatter" -c "1fa22191b7298daf4a5045451f8b2e012c036420"
Sending remark extrinsic from 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY to ws://127.0.0.1:8000
```

Now you know that it will always use `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` and you can
fund that with 1 DOT.
