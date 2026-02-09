use cruet::Inflector;
use std::fmt::{self, Display, Formatter};

/// Represents a complete sentence structure for an assertion
#[derive(Debug, Clone)]
pub struct AssertionSentence {
    /// The subject of the assertion (usually the variable name)
    pub subject: String,
    /// The verb of the assertion (e.g., "be", "have", "contain")
    pub verb: String,
    /// The object of the assertion (e.g., "greater than 42", "of length 5", "'test'")
    pub object: String,
    /// Optional qualifiers for the assertion (e.g., "within tolerance", "when rounded")
    pub qualifiers: Vec<String>,
    /// Whether the assertion is negated (e.g., "not be", "does not have")
    pub negated: bool,
}

impl AssertionSentence {
    /// Create a new assertion sentence
    pub fn new(verb: impl Into<String>, object: impl Into<String>) -> Self {
        return Self { subject: "".to_string(), verb: verb.into(), object: object.into(), qualifiers: Vec::new(), negated: false };
    }

    /// Set whether the assertion is negated
    pub fn with_negation(mut self, negated: bool) -> Self {
        self.negated = negated;
        return self;
    }

    /// Add a qualifier to the assertion
    pub fn with_qualifier(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifiers.push(qualifier.into());
        return self;
    }

    /// Format the sentence into a readable string (raw format, without subject)
    pub fn format(&self) -> String {
        let mut result = if self.negated { format!("not {} {}", self.verb, self.object) } else { format!("{} {}", self.verb, self.object) };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }

    /// Format the sentence with grammatically correct 'not' placement (after the verb)
    /// This is used for display purposes where improved grammar is desired
    pub fn format_grammatical(&self) -> String {
        let mut result = if self.negated {
            // Place "not" after the verb for grammatical correctness
            format!("{} not {}", self.verb, self.object)
        } else {
            format!("{} {}", self.verb, self.object)
        };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }

    /// Format the sentence with the correct verb conjugation based on the subject
    pub fn format_with_conjugation(&self, subject: &str) -> String {
        // Determine if the subject is plural
        let is_plural = Self::is_plural_subject(subject);

        // Convert the infinitive verb to the correct form based on plurality
        let conjugated_verb = self.conjugate_verb(is_plural);

        let mut result = if self.negated {
            // Place "not" after the conjugated verb for grammatical correctness
            format!("{} not {}", conjugated_verb, self.object)
        } else {
            format!("{} {}", conjugated_verb, self.object)
        };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }

    /// Determine if a subject name is likely plural using the cruet crate
    /// for proper English singularization. If singularizing a word changes it,
    /// the original was plural.
    fn is_plural_subject(subject: &str) -> bool {
        // Extract the base variable name from expressions like "var.method()" or "&var"
        let base_name = Self::extract_base_name(subject);

        // For snake_case variable names (common in Rust), check the last word segment
        let last_word = base_name.split('_').next_back().unwrap_or(&base_name);
        let last_word_lower = last_word.to_lowercase();

        // Use cruet's singularization: if singularizing changes the word, it was plural
        let singularized = last_word_lower.to_singular();

        return singularized != last_word_lower;
    }

    /// Extract the base variable name from expressions
    fn extract_base_name(expr: &str) -> String {
        // Remove reference symbols
        let without_ref = expr.trim_start_matches('&');

        // Handle method calls like "var.method()" - extract "var"
        if let Some(dot_pos) = without_ref.find('.') {
            return without_ref[0..dot_pos].to_string();
        }

        // Handle array/slice indexing like "var[0]" - extract "var"
        if let Some(bracket_pos) = without_ref.find('[') {
            return without_ref[0..bracket_pos].to_string();
        }

        // No special case, return as is
        return without_ref.to_string();
    }

