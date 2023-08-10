use super::limit_string_len;
use crate::forms::{CustomId, SelectMenuOptionSpec, SelectMenuSpec, SelectValue};

/// Applies Discord API string length limits to the given option spec, returning a new one
/// with the correct lengths.
pub fn apply_limits_to_select_option_spec(option: SelectMenuOptionSpec) -> SelectMenuOptionSpec {
    SelectMenuOptionSpec {
        label: limit_string_len(&option.label, 100),
        value_key: SelectValue(limit_string_len(&option.value_key.0, 100)),
        description: option
            .description
            .as_ref()
            .map(|desc| limit_string_len(desc, 100)),
        ..option
    }
}

/// Applies Discord API string length limits to the menu's options and other strings.
pub fn apply_limits_to_select_menu_spec(menu: SelectMenuSpec) -> SelectMenuSpec {
    SelectMenuSpec {
        custom_id: CustomId(limit_string_len(&menu.custom_id.0, 100)),
        options: menu
            .options
            .into_iter()
            .map(apply_limits_to_select_option_spec)
            .collect(),
        ..menu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_spec_limits_are_properly_applied_when_strings_surpass_100_chars() {
        let spec = SelectMenuOptionSpec {
            label: "a".repeat(101),
            value_key: SelectValue("a".repeat(101)),
            description: Some("a".repeat(101)),
            emoji: None,
            is_default: false,
        };
        assert_eq!(
            apply_limits_to_select_option_spec(spec),
            SelectMenuOptionSpec {
                label: "a".repeat(100 - 3) + "...",
                value_key: SelectValue("a".repeat(100 - 3) + "..."),
                description: Some("a".repeat(100 - 3) + "..."),
                emoji: None,
                is_default: false,
            }
        );
    }

    #[test]
    fn test_select_menu_spec_limits_are_properly_applied_when_strings_surpass_100_chars() {
        let spec = SelectMenuSpec {
            custom_id: CustomId("a".repeat(101)),
            options: vec![SelectMenuOptionSpec {
                label: "a".repeat(101),
                value_key: SelectValue("abc".to_owned()),
                description: Some("abc".to_owned()),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(
            apply_limits_to_select_menu_spec(spec),
            SelectMenuSpec {
                custom_id: CustomId("a".repeat(100 - 3) + "..."),
                options: vec![SelectMenuOptionSpec {
                    label: "a".repeat(100 - 3) + "...",
                    value_key: SelectValue("abc".to_owned()),
                    description: Some("abc".to_owned()),
                    ..Default::default()
                }],
                ..Default::default()
            }
        );
    }
}
