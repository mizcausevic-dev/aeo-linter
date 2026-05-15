mod linter;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;

use crate::linter::{lint_manifest, LintReport, Manifest};

#[derive(Parser)]
#[command(name = "aeo-linter")]
#[command(
    version,
    about = "Lint AEO manifests for answer-surface and citation readiness"
)]
#[command(
    long_about = "Validate AEO manifests for llm.txt posture, entity linking, claim support coverage, schema hygiene, freshness drift, and answer-surface readiness."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lint a local AEO manifest JSON file.
    Lint {
        /// Path to the manifest file.
        file: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Explain the linter rule set in operator language.
    Explain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{} {:#}", "error:".red().bold(), err);
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Lint { file, format } => cmd_lint(file, format),
        Command::Explain => {
            cmd_explain();
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn cmd_lint(path: PathBuf, format: OutputFormat) -> Result<ExitCode> {
    let raw = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let manifest: Manifest = serde_json::from_str(&raw).context("parsing AEO manifest JSON")?;
    let report = lint_manifest(&manifest);

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_text_report(&path, &report),
    }

    let code = if report.summary.error_count > 0 || report.summary.score < 75 {
        ExitCode::from(2)
    } else if report.summary.warning_count > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    };
    Ok(code)
}

fn cmd_explain() {
    println!("{}", "AEO Linter Rule Set".bold());
    println!("  - llm.txt presence and freshness");
    println!("  - entity identity and sameAs linkage");
    println!("  - claim-to-source evidence coverage");
    println!("  - Schema.org relationship hygiene");
    println!("  - answer-surface depth (FAQ, comparisons, examples)");
    println!("  - citation depth and policy posture");
    println!();
    println!(
        "The goal is not only to validate syntax. It is to make content and metadata easier for answer engines to cite without drifting into hallucinated summaries."
    );
}

fn print_text_report(path: &Path, report: &LintReport) {
    let status = match report.summary.posture.as_str() {
        "ready" => "READY".green().bold(),
        "watch" => "WATCH".yellow().bold(),
        _ => "BLOCKED".red().bold(),
    };
    println!(
        "{} {} — score {} / 100",
        status,
        path.display(),
        report.summary.score
    );
    println!("Entity:           {}", report.manifest.entity.name);
    println!(
        "Canonical URL:    {}",
        report.manifest.canonical_url.dimmed()
    );
    println!(
        "Findings:         {} errors / {} warnings",
        report.summary.error_count, report.summary.warning_count
    );
    println!("Lead issue:       {}", report.summary.lead_issue);
    println!("Recommendation:   {}", report.summary.recommendation);
    println!();
    println!("{}", "Rule results".bold());
    for finding in &report.findings {
        let badge = match finding.severity.as_str() {
            "error" => "error".red().bold(),
            "warning" => "warning".yellow().bold(),
            _ => "pass".green().bold(),
        };
        println!(
            "  [{}] {} — {}",
            badge,
            finding.rule_id.cyan(),
            finding.message
        );
        if !finding.evidence.is_empty() {
            println!("        evidence: {}", finding.evidence.join(" | "));
        }
    }
}
