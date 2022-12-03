use crate::rule::rules::*;

pub fn rule_normalize(data_to_change: &str, rule: &SingleRule) -> String {
    let return_string;

    match rule.rule_type {
        RuleType::Normalize => match rule.rule_place {
            RulePlace::ExtensionAndName => {
                return_string = slugmin::slugify_normal(data_to_change, !rule.rule_data.full_normalize);
            }
            _ => panic!("Invalid Rule Place for Normalization"),
        },
        _ => panic!("Invalid Rule Type for Normalization"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule::rule_normalize::rule_normalize;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_normalize() {
        let mut rule = SingleRule::new();

        rule.rule_type = RuleType::Normalize;
        rule.rule_place = RulePlace::ExtensionAndName;

        rule.rule_data.full_normalize = true;
        assert_eq!(rule_normalize("Świstak.txt", &rule), "swistak.txt");
        assert_eq!(rule_normalize("SŚFSÆŚFLASÆOW          .t", &rule), "ssfsaesflasaeow .t");
        assert_eq!(rule_normalize("ŚwSFS:F:F::F", &rule), "swsfs-f-f-f");
        assert_eq!(rule_normalize("       ---- sf s sf- --", &rule), "sf s sf");
        assert_eq!(rule_normalize("SFMWOMWOMWF  SFaflwp", &rule), "sfmwomwomwf sfaflwp");
        assert_eq!(rule_normalize("śfsśæśfsædŋę’’’’.txt", &rule), "sfssaesfsaednge-.txt");

        rule.rule_data.full_normalize = false;
        assert_eq!(rule_normalize("Świstak.txt", &rule), "Swistak.txt");
        assert_eq!(rule_normalize("SŚFSÆŚFLASÆOW          .t", &rule), "SSFSAESFLASAEOW .t");
        assert_eq!(rule_normalize("ŚwSFS:F:F::F", &rule), "SwSFS-F-F-F");
        assert_eq!(rule_normalize("       ---- sf s sf- --", &rule), "sf s sf");
        assert_eq!(rule_normalize("SFMWOMWOMWF  SFaflwp", &rule), "SFMWOMWOMWF SFaflwp");
        assert_eq!(rule_normalize("śfsśæśfsædŋę’’’’.txt", &rule), "sfssaesfsaedNGe-.txt");
    }
}
