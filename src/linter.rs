use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    pub manifest_version: String,
    pub canonical_url: String,
    pub entity: Entity,
    pub llm_txt: Option<LlmTxt>,
    pub headings: Headings,
    pub answer_blocks: AnswerBlocks,
    pub schema_org: SchemaOrg,
    pub citations: Citations,
    pub policy: Policy,
    pub claims: Vec<Claim>,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub same_as: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmTxt {
    pub url: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Headings {
    pub h1_count: u32,
    pub question_heading_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerBlocks {
    pub faq_count: u32,
    pub comparison_blocks: u32,
    pub code_examples: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaOrg {
    pub types: Vec<String>,
    pub linked_entities: u32,
    pub orphaned_entities: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Citations {
    pub external_citations: u32,
    pub primary_sources: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Policy {
    pub allows_ai_citation: bool,
    pub content_license: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claim {
    pub id: String,
    pub predicate: String,
    pub supported_by: Vec<String>,
    pub confidence: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    pub id: String,
    pub url: String,
    pub last_updated: String,
    pub authoritative: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct LintReport {
    pub manifest: ReportManifest,
    pub summary: ReportSummary,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportManifest {
    pub entity: Entity,
    pub canonical_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportSummary {
    pub posture: String,
    pub score: i32,
    pub error_count: usize,
    pub warning_count: usize,
    pub lead_issue: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
    pub evidence: Vec<String>,
}

pub fn lint_manifest(manifest: &Manifest) -> LintReport {
    let findings = vec![
        check_llm_txt(manifest),
        check_entity_linkage(manifest),
        check_claim_sources(manifest),
        check_schema_org(manifest),
        check_answer_blocks(manifest),
        check_citation_depth(manifest),
        check_policy_posture(manifest),
        check_source_freshness(manifest),
    ];

    let error_count = findings.iter().filter(|f| f.severity == "error").count();
    let warning_count = findings.iter().filter(|f| f.severity == "warning").count();
    let mut score = 100;
    for finding in &findings {
        match finding.severity.as_str() {
            "error" => score -= 15,
            "warning" => score -= 6,
            _ => {}
        }
    }
    score = score.clamp(0, 100);

    let posture = if error_count > 0 || score < 75 {
        "blocked"
    } else if warning_count > 0 {
        "watch"
    } else {
        "ready"
    };

    let lead_issue = findings
        .iter()
        .find(|f| f.severity == "error")
        .or_else(|| findings.iter().find(|f| f.severity == "warning"))
        .map(|f| f.message.clone())
        .unwrap_or_else(|| {
            "Manifest is structurally ready for answer-engine consumption.".to_string()
        });

    let recommendation = if posture == "ready" {
        "Publish the manifest and keep source freshness monitoring in the CI path.".to_string()
    } else if posture == "watch" {
        "Resolve the warning-tier gaps before relying on this manifest for high-trust AI citation flows.".to_string()
    } else {
        "Fix the blocking gaps before treating this manifest as trustworthy answer-engine infrastructure.".to_string()
    };

    LintReport {
        manifest: ReportManifest {
            entity: manifest.entity.clone(),
            canonical_url: manifest.canonical_url.clone(),
        },
        summary: ReportSummary {
            posture: posture.to_string(),
            score,
            error_count,
            warning_count,
            lead_issue,
            recommendation,
        },
        findings,
    }
}

fn check_llm_txt(manifest: &Manifest) -> Finding {
    match &manifest.llm_txt {
        None => Finding {
            rule_id: "llm_txt_presence".to_string(),
            severity: "error".to_string(),
            message: "Manifest is missing llm.txt discovery metadata.".to_string(),
            evidence: vec!["llm_txt field not found".to_string()],
        },
        Some(llm) => {
            let age = days_since(&llm.last_updated);
            if age > 120 {
                Finding {
                    rule_id: "llm_txt_presence".to_string(),
                    severity: "warning".to_string(),
                    message: "llm.txt is present but its freshness signal is aging out."
                        .to_string(),
                    evidence: vec![format!(
                        "last_updated={} ({} days old)",
                        llm.last_updated, age
                    )],
                }
            } else {
                Finding {
                    rule_id: "llm_txt_presence".to_string(),
                    severity: "pass".to_string(),
                    message: "llm.txt discovery metadata is present and fresh.".to_string(),
                    evidence: vec![llm.url.clone()],
                }
            }
        }
    }
}

fn check_entity_linkage(manifest: &Manifest) -> Finding {
    if manifest.entity.id.is_empty() || manifest.entity.same_as.len() < 2 {
        return Finding {
            rule_id: "entity_identity_linkage".to_string(),
            severity: "error".to_string(),
            message: "Entity identity is too thin for high-trust answer attribution.".to_string(),
            evidence: vec![
                format!("entity_id_present={}", !manifest.entity.id.is_empty()),
                format!("same_as_count={}", manifest.entity.same_as.len()),
            ],
        };
    }
    Finding {
        rule_id: "entity_identity_linkage".to_string(),
        severity: "pass".to_string(),
        message: "Entity identity includes reusable sameAs linkage.".to_string(),
        evidence: vec![format!("same_as_count={}", manifest.entity.same_as.len())],
    }
}

fn check_claim_sources(manifest: &Manifest) -> Finding {
    let source_ids: std::collections::HashSet<&str> = manifest
        .sources
        .iter()
        .map(|source| source.id.as_str())
        .collect();
    let unsupported = manifest
        .claims
        .iter()
        .filter(|claim| {
            claim.supported_by.is_empty()
                || claim
                    .supported_by
                    .iter()
                    .any(|source_id| !source_ids.contains(source_id.as_str()))
        })
        .count();
    if unsupported > 0 {
        return Finding {
            rule_id: "claim_source_coverage".to_string(),
            severity: "error".to_string(),
            message: "At least one claim does not map cleanly to a declared supporting source."
                .to_string(),
            evidence: vec![
                format!("claims={}", manifest.claims.len()),
                format!("unsupported_claims={unsupported}"),
            ],
        };
    }
    Finding {
        rule_id: "claim_source_coverage".to_string(),
        severity: "pass".to_string(),
        message: "Every claim references a declared source.".to_string(),
        evidence: vec![format!("claims={}", manifest.claims.len())],
    }
}

fn check_schema_org(manifest: &Manifest) -> Finding {
    if manifest.schema_org.orphaned_entities > 0 || manifest.schema_org.linked_entities < 2 {
        return Finding {
            rule_id: "schema_org_hygiene".to_string(),
            severity: "warning".to_string(),
            message: "Schema.org graph still has weak relationship hygiene.".to_string(),
            evidence: vec![
                format!("linked_entities={}", manifest.schema_org.linked_entities),
                format!(
                    "orphaned_entities={}",
                    manifest.schema_org.orphaned_entities
                ),
            ],
        };
    }
    Finding {
        rule_id: "schema_org_hygiene".to_string(),
        severity: "pass".to_string(),
        message: "Schema.org graph is linked tightly enough for citation reuse.".to_string(),
        evidence: vec![format!("types={}", manifest.schema_org.types.join(","))],
    }
}

fn check_answer_blocks(manifest: &Manifest) -> Finding {
    let answer_surface = manifest.answer_blocks.faq_count
        + manifest.answer_blocks.comparison_blocks
        + manifest.answer_blocks.code_examples;
    if manifest.headings.h1_count != 1
        || manifest.headings.question_heading_count < 2
        || answer_surface < 4
    {
        return Finding {
            rule_id: "answer_surface_depth".to_string(),
            severity: "warning".to_string(),
            message: "Answer-surface structure is still thin for citation-friendly retrieval."
                .to_string(),
            evidence: vec![
                format!("h1_count={}", manifest.headings.h1_count),
                format!(
                    "question_heading_count={}",
                    manifest.headings.question_heading_count
                ),
                format!("answer_surface_blocks={answer_surface}"),
            ],
        };
    }
    Finding {
        rule_id: "answer_surface_depth".to_string(),
        severity: "pass".to_string(),
        message: "Heading and answer-block coverage is strong enough for answer-engine extraction."
            .to_string(),
        evidence: vec![format!("answer_surface_blocks={answer_surface}")],
    }
}

fn check_citation_depth(manifest: &Manifest) -> Finding {
    if manifest.citations.primary_sources < 2 || manifest.citations.external_citations < 4 {
        return Finding {
            rule_id: "citation_depth".to_string(),
            severity: "warning".to_string(),
            message: "Citation depth is still shallow for high-confidence answer reuse."
                .to_string(),
            evidence: vec![
                format!("primary_sources={}", manifest.citations.primary_sources),
                format!(
                    "external_citations={}",
                    manifest.citations.external_citations
                ),
            ],
        };
    }
    Finding {
        rule_id: "citation_depth".to_string(),
        severity: "pass".to_string(),
        message: "Citation depth is strong enough to support answer-engine grounding.".to_string(),
        evidence: vec![
            format!("primary_sources={}", manifest.citations.primary_sources),
            format!(
                "external_citations={}",
                manifest.citations.external_citations
            ),
        ],
    }
}

fn check_policy_posture(manifest: &Manifest) -> Finding {
    if !manifest.policy.allows_ai_citation || manifest.policy.content_license.trim().is_empty() {
        return Finding {
            rule_id: "policy_posture".to_string(),
            severity: "error".to_string(),
            message: "Policy metadata still blocks or obscures AI citation reuse.".to_string(),
            evidence: vec![
                format!("allows_ai_citation={}", manifest.policy.allows_ai_citation),
                format!("content_license={}", manifest.policy.content_license),
            ],
        };
    }
    Finding {
        rule_id: "policy_posture".to_string(),
        severity: "pass".to_string(),
        message: "Policy metadata explicitly permits answer-engine citation flows.".to_string(),
        evidence: vec![manifest.policy.content_license.clone()],
    }
}

fn check_source_freshness(manifest: &Manifest) -> Finding {
    let stale_sources = manifest
        .sources
        .iter()
        .filter(|source| days_since(&source.last_updated) > 365 && source.authoritative)
        .count();
    if stale_sources > 0 {
        return Finding {
            rule_id: "source_freshness".to_string(),
            severity: "warning".to_string(),
            message: "Authoritative sources are drifting beyond comfortable freshness bounds."
                .to_string(),
            evidence: vec![format!("stale_authoritative_sources={stale_sources}")],
        };
    }
    Finding {
        rule_id: "source_freshness".to_string(),
        severity: "pass".to_string(),
        message: "Authoritative sources are fresh enough for current answer-engine trust."
            .to_string(),
        evidence: vec![format!("sources={}", manifest.sources.len())],
    }
}

fn days_since(value: &str) -> i64 {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(|date| (Utc::now().date_naive() - date).num_days())
        .unwrap_or(9_999)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy_manifest() -> Manifest {
        serde_json::from_str(
            r#"{
              "manifest_version": "0.1",
              "canonical_url": "https://example.com/aeo",
              "entity": {
                "id": "https://example.com/#org",
                "name": "Example Org",
                "entity_type": "Organization",
                "same_as": ["https://linkedin.com/company/example", "https://github.com/example"]
              },
              "llm_txt": {"url": "https://example.com/llm.txt", "last_updated": "2026-05-01"},
              "headings": {"h1_count": 1, "question_heading_count": 3},
              "answer_blocks": {"faq_count": 3, "comparison_blocks": 1, "code_examples": 1},
              "schema_org": {"types": ["Organization", "FAQPage"], "linked_entities": 3, "orphaned_entities": 0},
              "citations": {"external_citations": 6, "primary_sources": 3},
              "policy": {"allows_ai_citation": true, "content_license": "CC-BY-4.0"},
              "claims": [
                {"id": "claim-1", "predicate": "aeo:serviceArea", "supported_by": ["src-1"], "confidence": "high"}
              ],
              "sources": [
                {"id": "src-1", "url": "https://example.com/about", "last_updated": "2026-04-12", "authoritative": true}
              ]
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn healthy_manifest_is_ready() {
        let report = lint_manifest(&healthy_manifest());
        assert_eq!(report.summary.posture, "ready");
        assert_eq!(report.summary.error_count, 0);
    }

    #[test]
    fn missing_policy_blocks_manifest() {
        let mut manifest = healthy_manifest();
        manifest.policy.allows_ai_citation = false;
        let report = lint_manifest(&manifest);
        assert_eq!(report.summary.posture, "blocked");
        assert!(report.summary.error_count >= 1);
    }
}