    /// Conjugate the verb based on plurality
    ///
    /// Note: We use a manual match here rather than `cruet` because `cruet` only handles
    /// noun inflections (pluralize/singularize), not verb conjugation. Since the set of
    /// verbs used by matchers is small and controlled by this crate, a manual match is
    /// both correct and sufficient.
    fn conjugate_verb(&self, is_plural: bool) -> String {
        // Special case handling for common verbs
        match self.verb.as_str() {
            "be" => {
                if is_plural {
                    "are".to_string()
                } else {
                    "is".to_string()
                }
            }
            "have" => {
                if is_plural {
                    "have".to_string()
                } else {
                    "has".to_string()
                }
            }
            "contain" => {
                if is_plural {
                    "contain".to_string()
                } else {
                    "contains".to_string()
                }
            }
            "start with" => {
                if is_plural {
                    "start with".to_string()
                } else {
                    "starts with".to_string()
                }
            }
            "end with" => {
                if is_plural {
                    "end with".to_string()
                } else {
                    "ends with".to_string()
                }
            }
            // For other verbs, add 's' in singular form
            verb => {
                if is_plural {
                    verb.to_string()
                } else {
                    // Handle special cases for verbs ending in certain characters
                    if verb.ends_with('s') || verb.ends_with('x') || verb.ends_with('z') || verb.ends_with("sh") || verb.ends_with("ch") {
                        format!("{}es", verb)
                    } else if verb.ends_with('y')
                        && !verb.ends_with("ay")
                        && !verb.ends_with("ey")
                        && !verb.ends_with("oy")
                        && !verb.ends_with("uy")
                    {
                        format!("{}ies", &verb[0..verb.len() - 1])
                    } else {
                        format!("{}s", verb)
                    }
                }
            }
        }
    }
}

impl Display for AssertionSentence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.format());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertion_sentence_new() {
        let sentence = AssertionSentence::new("be", "positive");

