# Changelog

All notable changes to the Jat on-chain programs are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to follow
semantic versioning once the wire format is frozen for mainnet.

## [Unreleased]

### Changed

- Documentation pass across `README`, `SECURITY`, and `CEREMONY` to use the Jat brand while
  the on-chain program identifiers stay on their deployed values.

## [0.3.0] - 2026-05-30

### Added

- Withdraw path: `withdraw` instruction with a dedicated Groth16 verifying key, recipient
  binding by `Poseidon(hi16, lo16)`, and a global single-use withdraw nullifier.
- Trustless vault payout via `invoke_signed` from the `vault` PDA, no operator key.

### Changed

- Pool root history widened to thirty roots so a proof built against a slightly stale root
  still verifies.

## [0.2.0] - 2026-04-04

### Added

- Shielded pool: on-chain incremental Poseidon (BN254) Merkle tree of depth twenty, fixed
  deposit denominations, and per-context single-use nullifier accounts.
- `seal_verify` instruction other programs can CPI into as a proof-of-receipt gate.

## [0.1.0] - 2026-02-15

### Added

- Announcer program: write-once stealth announcement PDAs keyed by the ephemeral key `R`,
  carrying a one-byte view tag and a scheme byte.
- Initial Anchor workspace, host tests pinning the wire format against the generated IDL.
