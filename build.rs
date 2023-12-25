use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use csv::{ReaderBuilder, Trim, WriterBuilder};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    full_name: String,

    #[serde(default)]
    abbreviation_1: Option<String>,
    #[serde(default)]
    abbreviation_2: Option<String>,
    #[serde(default)]
    abbreviation_3: Option<String>,
}

enum Order {
    Dots,
    Dotless,
}

impl Order {
    fn file_suffixes(&self) -> &[&str] {
        match self {
            Order::Dots => &[
                "acs",
                "ams",
                "general",
                "geology_physics",
                "ieee",
                "lifescience",
                "mathematics",
                "mechanical",
                "meteorology",
                "sociology",
                "webofscience-dots",
            ],
            Order::Dotless => &["entrez", "medicus", "webofscience-dotless"],
        }
    }
}

fn main() -> Result<()> {
    let repo_dir = clone_repo()?;
    let import_order = if cfg!(feature = "dot") {
        Order::Dots
    } else {
        Order::Dotless
    };
    let journals = process_csv_files(&repo_dir, import_order)?;
    let _output_filename = if cfg!(feature = "dot") {
        "journalList_dots.csv"
    } else {
        "journalList_dotless.csv"
    };
    // write_journals_to_csv(&journals, output_filename)?;
    write_journals_to_static_array(&journals)?;
    Ok(())
}

fn clone_repo() -> Result<PathBuf> {
    let out_dir = env::var("CARGO_MANIFEST_DIR")?;
    let repo_dir = Path::new(&out_dir).join("abbrv.jabref.org");
    if !repo_dir.exists() {
        Command::new("git")
            .args([
                "clone",
                "https://github.com/JabRef/abbrv.jabref.org.git",
                repo_dir.to_str().unwrap(),
            ])
            .status()
            .with_context(|| format!("Failed to clone the repository into {:?}", repo_dir))?;
    }
    Ok(repo_dir)
}

fn process_csv_files(repo_dir: &Path, import_order: Order) -> Result<Vec<Record>> {
    let journals_path = repo_dir.join("journals");
    import_order
        .file_suffixes()
        .iter()
        .map(|suffix| {
            let file_path = journals_path.join(format!("journal_abbreviations_{}.csv", suffix));
            read_csv(&file_path)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().flatten().collect())
}

fn read_csv(file_path: &Path) -> Result<Vec<Record>> {
    ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .trim(Trim::All)
        .from_path(file_path)
        .with_context(|| format!("Failed to open CSV file {:?}", file_path))?
        .deserialize()
        .collect::<Result<Vec<Record>, _>>()
        .context("Failed to read and deserialize CSV records")
}

fn write_journals_to_csv(journals: &[Record], output_csv_filename: &str) -> Result<()> {
    let mut wtr = WriterBuilder::new().from_writer(File::create(output_csv_filename)?);
    for journal in journals {
        wtr.serialize(journal)?;
    }
    wtr.flush()?;
    println!("CSV written to {}", output_csv_filename);
    Ok(())
}

fn write_journals_to_static_array(journals: &[Record]) -> Result<()> {
    let mut file = File::create("src/static_journals.rs")?;
    writeln!(file, "static JOURNALS: &[JournalRecord] = &[")?;
    for journal in journals {
        writeln!(
            file,
            "    JournalRecord {{ name: {:?}, abbr_1: {:?}, abbr_2: {:?}, abbr_3: {:?} }},",
            journal.full_name,
            journal.abbreviation_1,
            journal.abbreviation_2,
            journal.abbreviation_3
        )?;
    }
    writeln!(file, "];")?;
    Ok(())
}
