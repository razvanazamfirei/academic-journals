mod journal;

use crate::journal::JournalRecord;
use lazy_static::lazy_static;
use std::collections::HashMap;

include!("static_journals.rs");

//noinspection RsUnresolvedReference
lazy_static! {
    pub static ref FULL_NAME_TO_RECORD: HashMap<&'static str, &'static JournalRecord> = {
        let mut m = HashMap::with_capacity(JOURNALS.len());
        for journal in JOURNALS.iter() {
            m.insert(journal.name, journal);
        }
        m
    };
    pub static ref ABBREVIATION_TO_FULL_NAME: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::with_capacity(JOURNALS.len());
        for journal in JOURNALS.iter() {
            if let Some(abbr) = journal.abbr_1 {
                m.insert(abbr, journal.name);
            }
            if let Some(abbr) = journal.abbr_2 {
                m.insert(abbr, journal.name);
            }
            if let Some(abbr) = journal.abbr_3 {
                m.insert(abbr, journal.name);
            }
        }
        m
    };
}

pub fn get_abbreviation(full_name: &str) -> Option<&'static str> {
    FULL_NAME_TO_RECORD.get(full_name).and_then(|record| {
        record
            .abbr_1
            .or(record.abbr_2)
            .or(record.abbr_3)
    })
}

pub fn get_full_name(abbreviation: &str) -> Option<&'static str> {
    ABBREVIATION_TO_FULL_NAME.get(abbreviation).copied()
}
