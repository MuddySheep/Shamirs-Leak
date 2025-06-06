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

- `--share1` – first Shamir mnemonic share or a path to a file containing the share words.
- `--share2` – second Shamir mnemonic share or a file path.

### Optional arguments

- `--zpub` – BIP84 extended public key used to verify recovered mnemonics. If omitted, all candidates are accepted.
- `--threads` – number of Rayon worker threads. Defaults to the value in `Config::default()`.
- `--prng-reuse` – period for PRNG seed reuse used by heuristic generation.
- `--prng-mask` – bit mask applied to the PRNG output when generating candidate entropy.
- `--index-collision` – probability in `[0,1]` that the unknown share index collides with a known index.
- `--progress` – display live progress metrics on stdout.

At the end of a run the tool prints a table summarising candidates tested, matches found and the best zpub similarity prefix.

Recovered mnemonics and zpub matches are written to `codex-replay.md`.
