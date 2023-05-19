/// Converts a certain duration to a human-readable String.
///
/// Displays, at most, the amount of days. Otherwise, uses smaller units (the largest possible).
pub fn convert_duration_to_string(duration: std::time::Duration) -> String {
    let dur = chrono::Duration::from_std(duration);
    if let Ok(dur) = dur {
        convert_chrono_duration_to_string(dur)
    } else {
        let seconds = duration.as_secs();
        format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" })
    }
}

/// Converts a certain duration to a human-readable String in Portuguese.
///
/// Displays, at most, the amount of days. Otherwise, uses smaller units (the largest possible).
pub fn convert_duration_to_brazilian_string(duration: std::time::Duration) -> String {
    let dur = chrono::Duration::from_std(duration);
    if let Ok(dur) = dur {
        convert_chrono_duration_to_brazilian_string(dur)
    } else {
        let seconds = duration.as_secs();
        format!("{} segundo{}", seconds, if seconds != 1 { "s" } else { "" })
    }
}

/// Converts a Chrono duration to a string.
pub fn convert_chrono_duration_to_string(duration: chrono::Duration) -> String {
    if duration.num_minutes() < 1 {
        let seconds = duration.num_seconds();
        format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" })
    } else if duration.num_hours() < 1 {
        let minutes = duration.num_minutes();
        format!("{} minute{}", minutes, if minutes != 1 { "s" } else { "" })
    } else if duration.num_days() < 1 {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        format!(
            "{} hour{}{}",
            hours,
            if hours != 1 { "s" } else { "" },
            if minutes > 0 {
                format!(
                    " and {} minute{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                )
            } else {
                String::new()
            }
        )
    } else {
        let days = duration.num_days();
        format!("{} day{}", days, if days != 1 { "s" } else { "" })
    }
}

/// Converts a Chrono duration to a string in Brazilian Portuguese.
pub fn convert_chrono_duration_to_brazilian_string(duration: chrono::Duration) -> String {
    if duration.num_minutes() < 1 {
        let seconds = duration.num_seconds();
        format!("{} segundo{}", seconds, if seconds != 1 { "s" } else { "" })
    } else if duration.num_hours() < 1 {
        let minutes = duration.num_minutes();
        format!("{} minuto{}", minutes, if minutes != 1 { "s" } else { "" })
    } else if duration.num_days() < 1 {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        format!(
            "{} hora{}{}",
            hours,
            if hours != 1 { "s" } else { "" },
            if minutes > 0 {
                format!(
                    " e {} minuto{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                )
            } else {
                String::new()
            }
        )
    } else {
        let days = duration.num_days();
        format!("{} dia{}", days, if days != 1 { "s" } else { "" })
    }
}

/// Returns a [`poise::Context`]'s locale, or "en" if it was not found.
pub fn get_defaulted_locale<D, E>(ctx: poise::Context<'_, D, E>) -> &str {
    ctx.locale()
        .map(|loc| if loc != "pt-BR" { "en" } else { loc })
        .unwrap_or("pt-BR")
}

/// Returns a [`poise::ApplicationContext`]'s locale, or "en" if it was not found.
pub fn get_defaulted_app_ctx_locale<D, E>(ctx: poise::ApplicationContext<'_, D, E>) -> &str {
    ctx.locale()
        .map(|loc| if loc != "pt-BR" { "en" } else { loc })
        .unwrap_or("pt-BR")
}

/// Filters unknown locales, restricting them to returning "en".
pub fn default_locale(locale: &str) -> &'static str {
    match locale {
        "pt-BR" => "pt-BR",
        _ => "en",
    }
}
