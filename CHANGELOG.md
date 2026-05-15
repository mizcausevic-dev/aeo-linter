# Changelog

All notable changes to this project are documented here.

## [1.0.0] - 2026-05-15

### Released
- Published **aeo-linter** as a Rust CLI for manifest readiness, entity linkage, source coverage, and answer-surface validation.
- Added sample manifests, JSON and text output, proof screenshots, tests, and CI.
- Positioned the repo as a stricter validation layer for the AEO reference stack.

### Why this mattered
- Valid JSON is not enough for answer-engine trust.
- Teams need fast feedback on whether a manifest is actually citation-ready before they ship it.

## [0.1.0] - 2026-02-28

### Shipped
- Locked the first internal lint rules around source coverage, policy posture, and answer-surface depth.
- Proved that a focused readiness CLI was more useful than a generic manifest inspector.

## [Prototype] - 2025-06-21

### Built
- Built the earliest manifest checks for missing `llm.txt`, weak entity linkage, and unsupported claims.
- Tested scoring patterns against common AEO declaration mistakes.

## [Design Phase] - 2024-10-03

### Designed
- Chose a lint-first CLI instead of another SDK wrapper.
- Treated retrieval quality, citation safety, and policy clarity as first-class rule categories.

## [Idea Origin] - 2023-08-18

### Observed
- Teams could produce AEO-like metadata that looked complete while still being risky for real answer-engine reuse.
- The missing artifact was a linter that graded manifest quality, not just schema validity.
