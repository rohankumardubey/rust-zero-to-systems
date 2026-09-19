//! Validate the repository's machine-readable concept coverage manifest.

use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    rust_version: String,
    rust_edition: String,
    last_reviewed: String,
    #[serde(default)]
    concept: Vec<Concept>,
}

#[derive(Debug, Deserialize)]
struct Concept {
    id: String,
    category: String,
    level: String,
    status: String,
    stability: String,
    source: Option<String>,
    docs: Option<String>,
    #[serde(default)]
    prerequisites: Vec<String>,
    #[serde(default)]
    related: Vec<String>,
    tested: bool,
}

fn main() {
    let manifest_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "coverage/manifest.toml".to_owned());
    let manifest_path = PathBuf::from(manifest_path);
    let repo_root = manifest_path
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("."));

    match validate_file(&manifest_path, repo_root) {
        Ok(summary) => println!("{summary}"),
        Err(errors) => {
            eprintln!("coverage validation failed with {} error(s):", errors.len());
            for error in errors {
                eprintln!("- {error}");
            }
            std::process::exit(1);
        }
    }
}

fn validate_file(path: &Path, repo_root: &Path) -> Result<String, Vec<String>> {
    let input = fs::read_to_string(path)
        .map_err(|error| vec![format!("could not read {}: {error}", path.display())])?;
    let manifest: Manifest = toml::from_str(&input)
        .map_err(|error| vec![format!("could not parse {}: {error}", path.display())])?;
    validate(&manifest, repo_root)
}

fn validate(manifest: &Manifest, repo_root: &Path) -> Result<String, Vec<String>> {
    let mut errors = Vec::new();

    if manifest.schema_version != 1 {
        errors.push(format!(
            "unsupported schema_version {}; expected 1",
            manifest.schema_version
        ));
    }
    if manifest.rust_version.trim().is_empty()
        || manifest.rust_edition.trim().is_empty()
        || manifest.last_reviewed.trim().is_empty()
    {
        errors.push("manifest metadata must not be empty".to_owned());
    }

    let mut ids = HashSet::new();
    for concept in &manifest.concept {
        if !ids.insert(concept.id.as_str()) {
            errors.push(format!("{}: duplicate concept id", concept.id));
        }
        validate_concept(concept, repo_root, &mut errors);
    }

    for concept in &manifest.concept {
        for dependency in concept.prerequisites.iter().chain(&concept.related) {
            if !ids.contains(dependency.as_str()) {
                errors.push(format!(
                    "{}: reference {dependency:?} is not declared",
                    concept.id
                ));
            }
        }
    }

    validate_book_navigation(manifest, repo_root, &mut errors);

    if errors.is_empty() {
        let complete = manifest
            .concept
            .iter()
            .filter(|concept| concept.status == "complete")
            .count();
        let total = manifest.concept.len();
        let per_mille = complete
            .saturating_mul(1_000)
            .saturating_add(total / 2)
            .checked_div(total)
            .unwrap_or(0);
        Ok(format!(
            "coverage manifest valid: {complete}/{total} inventoried concepts complete ({}.{:01}%)",
            per_mille / 10,
            per_mille % 10
        ))
    } else {
        Err(errors)
    }
}

fn validate_concept(concept: &Concept, repo_root: &Path, errors: &mut Vec<String>) {
    if !valid_id(&concept.id) {
        errors.push(format!("{}: invalid dotted concept id", concept.id));
    }
    if concept.category.trim().is_empty() {
        errors.push(format!("{}: category must not be empty", concept.id));
    }
    if !matches!(
        concept.level.as_str(),
        "beginner" | "intermediate" | "advanced"
    ) {
        errors.push(format!("{}: invalid level {:?}", concept.id, concept.level));
    }
    if !matches!(
        concept.status.as_str(),
        "planned" | "in_progress" | "complete" | "blocked"
    ) {
        errors.push(format!(
            "{}: invalid status {:?}",
            concept.id, concept.status
        ));
    }
    if !matches!(
        concept.stability.as_str(),
        "stable" | "nightly" | "experimental" | "deprecated" | "ecosystem" | "historical"
    ) {
        errors.push(format!(
            "{}: invalid stability {:?}",
            concept.id, concept.stability
        ));
    }
    if concept.status == "complete" && !concept.tested {
        errors.push(format!("{}: complete concept must be tested", concept.id));
    }
    if concept.status == "complete" && (concept.source.is_none() || concept.docs.is_none()) {
        errors.push(format!(
            "{}: complete concept needs source and documentation",
            concept.id
        ));
    }
    for (kind, declared) in [("source", &concept.source), ("docs", &concept.docs)] {
        if let Some(relative) = declared {
            validate_path(repo_root, &concept.id, kind, relative, errors);
        }
    }
}

fn validate_book_navigation(manifest: &Manifest, repo_root: &Path, errors: &mut Vec<String>) {
    let summary_path = repo_root.join("book/src/SUMMARY.md");
    let Ok(summary) = fs::read_to_string(&summary_path) else {
        errors.push(format!("could not read {}", summary_path.display()));
        return;
    };
    for concept in &manifest.concept {
        if let Some(docs) = &concept.docs {
            let book_path = docs.strip_prefix("book/src/").unwrap_or(docs);
            if !summary.contains(book_path) {
                errors.push(format!(
                    "{}: documentation is missing from book/src/SUMMARY.md",
                    concept.id
                ));
            }
        }
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('.').all(|part| {
            !part.is_empty()
                && part.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
        })
}

fn validate_path(repo_root: &Path, id: &str, kind: &str, relative: &str, errors: &mut Vec<String>) {
    let path = Path::new(relative);
    let safe = !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)));
    if !safe {
        errors.push(format!(
            "{id}: {kind} path must be a safe repository-relative path"
        ));
    } else if !repo_root.join(path).is_file() {
        errors.push(format!("{id}: missing {kind} file {relative:?}"));
    }
}

#[cfg(test)]
mod tests {
    use super::valid_id;

    #[test]
    fn accepts_dotted_ids() {
        assert!(valid_id("ownership.partial_moves"));
    }

    #[test]
    fn rejects_unsafe_or_ambiguous_ids() {
        assert!(!valid_id("Ownership.Move"));
        assert!(!valid_id("ownership..move"));
        assert!(!valid_id("ownership/move"));
    }
}
