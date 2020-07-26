use regex::{Regex, RegexSet, RegexSetBuilder};

pub struct RegexPatternManager {
    pub pattern:    RegexSet
}
impl RegexPatternManager {
    pub fn load_subtechnique() -> Self
    {
        RegexPatternManager {
            pattern: RegexSetBuilder::new(&[
                r#"T\d{4}\.\d{3}"#,
            ]).case_insensitive(true)
              .unicode(true)
              .build()
              .expect("(?) Error: RegexPatternManager | Cannot Build Subtechnique Pattern")
        }
    }
    pub fn load_technique() -> Self
    {
        RegexPatternManager {
            pattern:  RegexSetBuilder::new(&[
                r#"T\d{4}"#,
            ]).case_insensitive(true)
              .unicode(true)
              .build()
              .expect("(?) Error: RegexPatternManager | Cannot Build Technique ID Pattern")
        }
    }
    pub fn load_search_term_patterns() -> Self
    {
        RegexPatternManager {
            pattern:  RegexSetBuilder::new(&[
                r#"^T\d{4}$"#,                  // Technique ID
                r#"^T\d{4}\.\d{3}$"#,           // Subtechnique ID
                r#"(\W|^)[A-z]{4,}(\W|$)"#,     // Technique Name
            ]).case_insensitive(true)
              .unicode(true)
              .build()
              .expect("(?) Error: RegexPatternManager | Cannot Build Search Terms Patterns")
        }
    }
}