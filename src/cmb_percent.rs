//! `+cmb%` -- how much of an account's XP is combat.
//!
//! Reports the share of total XP earned in the seven combat skills, with the
//! levels and XP on each side of that split.

use crate::common::{Listings, collect_hiscores, resolve_rsn};
use crate::stats::{stats_parameters, strip_stats_parameters};
use anyhow::Result;
use common::{commas, source::Source};

/// The seven skills that feed the combat level.
const COMBAT: [&str; 7] = [
    "Attack",
    "Strength",
    "Defence",
    "Prayer",
    "Hitpoints",
    "Ranged",
    "Magic",
];

/// Levels and XP for one side of the split.
#[derive(Default)]
struct Tally {
    levels: u64,
    xp: u64,
}

/// Combat and non-combat totals for a set of hiscores.
///
/// `Overall` is skipped -- it is the sum of the others, and counting it would
/// double every figure. The XP accumulates in `u64` on purpose: a maxed
/// account carries more total XP than a `u32` holds, so the combat-only sums
/// elsewhere in this crate do not generalise to every skill at once.
fn split(hiscores: &Listings) -> (Tally, Tally) {
    let mut combat = Tally::default();
    let mut other = Tally::default();

    for listing in hiscores.iter() {
        let name = listing.name.to_string();
        if name == "Overall" {
            continue;
        }

        let side = if COMBAT.contains(&name.as_str()) {
            &mut combat
        } else {
            &mut other
        };
        side.levels += listing.level as u64;
        side.xp += listing.xp as u64;
    }

    (combat, other)
}

fn describe(tally: &Tally, s: &Source) -> String {
    vec![
        s.c2(&commas(tally.levels as f64, "d")),
        s.c1("lvls"),
        s.c2(&commas(tally.xp as f64, "d")),
        s.c1("xp"),
    ]
    .join(" ")
}

pub fn percent(s: Source) -> Result<Vec<String>> {
    let prefix = s.l("Combat%");
    let not_found: Vec<String> = vec![vec![prefix.as_str(), &s.c1("No stats found")].join(" ")];

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let rsn = resolve_rsn(&joined, &s);
    let hiscores = match collect_hiscores(&joined, &s) {
        Ok(hiscores) => hiscores,
        Err(_) => return Ok(not_found),
    };
    let _ = flags;

    let (combat, other) = split(&hiscores);
    let total = combat.xp + other.xp;
    if total == 0 {
        return Ok(not_found);
    }

    let share = combat.xp as f64 / total as f64 * 100.0;

    let output = vec![
        prefix,
        s.l(&rsn),
        s.c1("Combat:"),
        s.c2(&format!("{share:.1}%")),
        s.c1("of"),
        s.c2(&commas(total as f64, "d")),
        s.c1("xp"),
        s.c1("|"),
        s.c1("Combat:"),
        s.p(&describe(&combat, &s)),
        s.c1("Non-combat:"),
        s.p(&describe(&other, &s)),
    ]
    .join(" ");

    Ok(vec![output])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{HiscoreName, Listing};

    fn listing(name: HiscoreName, level: u32, xp: u32) -> Listing {
        Listing {
            name,
            rank: 0,
            level,
            xp,
        }
    }

    #[test]
    fn overall_is_not_counted_on_either_side() {
        // Overall is the sum of the rest; counting it would double everything.
        let hiscores = Listings::new(vec![
            listing(HiscoreName::Overall, 200, 3_000_000),
            listing(HiscoreName::Attack, 99, 2_000_000),
            listing(HiscoreName::Cooking, 99, 1_000_000),
        ]);

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 2_000_000);
        assert_eq!(other.xp, 1_000_000);
        assert_eq!(combat.levels + other.levels, 198);
    }

    #[test]
    fn the_seven_combat_skills_land_on_the_combat_side() {
        let hiscores = Listings::new(
            [
                HiscoreName::Attack,
                HiscoreName::Strength,
                HiscoreName::Defence,
                HiscoreName::Prayer,
                HiscoreName::Hitpoints,
                HiscoreName::Ranged,
                HiscoreName::Magic,
            ]
            .into_iter()
            .map(|name| listing(name, 1, 100))
            .collect(),
        );

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 700);
        assert_eq!(other.xp, 0);
    }

    #[test]
    fn non_combat_skills_land_on_the_other_side() {
        let hiscores = Listings::new(vec![
            listing(HiscoreName::Cooking, 99, 13_034_431),
            listing(HiscoreName::Runecrafting, 99, 13_034_431),
        ]);

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 0);
        assert_eq!(other.xp, 26_068_862);
        assert_eq!(other.levels, 198);
    }

    #[test]
    fn a_total_that_would_overflow_a_u32_still_adds_up() {
        // Twenty skills at 200m is 4b, which a u32 cannot hold once Overall
        // is included -- the reason this sums into u64.
        let hiscores = Listings::new(
            [
                HiscoreName::Attack,
                HiscoreName::Strength,
                HiscoreName::Defence,
                HiscoreName::Prayer,
                HiscoreName::Hitpoints,
                HiscoreName::Ranged,
                HiscoreName::Magic,
                HiscoreName::Cooking,
                HiscoreName::Woodcutting,
                HiscoreName::Fletching,
                HiscoreName::Fishing,
                HiscoreName::Firemaking,
                HiscoreName::Crafting,
                HiscoreName::Smithing,
                HiscoreName::Mining,
                HiscoreName::Herblore,
                HiscoreName::Agility,
                HiscoreName::Thieving,
                HiscoreName::Farming,
                HiscoreName::Runecrafting,
            ]
            .into_iter()
            .map(|name| listing(name, 99, 200_000_000))
            .collect(),
        );

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp + other.xp, 4_000_000_000);
        assert_eq!(combat.xp, 1_400_000_000);
    }
}
