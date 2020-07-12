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
}