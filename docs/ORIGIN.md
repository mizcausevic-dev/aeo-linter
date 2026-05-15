# Why We Built This

**aeo-linter** started from a simple but costly observation: answer-engine
manifests can look complete long before they are actually trustworthy. Teams can
publish `llm.txt`, entity metadata, JSON-LD, and a few claims, then assume they
have built something reusable for retrieval systems. In practice, the real
problems show up one layer deeper. Claims lack declared supporting sources,
entity identity is too thin to support confident citation, heading structure is
weak for extraction, or policy metadata never makes it clear whether AI systems
are allowed to reuse the material at all.

Existing tools help with validation, but they often stop too early. Schema
checks catch malformed JSON. Crawlers can verify discovery paths. SDKs make it
easier to parse and emit protocol documents. None of that guarantees that the
manifest is genuinely strong for answer-engine use. The missing step is a lint
layer that treats citation safety, evidence coverage, and answer-surface quality
as product concerns instead of optional polish.

That is why **aeo-linter** is intentionally narrow. It does not try to be a
fetch tool, crawler, or general protocol workbench. It focuses on the higher
trust questions:

- is `llm.txt` discoverable and still fresh
- is the entity link graph strong enough to disambiguate citations
- do claims map to declared sources cleanly
- is Schema.org relationship hygiene strong enough to avoid orphaned entities
- are the answer blocks and headings deep enough for extraction
- does policy metadata explicitly support AI citation

The design philosophy is straightforward:

- **lint-first** so it fits directly into local checks and CI
- **operator-readable** so output can guide action without another dashboard
- **AEO-native** so the rule set maps to real answer-engine concerns, not generic SEO heuristics
- **strict where trust matters** so manifests fail fast when evidence posture is weak

The sample manifests in this repo are deliberate. One is fully ready, one is
problematic. That contrast keeps the tool grounded in the kinds of mistakes
teams actually make when they publish metadata too early.

Next on the roadmap is richer manifest import support, weighted rule profiles
for different publisher types, and optional SARIF-style output so findings can
travel more naturally through enterprise code-review and compliance workflows.
