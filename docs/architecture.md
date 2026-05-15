# AEO Linter Architecture

## Intent

This repo validates whether an AEO manifest is actually ready for real
answer-engine consumption, not just whether it parses.

It focuses on:

- `llm.txt` discovery posture
- entity identity and `sameAs` depth
- claim-to-source evidence coverage
- Schema.org relationship hygiene
- answer-surface structure
- citation depth
- AI citation policy clarity
- source freshness

## Flow

1. `samples/*.json` provide ready and problematic manifest fixtures.
2. `src/main.rs` parses CLI commands and handles text or JSON output.
3. `src/linter.rs` deserializes manifests and runs the lint rule set.
4. `tests/cli.rs` verifies ready and failing manifest behavior through the built binary.
5. `scripts/generate_screenshots.py` runs real CLI commands and captures proof screenshots for the README.

## CLI surface

- `aeo-linter lint <file>`
  - Text summary plus rule findings
- `aeo-linter lint <file> --format json`
  - Machine-readable lint report
- `aeo-linter explain`
  - Human-readable overview of the rule categories

## Validation

- `cargo test`
- `cargo build`
- `py -3.11 .\scripts\generate_screenshots.py`
