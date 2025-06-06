# Mnemonic Share Reverse Simulator (MSRS)

MSRS attempts to reconstruct a full BIP39 mnemonic from two Shamir secret shares. The tool brute-forces the missing share, then verifies recovered mnemonics against a target zpub.

## Build requirements

- Rust 1.70 or newer
- Build with `cargo build --release`

## Command line usage

```
cargo run -- --share1 SHARE1 --share2 SHARE2 --zpub ZPUB --progress
```

### Required arguments

- `--share1`: first Shamir mnemonic share or path to file
- `--share2`: second Shamir mnemonic share or path to file

### Optional arguments

- `--zpub`: target BIP84 public key used for verification
- `--threads`: number of worker threads
- `--prng-reuse`: period for PRNG seed reuse
- `--prng-mask`: bit mask applied to PRNG output
- `--index-collision`: probability of share index collision
- `--progress`: show live search progress

Recovered mnemonics and zpub matches are written to `codex-replay.md`.
