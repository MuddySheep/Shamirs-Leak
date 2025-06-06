🔐 MSRS Agent Guide
This document outlines the structure, roles, and coordination logic for the modular cryptanalysis agents within the Mnemonic Share Reverse Simulator (MSRS). These agents work in tandem to reverse-engineer or simulate BIP39 mnemonics protected by Shamir's Secret Sharing Scheme (SSSS).

Repository Overview
Rust 1.70+ toolchain using Cargo. Modules live in src/ with clean separation for entropy, BIP39, SSSS, and brute force logic.

Agent-Oriented Design: Each task in the share synthesis pipeline is encapsulated as a programmable agent, designed for recursive learning and CLI/WASM integration.

Concurrency: The framework is built to scale with threads or distributed networks.

Key agent components:

EntropyProfilerAgent – entropy modeling and PRNG simulation

ChecksumForgeAgent – brute-forces and corrects BIP39 checksums

PolynomialSynthAgent – simulates and interpolates partial GF(256) polynomials

HDPathVerifierAgent – verifies keys against known zpubs using seed-derived paths

Agent Roles
🧬 EntropyProfilerAgent
Purpose: Model the entropy source used to generate mnemonic shares.

Simulates PRNG or dice roll logic

Detects patterns, entropy leaks, or seed reuse

Attempts to reverse entropy sequences from known shares

Example Task:

rust
Copy
Edit
simulate_entropy_source()
🔎 ChecksumForgeAgent
Purpose: Force the correct BIP39 checksum when brute-forcing the last word of the mnemonic.

Applies BIP39 checksum rules to incomplete mnemonics

Narrows down candidates by computing derived seed and comparing against known HD outputs

Example Task:

rust
Copy
Edit
validate_checksum()
🧪 PolynomialSynthAgent
Purpose: Interpolate or generate plausible third Shamir share using GF(256) math.

Uses Lagrange interpolation over byte shares

Applies heuristic pruning to limit third share space

Recovers the full mnemonic secret if match is valid

Example Task:

rust
Copy
Edit
attempt_reconstruction()
🔐 HDPathVerifierAgent
Purpose: Validate that a recovered mnemonic maps to a known public key.

Uses standard BIP32/BIP84 path derivation (m/84'/0'/0'/0/0)

Confirms derived key or address matches provided Zpub

Example Task:

rust
Copy
Edit
derive_seed()
Simulation Flow
Each agent passes intermediate state to the next:

cpp
Copy
Edit
EntropyProfilerAgent
    → candidate entropy states
    → ChecksumForgeAgent
        → valid mnemonics
        → PolynomialSynthAgent
            → reconstructed secrets
            → HDPathVerifierAgent
                → final validation against zpub
Agents can be run recursively, in batch, or interactively via CLI or distributed mode (future).

Contributing
Code in Rust using idiomatic async/task/thread-safe patterns.

Each agent lives in its own module and can be run independently.

Traits for agents may be extracted to define a common interface for orchestration.

Tests for agents go in tests/ with seed/entropy fixtures.
