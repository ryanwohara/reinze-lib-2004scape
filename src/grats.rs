extern crate common;

use crate::common::{eval_query, skill as get_skill};
use anyhow::Result;
use common::source::Source;
use regex::Regex;

pub fn get(s: &Source) -> Result<Vec<String>> {
    let mut split = s.query.split_whitespace();

    let first_token = split.next().unwrap_or_default();
    let second_token = split.next().unwrap_or_default();
    let third_token = split.next().unwrap_or_default();

    let mut nick = s.author.nick.to_string();
    let mut milestone = first_token.to_string();
    let mut skill_token = second_token.to_string();

    let err = Ok(vec!["Syntax: +congrats [nick] (level) (skill)".to_string()]);

    if !third_token.is_empty() {
        nick = first_token.to_string();
        milestone = second_token.to_string();
        skill_token = third_token.to_string();
    } else if second_token.is_empty() {
        return err;
    }

    let skill_name = match combat_skill(&skill_token) {
        Some(name) => name.to_string(),
        None => get_skill(&skill_token),
    };

    if skill_name.is_empty() {
        return err;
    }

    let skill = &skill_name;

    let re = Regex::new(r"^([\d.]+)[kmb]?$").unwrap();
    _ = match re.captures(&milestone) {
        Some(captures) => vec![captures],
        None => return err,
    };

    let processed_milestone = eval_query(&milestone.replace(",", ""))
        .map_err(|e| anyhow::anyhow!("Failed to evaluate milestone: {}", e))?
        as u32;
    let comma_milestone = common::commas(processed_milestone as f64, "d");

    if skill_name == "Combat" && !(4..=126).contains(&processed_milestone) {
        return Ok(vec!["Combat level must be between 4 and 126".to_string()]);
    }

    let output = if skill == "Overall" {
        format!(
            "{0}: Congratulations on {1} {2}! Pretty impressive!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 40 {
        format!(
            "{0}: grats on {1} {2}! Keep up the good work.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 55 {
        format!(
            "{0}: getting somewhere! Grats on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 70 {
        format!(
            "{0}: awesome! Congratulations on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 85 {
        format!(
            "{0}: you are a CHAMPION! Congratulations on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 95 {
        format!(
            "{0}: you are one of the elite! Congratulations on {1} {2}!!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 99 {
        format!(
            "{0}: I am not worthy! Congratulations on {1} {2}!!!!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone > 98 && processed_milestone <= 150 {
        format!(
            "{0}: \\o/ CONGRATULATIONS ON {1} {2}! You are a true Runescaper!",
            nick, comma_milestone, skill
        )
    // We'll just assume it's XP
    } else if processed_milestone <= 1000000 {
        format!(
            "{0}: Congratulations on {1} {2} xp! Pretty impressive!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 2500000 {
        format!(
            "{0}: Hey congratulations on {1} {2} xp! Moving on up!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 5000000 {
        format!(
            "{0}: Congrats on {1} {2} xp! Almost halfway to 92!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 7500000 {
        format!(
            "{0}: More than halfway there! Keep on trucking! Congratulations for {1} {2} xp!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 10000000 {
        format!(
            "{0}: ALMOST TO 99! Congratulations for reaching {1} {2} xp! *jealous*",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 15000000 {
        format!(
            "{0}: IMPRESSIVE WORK! You must really love {2}. Congrats on {1} {2} xp.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 20000000 {
        format!(
            "{0}: WOW congratulations on {1} {2} xp! Go get yourself a snack. You earned it.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 50000000 {
        format!(
            "{0}: I\"m jealous of your {1} {2} xp! Congrats though!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 100000000 {
        format!(
            "{0}: You might be insane! Incredible congratulations on {1} xp! Everyone else is super jelly of your {2} skillz.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 200000000 {
        format!(
            "{0}: I have no more words for you. I am Hulk green with envy. Go train {1} more you beast. (Okay, congrats on {2} xp!)",
            nick, skill, comma_milestone
        )
    } else if processed_milestone == 200000000 {
        format!(
            "{0}: Okay, you win. You are on the highscores forever. Endless congratulations on maxing {1}. Go get some sunshine and a nice snack to celebrate!",
            nick, skill
        )
    } else {
        format!("{0}: That is not even a thing, get out of here.", nick)
    };

    Ok(vec![output])
}

/// Recognises the combat-level triggers for +gz. Case-insensitive.
fn combat_skill(token: &str) -> Option<&'static str> {
    match token.to_lowercase().as_str() {
        "combat" | "cmb" | "cmbt" => Some("Combat"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::author::Author;
    use common::ColorResult;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn source_with(query: &str) -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "gz",
            query,
        )
    }

    #[test]
    fn combat_skill_recognises_triggers() {
        assert_eq!(combat_skill("combat"), Some("Combat"));
        assert_eq!(combat_skill("cmb"), Some("Combat"));
        assert_eq!(combat_skill("cmbt"), Some("Combat"));
        assert_eq!(combat_skill("CMB"), Some("Combat"));
        assert_eq!(combat_skill("Combat"), Some("Combat"));
    }

    #[test]
    fn combat_skill_rejects_non_combat() {
        assert_eq!(combat_skill("attack"), None);
        assert_eq!(combat_skill("overall"), None);
        assert_eq!(combat_skill(""), None);
    }

    #[test]
    fn gz_combat_low_level() {
        let out = get(&source_with("4 combat")).unwrap();
        assert!(out[0].contains("4 Combat"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_max_level() {
        let out = get(&source_with("126 cmb")).unwrap();
        assert!(out[0].contains("126 Combat"), "got: {}", out[0]);
        assert!(out[0].contains("true Runescaper"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_mid_tier_uses_ladder() {
        // 70 lands in the "<85" CHAMPION bracket — proves a combat level flows
        // through the reused ladder labelled "Combat", not just the end tiers.
        let out = get(&source_with("70 combat")).unwrap();
        assert!(out[0].contains("70 Combat"), "got: {}", out[0]);
        assert!(out[0].contains("CHAMPION"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_below_range_errors() {
        let out = get(&source_with("3 cmb")).unwrap();
        assert_eq!(out[0], "Combat level must be between 4 and 126");
    }

    #[test]
    fn gz_combat_above_range_errors() {
        let out = get(&source_with("127 cmbt")).unwrap();
        assert_eq!(out[0], "Combat level must be between 4 and 126");
    }

    #[test]
    fn gz_non_combat_skill_unaffected() {
        let out = get(&source_with("99 attack")).unwrap();
        assert!(out[0].contains("99 Attack"), "got: {}", out[0]);
    }
}
