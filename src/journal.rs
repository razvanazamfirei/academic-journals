#[derive(Debug, Clone)]
pub struct JournalRecord {
    pub name: &'static str,
    pub abbr_1: Option<&'static str>,
    pub abbr_2: Option<&'static str>,
    pub abbr_3: Option<&'static str>,
}