        assert_eq!(sentence.subject, "");
        assert_eq!(sentence.verb, "be");
        assert_eq!(sentence.object, "positive");
        assert_eq!(sentence.qualifiers.len(), 0);
        assert_eq!(sentence.negated, false);
    }

    #[test]
    fn test_with_negation() {
        let sentence = AssertionSentence::new("be", "positive").with_negation(true);

        assert_eq!(sentence.negated, true);

        // Test chaining and toggle
        let toggled_sentence = sentence.with_negation(false);

        assert_eq!(toggled_sentence.negated, false);
    }

    #[test]
    fn test_with_qualifier() {
        let sentence = AssertionSentence::new("be", "in range").with_qualifier("when rounded");

        assert_eq!(sentence.qualifiers, vec!["when rounded"]);

        // Test multiple qualifiers
        let updated_sentence = sentence.with_qualifier("with tolerance");

        assert_eq!(updated_sentence.qualifiers, vec!["when rounded", "with tolerance"]);
    }

    #[test]
    fn test_format_basic() {
        let sentence = AssertionSentence::new("be", "positive");

        assert_eq!(sentence.format(), "be positive");
    }

    #[test]
    fn test_format_with_negation() {
        let sentence = AssertionSentence::new("be", "positive").with_negation(true);

        assert_eq!(sentence.format(), "not be positive");
    }

    #[test]
    fn test_format_with_qualifiers() {
        let sentence = AssertionSentence::new("be", "in range").with_qualifier("when rounded").with_qualifier("with tolerance");

        assert_eq!(sentence.format(), "be in range when rounded with tolerance");
    }

    #[test]
    fn test_format_with_negation_and_qualifiers() {
        let sentence = AssertionSentence::new("be", "in range").with_negation(true).with_qualifier("when rounded");

        assert_eq!(sentence.format(), "not be in range when rounded");
    }

    #[test]
    fn test_format_grammatical() {
        let sentence = AssertionSentence::new("be", "positive");

        assert_eq!(sentence.format_grammatical(), "be positive");

        let negated = sentence.clone().with_negation(true);

        // The "not" should be after the verb for grammatical correctness
        assert_eq!(negated.format_grammatical(), "be not positive");
    }

    #[test]
    fn test_format_grammatical_with_qualifiers() {
        let sentence = AssertionSentence::new("be", "in range").with_negation(true).with_qualifier("when rounded");

        assert_eq!(sentence.format_grammatical(), "be not in range when rounded");
    }

    #[test]
    fn test_is_plural_subject() {
        // Test singular subjects
        assert_eq!(AssertionSentence::is_plural_subject("value"), false);
        assert_eq!(AssertionSentence::is_plural_subject("number"), false);
        assert_eq!(AssertionSentence::is_plural_subject("count"), false);
        assert_eq!(AssertionSentence::is_plural_subject("item"), false);

        // Test singular subjects that end in 's' (the bug this fix addresses)
        assert_eq!(AssertionSentence::is_plural_subject("status"), false);
        assert_eq!(AssertionSentence::is_plural_subject("address"), false);
        assert_eq!(AssertionSentence::is_plural_subject("process"), false);
        assert_eq!(AssertionSentence::is_plural_subject("bus"), false);

        // Test plural subjects
        assert_eq!(AssertionSentence::is_plural_subject("values"), true);
        assert_eq!(AssertionSentence::is_plural_subject("numbers"), true);
        assert_eq!(AssertionSentence::is_plural_subject("items"), true);
        assert_eq!(AssertionSentence::is_plural_subject("lists"), true);
        assert_eq!(AssertionSentence::is_plural_subject("entries"), true);
        assert_eq!(AssertionSentence::is_plural_subject("data"), true);

        // Test snake_case compound variable names (checks last segment)
        assert_eq!(AssertionSentence::is_plural_subject("my_values"), true);
        assert_eq!(AssertionSentence::is_plural_subject("test_items"), true);
        assert_eq!(AssertionSentence::is_plural_subject("user_status"), false);
        assert_eq!(AssertionSentence::is_plural_subject("http_address"), false);
    }

    #[test]
    fn test_extract_base_name() {
        // Test reference extraction
        assert_eq!(AssertionSentence::extract_base_name("&value"), "value");

        // Test method call extraction
        assert_eq!(AssertionSentence::extract_base_name("values.len()"), "values");

        // Test array indexing extraction
        assert_eq!(AssertionSentence::extract_base_name("items[0]"), "items");

        // Test combined cases
        assert_eq!(AssertionSentence::extract_base_name("&items[0]"), "items");
        assert_eq!(AssertionSentence::extract_base_name("&values.len()"), "values");
    }

    #[test]
    fn test_conjugate_verb() {
        // Create a test sentence
        let sentence = AssertionSentence::new("", "");

        // Test special case verbs
        let special_verbs = [
            ("be", "is", "are"),
            ("have", "has", "have"),
            ("contain", "contains", "contain"),
            ("start with", "starts with", "start with"),
            ("end with", "ends with", "end with"),
        ];

        for (base, singular, plural) in special_verbs.iter() {
            let mut test_sentence = sentence.clone();
            test_sentence.verb = base.to_string();

            assert_eq!(test_sentence.conjugate_verb(false), *singular);
            assert_eq!(test_sentence.conjugate_verb(true), *plural);
        }

        // Test regular verbs
        let regular_verbs = [("match", "matches"), ("exceed", "exceeds"), ("include", "includes")];

        for (base, singular) in regular_verbs.iter() {
            let mut test_sentence = sentence.clone();
            test_sentence.verb = base.to_string();

            assert_eq!(test_sentence.conjugate_verb(false), *singular);
            assert_eq!(test_sentence.conjugate_verb(true), *base);
        }

        // Test verbs with special spelling rules
        let special_spelling = [
            // Verbs ending in s, x, z, sh, ch get 'es'
            ("pass", "passes"),
            ("fix", "fixes"),
            ("buzz", "buzzes"),
            ("wash", "washes"),
            ("match", "matches"),
            // Verbs ending in y (not preceded by a vowel) get 'ies'
            ("try", "tries"),
            ("fly", "flies"),
            ("comply", "complies"),
            // Verbs ending in y preceded by a vowel just get 's'
            ("play", "plays"),
            ("enjoy", "enjoys"),
        ];

        for (base, singular) in special_spelling.iter() {
            let mut test_sentence = sentence.clone();
            test_sentence.verb = base.to_string();

            assert_eq!(test_sentence.conjugate_verb(false), *singular);
            assert_eq!(test_sentence.conjugate_verb(true), *base);
        }
    }

    #[test]
    fn test_format_with_conjugation() {
        // Test singular subject conjugation
        let sentence = AssertionSentence::new("be", "positive");
        assert_eq!(sentence.format_with_conjugation("value"), "is positive");

        // Test plural subject conjugation
        assert_eq!(sentence.format_with_conjugation("values"), "are positive");

        // Test with negation - Note: we need to clone since with_negation consumes self
        let negated = sentence.clone().with_negation(true);
        assert_eq!(negated.format_with_conjugation("value"), "is not positive");
        assert_eq!(negated.format_with_conjugation("values"), "are not positive");

        // Test with qualifiers - Note: we need to clone since with_qualifier consumes self
        let qualified = sentence.clone().with_qualifier("always");
        assert_eq!(qualified.format_with_conjugation("value"), "is positive always");

        // Test different verbs
        let contain_sentence = AssertionSentence::new("contain", "element");
        assert_eq!(contain_sentence.format_with_conjugation("list"), "contains element");
        assert_eq!(contain_sentence.format_with_conjugation("lists"), "contain element");
    }

    #[test]
    fn test_display_trait() {
        let sentence = AssertionSentence::new("be", "positive");
        assert_eq!(format!("{}", sentence), "be positive");

        let negated = sentence.clone().with_negation(true);
        assert_eq!(format!("{}", negated), "not be positive");
    }
}
