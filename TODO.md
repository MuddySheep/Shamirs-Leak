# 📋 MSRS Development TODOs – Codex-Targetable Blueprint

This task map defines every Rust module and the specific functionality it must implement. The focus is on **breaking the JS-based Shamir secret splitting implementation**, exploiting entropy patterns, index leakage, and checksum override logic.

---

## 🔥 Core Objectives

🎯 **Endgame**: Reconstruct the full BIP39 mnemonic using only 2 of 3 Shamir shares
🎯 **Target**: JS implementation flaws (in randomness, GF(256) operations, share-index logic)

---

## 🔧 src/main.rs

* [ ] Accept CLI input for known shares (2 out of 3)
* [ ] Dispatch agent execution pipeline (entropy → forge → synth → verify)

---

## 🧠 src/lib.rs

* [ ] Wire up modular calls to each agent (run pipeline from main)
* [ ] Prepare `pub use` statements for exposing tool to CLI/WASM

---

## ⚙️ src/config.rs

* [ ] Define configurable params: thread count, share space depth, search limits
* [ ] Optional: Load from config file or CLI args

---

## 🛠 src/utils.rs

* [ ] Add byte-to-word index mapper (use BIP39 wordlist)
* [ ] Implement zpub → xpub converter for HD path tests
* [ ] Logging formatter for terminal & file output

---

## 🔐 src/entropy/prng.rs

* [ ] Reproduce JS entropy generator logic (byte-wise via `Math.random()` style)
* [ ] Simulate seed reuse / low-entropy cycles
* [ ] Score entropy states by match-likelihood vs given shares

---

## 🧮 src/shamir/gf256.rs

* [ ] Implement full GF(256) math:

  * [ ] exp/log table precompute
  * [ ] addition, multiplication, division, pow
* [ ] Validate math against JS functions (`__GF256_mul`, `__shamirFn`, etc.)

---

## 🔓 src/shamir/reconstruct.rs

* [ ] Take 2 shares + candidate 3rd
* [ ] Interpolate bytewise over GF(256)
* [ ] Return full mnemonic if valid, else error

---

## 🔢 src/bip39/checksum.rs

* [ ] Implement BIP39 checksum logic from scratch
* [ ] Allow brute-force final word adjustment for manual generation
* [ ] Support "auto-correction" fallback like the JS version permits

---

## 🌱 src/bip39/seed.rs

* [ ] Convert full 12-word mnemonic to seed
* [ ] Derive HD keys using `m/84'/0'/0'/0/0`
* [ ] Match output against known zpub

---

## 🔍 src/search/brute.rs

* [ ] Generate plausible 3rd shares (GF(256)-valid values)
* [ ] Interface with `reconstruct` + `checksum` to validate mnemonic
* [ ] Use Rayon for multithreaded search space

---

## 🧠 src/search/heuristics.rs

* [ ] Detect patterns in byte distributions of shares
* [ ] Limit candidate pool via entropy model from `prng.rs`
* [ ] Implement "smart sampling" and mutate top results recursively



msrs/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── utils.rs
│   ├── entropy/
│   │   └── prng.rs
│   ├── shamir/
│   │   ├── gf256.rs
│   │   └── reconstruct.rs
│   ├── bip39/
│   │   ├── checksum.rs
│   │   └── seed.rs
│   └── search/
│       ├── brute.rs
│       └── heuristics.rs

