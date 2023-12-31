mod cli;
use crate::cli::Cli;
use academic_journals::{get_abbreviation, get_full_name};
use clap::Parser;

/// The entry point of the application.
///
/// This function parses command-line arguments using the `Cli` struct from the `cli` module and
/// interacts with the `academic_journals` module to resolve journal abbreviations or full names.
///
/// # Examples
/// ```bash
/// # To get the abbreviation of a journal:
/// academic-journals --abbreviation "Journal of Rust Studies"
///
/// # To get the full name from an abbreviation:
/// academic-journals "JRS"
/// ```
fn main() {
    let cli: Cli = Cli::parse();

    if cli.abbreviation {
        let result = match get_abbreviation(&cli.input) {
            Some(abbreviation) => abbreviation.to_string(),
            None => format!("No abbreviation found for {}", cli.input),
        };
        println!("{}", result);
    } else {
        let result = match get_full_name(&cli.input) {
            Some(full_name) => full_name.to_string(),
            None => format!("No full name found for {}", cli.input),
        };
        println!("{}", result);
    }
}
