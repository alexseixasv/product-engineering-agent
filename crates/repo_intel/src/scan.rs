use regex::Regex;
use schemas::{
    Confidence, Evidence, Finding, FindingCategory, validate_finding,
};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, thiserror::Error)]
pub enum RepoIntelError {
    #[error("io: {0}")]
    Io(String),
    #[error("validation: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct DebtSignal {
    pub title: String,
    pub detail: String,
    pub evidence: Evidence,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub debt_signals: Vec<DebtSignal>,
    pub component_paths: Vec<String>,
    pub api_paths: Vec<String>,
    pub model_paths: Vec<String>,
}

fn read_excerpt(path: &Path, start_line: u32, end_line: u32) -> Result<String, RepoIntelError> {
    let content = fs::read_to_string(path).map_err(|e| RepoIntelError::Io(e.to_string()))?;
    let lines: Vec<&str> = content.lines().collect();
    let start = start_line.saturating_sub(1) as usize;
    let end = (end_line as usize).min(lines.len());
    if start >= lines.len() {
        return Err(RepoIntelError::Io(format!(
            "line out of range in {}",
            path.display()
        )));
    }
    Ok(lines[start..end].join("\n"))
}

fn evidence_for(root: &Path, file: &Path, start: u32, end: u32) -> Result<Evidence, RepoIntelError> {
    let rel = file
        .strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");
    let excerpt = read_excerpt(file, start, end)?;
    let evidence = Evidence {
        path: rel,
        start_line: start,
        end_line: end,
        excerpt,
    };
    evidence
        .validate()
        .map_err(RepoIntelError::Validation)?;
    Ok(evidence)
}

fn is_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("ts" | "tsx" | "js" | "jsx" | "rs" | "md")
    )
}

/// Read-only scan of a repository root. Never writes.
pub fn scan_repository(root: impl AsRef<Path>) -> Result<ScanResult, RepoIntelError> {
    let root = root.as_ref();
    let mut findings = Vec::new();
    let mut debt_signals = Vec::new();
    let mut component_paths = Vec::new();
    let mut api_paths = Vec::new();
    let mut model_paths = Vec::new();
    let mut finding_id = 0u32;

    let export_fn = Regex::new(r"export\s+(async\s+)?function\s+(\w+)").expect("regex");
    let export_type = Regex::new(r"export\s+type\s+(\w+)").expect("regex");

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        if rel.contains("node_modules") || rel.contains("dist") || rel.contains("target") {
            continue;
        }
        if !is_source(path) {
            continue;
        }

        if rel.contains("components/ui/") {
            component_paths.push(rel.clone());
            let ev = evidence_for(root, path, 1, 5)?;
            finding_id += 1;
            let finding = Finding {
                id: format!("F{finding_id}"),
                category: FindingCategory::DesignSystem,
                summary: format!("Design system component at {rel}"),
                confidence: Confidence::Fact,
                evidence: vec![ev],
            };
            validate_finding(&finding).map_err(|e| RepoIntelError::Validation(e.to_string()))?;
            findings.push(finding);
        }

        if rel.contains("/api/") {
            api_paths.push(rel.clone());
            let content = fs::read_to_string(path).map_err(|e| RepoIntelError::Io(e.to_string()))?;
            for (idx, line) in content.lines().enumerate() {
                if let Some(cap) = export_fn.captures(line) {
                    let name = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");
                    let line_no = (idx + 1) as u32;
                    let ev = evidence_for(root, path, line_no, line_no)?;
                    finding_id += 1;
                    let finding = Finding {
                        id: format!("F{finding_id}"),
                        category: FindingCategory::Api,
                        summary: format!("API function `{name}` in {rel}"),
                        confidence: Confidence::Fact,
                        evidence: vec![ev],
                    };
                    validate_finding(&finding)
                        .map_err(|e| RepoIntelError::Validation(e.to_string()))?;
                    findings.push(finding);
                }
            }
            if content.contains("from \"@/data/mock\"") || content.contains("from '@/data/mock'") {
                let ev = evidence_for(root, path, 1, 3)?;
                debt_signals.push(DebtSignal {
                    title: "Client API backed by mock data".into(),
                    detail: format!("{rel} imports mock data instead of a real backend"),
                    evidence: ev,
                });
            }
        }

        if rel.contains("/models/") {
            model_paths.push(rel.clone());
            let content = fs::read_to_string(path).map_err(|e| RepoIntelError::Io(e.to_string()))?;
            for (idx, line) in content.lines().enumerate() {
                if let Some(cap) = export_type.captures(line) {
                    let name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                    let line_no = (idx + 1) as u32;
                    let ev = evidence_for(root, path, line_no, line_no)?;
                    finding_id += 1;
                    let finding = Finding {
                        id: format!("F{finding_id}"),
                        category: FindingCategory::Model,
                        summary: format!("Domain model `{name}` in {rel}"),
                        confidence: Confidence::Fact,
                        evidence: vec![ev],
                    };
                    validate_finding(&finding)
                        .map_err(|e| RepoIntelError::Validation(e.to_string()))?;
                    findings.push(finding);
                }
            }
        }

        if rel.ends_with("README.md") {
            let content = fs::read_to_string(path).map_err(|e| RepoIntelError::Io(e.to_string()))?;
            if content.to_lowercase().contains("intentional gaps")
                || content.to_lowercase().contains("no analytics")
            {
                let ev = evidence_for(root, path, 1, 8)?;
                finding_id += 1;
                let finding = Finding {
                    id: format!("F{finding_id}"),
                    category: FindingCategory::Gap,
                    summary: "README documents intentional product gaps".into(),
                    confidence: Confidence::Fact,
                    evidence: vec![ev],
                };
                validate_finding(&finding).map_err(|e| RepoIntelError::Validation(e.to_string()))?;
                findings.push(finding);
            }
        }

    }

    // Gap findings derived from absence of analytics artifacts
    let has_charts = component_paths.iter().any(|p| p.to_lowercase().contains("chart"));
    let has_analytics_page = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .any(|e| {
            e.path()
                .to_string_lossy()
                .to_lowercase()
                .contains("analytics")
        });

    if !has_charts {
        findings.push(Finding {
            id: format!("F{}", finding_id + 1),
            category: FindingCategory::Gap,
            summary: "No chart components found under design system paths".into(),
            confidence: Confidence::Assumption,
            evidence: vec![],
        });
        finding_id += 1;
    }
    if !has_analytics_page {
        findings.push(Finding {
            id: format!("F{}", finding_id + 1),
            category: FindingCategory::Gap,
            summary: "No analytics route/page detected in repository tree".into(),
            confidence: Confidence::Assumption,
            evidence: vec![],
        });
    }

    Ok(ScanResult {
        findings,
        debt_signals,
        component_paths,
        api_paths,
        model_paths,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/sample-app")
    }

    #[test]
    fn scan_fixture_produces_evidenced_facts() {
        let root = fixture_root();
        assert!(root.exists(), "fixture must exist at {:?}", root);
        let result = scan_repository(&root).expect("scan");
        assert!(!result.component_paths.is_empty());
        assert!(!result.api_paths.is_empty());
        assert!(!result.model_paths.is_empty());
        let facts: Vec<_> = result
            .findings
            .iter()
            .filter(|f| f.confidence == Confidence::Fact)
            .collect();
        assert!(!facts.is_empty());
        for f in facts {
            assert!(!f.evidence.is_empty());
            validate_finding(f).unwrap();
        }
        assert!(!result.debt_signals.is_empty());
    }

}
