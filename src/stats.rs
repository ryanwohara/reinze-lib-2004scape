mod agility;
mod cooking;
mod crafting;
mod firemaking;
mod fishing;
mod fletching;
mod herblore;
mod magic;
mod mining;
mod prayer;
mod runecraft;
pub mod skill;
mod smithing;
mod thieving;
mod woodcutting;

use super::common::{
    HiscoreName, Listing, Listings, Stats, collect_hiscores, eval_query, level_to_xp, skill,
    skills, xp_to_level,
};
use crate::stats::skill::details_by_skill_id;
use anyhow::Result;
use common::{commas, source::Source};
use regex::Regex;

pub struct StatsFlags {
    pub filter_by: FilterBy,
    pub filter_at: u32,
    pub prefix: Prefix,
    pub flag: MutuallyExclusiveFlag,
    pub start: u32,
    pub end: u32,
    pub search: String,
}

impl StatsFlags {
    pub fn filter(&self, input: &u32) -> bool {
        (input > &0)
            && ((self.filter_by == FilterBy::None)
                || (self.filter_by == FilterBy::GreaterThan && input > &self.filter_at)
                || (self.filter_by == FilterBy::FewerThan && input < &self.filter_at)
                || (self.filter_by == FilterBy::GreaterThanOrEqualTo && input >= &self.filter_at)
                || (self.filter_by == FilterBy::FewerThanOrEqualTo && input <= &self.filter_at)
                || (self.filter_by == FilterBy::EqualTo && input == &self.filter_at))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterBy {
    EqualTo,
    FewerThan,
    FewerThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    None,
}

impl From<&str> for FilterBy {
    fn from(value: &str) -> Self {
        match value.to_string().as_str() {
            "<" => FilterBy::FewerThan,
            "<=" => FilterBy::FewerThanOrEqualTo,
            ">" => FilterBy::GreaterThan,
            ">=" => FilterBy::GreaterThanOrEqualTo,
            "=" => FilterBy::EqualTo,
            _ => FilterBy::None,
        }
    }
}

#[allow(dead_code)]
pub enum Prefix {
    Combat,
    Level,
    LowToHigh,
    None,
    Rank,
    Xp,
    XpToLevel,
}

impl Prefix {
    pub fn to_string(&self, s: &Source) -> String {
        let prefix = match self {
            Self::Combat => "Combat",
            Self::Level => "Level",
            Self::LowToHigh => "Low->High",
            Self::None => "",
            Self::Rank => "Rank",
            Self::Xp => "XP",
            Self::XpToLevel => "XPtoLevel",
        };

        if prefix.len() > 0 {
            s.p(prefix)
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MutuallyExclusiveFlag {
    Exp,
    None,
    Order,
    Rank,
    Sort,
}

impl From<&str> for MutuallyExclusiveFlag {
    fn from(s: &str) -> Self {
        match s {
            "-o" => Self::Order,
            "-s" => Self::Sort,
            "-r" => Self::Rank,
            "-x" => Self::Exp,
            _ => Self::None,
        }
    }
}

pub fn get_stats_regex() -> Regex {
    Regex::new(r"(?:^|\b|\s)(?:(-([serox]))|([<>=]=?)\s?([\d,.]+[kmb]?)|([#^])([\d,.]+[kmb]?)|(@)(\S+))(?:\b|$)").unwrap()
}

pub fn stats_parameters(query: &str) -> StatsFlags {
    let mut stats = StatsFlags {
        filter_by: FilterBy::None,
        filter_at: 0,
        prefix: Prefix::None,
        flag: MutuallyExclusiveFlag::None,
        start: 0,
        end: 0,
        search: "".to_string(),
    };

    for (_, [flag_identifier, detail]) in get_stats_regex()
        .captures_iter(query)
        .map(|capture| capture.extract())
    {
        match flag_identifier {
            "-s" => stats.flag = MutuallyExclusiveFlag::Sort,
            "-o" => stats.flag = MutuallyExclusiveFlag::Order,
            "-r" => stats.flag = MutuallyExclusiveFlag::Rank,
            "-e" | "-x" => stats.flag = MutuallyExclusiveFlag::Exp,
            "^" => stats.start = eval_query(detail).unwrap_or(0.0) as u32,
            "#" => stats.end = eval_query(detail).unwrap_or(0.0) as u32,
            "@" => stats.search = detail.to_string(),
            ">" | "<" | ">=" | "<=" | "=" | "==" => {
                stats.filter_by = FilterBy::from(flag_identifier);
                stats.filter_at = eval_query(detail).unwrap_or(0.0) as u32;
            }
            _ => {}
        };
    }

    stats
}

pub fn strip_stats_parameters(query: &str) -> String {
    get_stats_regex().replace_all(query, "").to_string()
}

fn invalid<T>(prefix: T, s: &Source) -> String
where
    T: ToString,
{
    vec![
        prefix.to_string(),
        s.c1("Level"),
        s.p("N/A"),
        s.c2("|"),
        s.c1("XP"),
        s.p("N/A"),
        s.c2("|"),
        s.c1("Rank"),
        s.p("N/A"),
    ]
    .join(" ")
}

fn prepare(command: &str) -> (usize, String) {
    let skill_name = skill(command);
    let skill_names = skills();
    let skill_id = skill_names
        .iter()
        .position(|r| r.eq(&skill_name))
        .unwrap_or(0);

    (skill_id, skill_name)
}

fn prefix(skill_name: &str, flags: &StatsFlags, s: &Source) -> String {
    vec![s.l(&skill_name), flags.prefix.to_string(s)]
        .join(" ")
        .trim()
        .replace("  ", " ")
}

pub fn lookup(s: Source) -> Result<Vec<String>> {
    let (skill_id, skill_name) = prepare(&s.command);

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let prefix = prefix(&skill_name, &flags, &s);

    let not_found = vec![invalid(&prefix, &s)];

    let start_xp = if flags.start > 126 {
        flags.start
    } else {
        level_to_xp(flags.start)
    };

    let start_level = xp_to_level(start_xp);

    let mut hiscores: Listings = HiscoreName::all()
        .iter()
        .map(|name| Listing {
            name: name.to_owned(),
            level: start_level,
            xp: start_xp,
            rank: 0,
        })
        .collect();

    if flags.start == 0 {
        hiscores = match collect_hiscores(&joined, &s) {
            Ok(hiscores) => hiscores,
            Err(_) => return Ok(not_found),
        };
    }

    let mut stats: Stats = Stats {
        flags,
        hiscores,
        source: s,
    };

    let s = &stats.source;

    if skill_id > 0 {
        // Individual skill lookup

        let listing = stats.hiscores.skill(&skill_name);

        if listing.is_none() {
            return Ok(not_found);
        }
        let listing = listing.unwrap();

        let next_level = listing.next_level(&stats.flags);
        let next_level_xp = level_to_xp(next_level);
        let xp_difference = next_level_xp - listing.xp;
        let actual_level = listing.actual_level();

        let actual_level_string = if actual_level > listing.level {
            s.p(&actual_level.to_string())
        } else {
            "".to_string()
        };

        let goal = vec![
            s.c1(&format!("XP to {}", next_level)),
            s.c2(&commas(xp_difference as f64, "d")),
            s.p(&format!("{}%", {
                let current_level_xp = level_to_xp(actual_level);
                let total_level_gap = next_level_xp - current_level_xp;
                let percentage = (1.0 - (xp_difference as f64 / total_level_gap as f64)) * 100.0;

                percentage.round()
            })),
        ]
        .join(" ");

        let level_string = vec![
            prefix,
            s.c1("Level"),
            s.c2(&commas(listing.actual_level() as f64, "d")),
            actual_level_string,
        ]
        .join(" ");

        let xp_string = vec![s.c1("XP"), s.c2(&commas(listing.xp as f64, "d"))].join(" ");

        let rank_string = vec![s.c1("Rank"), s.c2(&commas(listing.rank as f64, "d"))].join(" ");

        let mut result = vec![
            level_string.trim(),
            xp_string.trim(),
            goal.trim(),
            rank_string.trim(),
        ];
        result.retain(|x| x.len() > 0);

        let output = result.join(&s.c1(" | "));

        let details = details_by_skill_id(skill_id as u32, &stats.flags.search);

        let calc = details
            .iter()
            .map(|detail| detail.to_string(s, xp_difference as f64))
            .collect::<Vec<String>>()
            .join(&s.c1(" | "));

        Ok(vec![output, calc])
    } else {
        // Overall lookup

        let combat = stats.combat();
        let overall = stats.summary("Overall");

        stats.hiscores.filter(&stats.flags);

        let results = &mut stats
            .hiscores
            .iter()
            .map(|listing| match stats.flags.flag {
                MutuallyExclusiveFlag::Sort => {
                    let next_level = listing.next_level(&stats.flags);
                    let next_level_xp = level_to_xp(next_level);
                    let xp_difference = next_level_xp - listing.xp;

                    (listing.name.to_string(), xp_difference)
                }
                MutuallyExclusiveFlag::Order | MutuallyExclusiveFlag::Exp => {
                    (listing.name.to_string(), listing.xp)
                }
                MutuallyExclusiveFlag::Rank => (listing.name.to_string(), listing.rank),
                MutuallyExclusiveFlag::None => (listing.name.to_string(), listing.actual_level()),
            })
            .collect::<Vec<(String, u32)>>();

        let summary = vec![combat.to_string(s), overall].join(" ");

        match stats.flags.flag {
            MutuallyExclusiveFlag::Order | MutuallyExclusiveFlag::Sort => {
                results.sort_by(|(_name1, number1), (_name2, number2)| number1.cmp(number2))
            }
            _ => (),
        }

        let tmp = if stats.flags.flag.ne(&MutuallyExclusiveFlag::Order) {
            results
        } else {
            &mut results
                .iter()
                .map(|(name, number)| (name.to_string(), xp_to_level(*number)))
                .collect::<Vec<(String, u32)>>()
        };

        let message = tmp
            .iter()
            .map(|(name, number)| {
                vec![
                    s.c1(&vec![name, ":"].join("")),
                    s.c2(&commas(*number as f64, "d")),
                ]
                .join("")
            })
            .collect::<Vec<String>>()
            .join(" ");

        let output = vec![prefix, summary, message].join(" ");

        Ok(vec![output])
    }
}

#[allow(dead_code)]
fn tier(points: u32) -> String {
    match points {
        0..=2499 => "Unranked",
        2500..=4999 => "Bronze",
        5000..=9999 => "Iron",
        10000..=17999 => "Steel",
        18000..=27999 => "Mithril",
        28000..=41999 => "Adamant",
        42000..=55999 => "Rune",
        _ => "Dragon",
    }
    .to_string()
}

pub fn combat(s: Source) -> Result<Vec<String>> {
    let prefix = s.l("Combat");

    let not_found: Vec<String> =
        vec![vec![prefix.as_str(), &s.c1("No combat stats found")].join(" ")];

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let hiscores = match collect_hiscores(&joined, &s) {
        Ok(hiscores) => hiscores,
        Err(_) => return Ok(not_found),
    };

    let mut stats: Stats = Stats {
        flags,
        hiscores,
        source: s,
    };

    let s = &stats.source;

    let combat = stats.combat();
    stats.hiscores.retain_combat();
    let total_level: u32 = stats.hiscores.iter().map(|listing| listing.level).sum();
    if total_level == 0 {
        return Ok(not_found);
    }
    let total_lvl_str = vec![s.c1("Levels:"), s.c2(&commas(total_level as f64, "d"))].join(" ");

    let total_xp: u32 = stats.hiscores.iter().map(|listing| listing.xp).sum();
    let total_xp_str = vec![s.c1("XP:"), s.c2(&commas(total_xp as f64, "d"))].join(" ");
    let total_str = vec![total_lvl_str, total_xp_str].join(&s.c1(" | "));

    let summary = stats
        .hiscores
        .iter()
        .map(|listing| {
            vec![
                s.c1(&vec![&listing.name.to_string(), ":"].join("")),
                s.c2(&listing.level.to_string()),
            ]
            .join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let mut calculations = combat.calc(&stats);
    calculations.retain(|(_string, int)| int > &0u32);
    let calc = calculations
        .iter()
        .map(|(string, int)| {
            vec![s.c1(&vec![string, ":"].join("")), s.c2(&int.to_string())].join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let output = vec![
        prefix,
        combat.to_string(s),
        s.c1("Total Combat"),
        s.l(&total_str),
        s.c1("To Next Level:"),
        s.p(&calc),
        s.c1("Current Levels:"),
        s.p(&summary),
    ]
    .join(" ");

    Ok(vec![output])
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- FilterBy tests ---

    #[test]
    fn test_filter_by_from() {
        assert_eq!(FilterBy::from("<"), FilterBy::FewerThan);
        assert_eq!(FilterBy::from("<="), FilterBy::FewerThanOrEqualTo);
        assert_eq!(FilterBy::from(">"), FilterBy::GreaterThan);
        assert_eq!(FilterBy::from(">="), FilterBy::GreaterThanOrEqualTo);
        assert_eq!(FilterBy::from("="), FilterBy::EqualTo);
        assert_eq!(FilterBy::from("invalid"), FilterBy::None);
        assert_eq!(FilterBy::from(""), FilterBy::None);
    }

    // --- MutuallyExclusiveFlag tests ---

    #[test]
    fn test_mutually_exclusive_flag_from() {
        assert_eq!(
            MutuallyExclusiveFlag::from("-o"),
            MutuallyExclusiveFlag::Order
        );
        assert_eq!(
            MutuallyExclusiveFlag::from("-s"),
            MutuallyExclusiveFlag::Sort
        );
        assert_eq!(
            MutuallyExclusiveFlag::from("-r"),
            MutuallyExclusiveFlag::Rank
        );
        assert_eq!(
            MutuallyExclusiveFlag::from("-x"),
            MutuallyExclusiveFlag::Exp
        );
        assert_eq!(MutuallyExclusiveFlag::from(""), MutuallyExclusiveFlag::None);
        assert_eq!(
            MutuallyExclusiveFlag::from("garbage"),
            MutuallyExclusiveFlag::None
        );
    }

    // --- StatsFlags::filter tests ---

    #[test]
    fn test_stats_flags_filter_none() {
        let flags = StatsFlags {
            filter_by: FilterBy::None,
            filter_at: 0,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&50));
        assert!(!flags.filter(&0)); // zero always filtered out
    }

    #[test]
    fn test_stats_flags_filter_greater_than() {
        let flags = StatsFlags {
            filter_by: FilterBy::GreaterThan,
            filter_at: 50,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&51));
        assert!(!flags.filter(&50));
        assert!(!flags.filter(&49));
    }

    #[test]
    fn test_stats_flags_filter_fewer_than() {
        let flags = StatsFlags {
            filter_by: FilterBy::FewerThan,
            filter_at: 50,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&49));
        assert!(!flags.filter(&50));
        assert!(!flags.filter(&51));
    }

    #[test]
    fn test_stats_flags_filter_equal_to() {
        let flags = StatsFlags {
            filter_by: FilterBy::EqualTo,
            filter_at: 99,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&99));
        assert!(!flags.filter(&98));
    }

    #[test]
    fn test_stats_flags_filter_gte() {
        let flags = StatsFlags {
            filter_by: FilterBy::GreaterThanOrEqualTo,
            filter_at: 50,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&50));
        assert!(flags.filter(&51));
        assert!(!flags.filter(&49));
    }

    #[test]
    fn test_stats_flags_filter_lte() {
        let flags = StatsFlags {
            filter_by: FilterBy::FewerThanOrEqualTo,
            filter_at: 50,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert!(flags.filter(&50));
        assert!(flags.filter(&49));
        assert!(!flags.filter(&51));
    }

    // --- stats_parameters tests ---

    #[test]
    fn test_stats_parameters_empty() {
        let flags = stats_parameters("");
        assert_eq!(flags.filter_by, FilterBy::None);
        assert_eq!(flags.flag, MutuallyExclusiveFlag::None);
        assert_eq!(flags.start, 0);
        assert_eq!(flags.end, 0);
        assert_eq!(flags.search, "");
    }

    #[test]
    fn test_stats_parameters_sort_flag() {
        let flags = stats_parameters("-s");
        assert_eq!(flags.flag, MutuallyExclusiveFlag::Sort);
    }

    #[test]
    fn test_stats_parameters_order_flag() {
        let flags = stats_parameters("-o");
        assert_eq!(flags.flag, MutuallyExclusiveFlag::Order);
    }

    #[test]
    fn test_stats_parameters_rank_flag() {
        let flags = stats_parameters("-r");
        assert_eq!(flags.flag, MutuallyExclusiveFlag::Rank);
    }

    #[test]
    fn test_stats_parameters_exp_flag() {
        let flags = stats_parameters("-x");
        assert_eq!(flags.flag, MutuallyExclusiveFlag::Exp);

        let flags2 = stats_parameters("-e");
        assert_eq!(flags2.flag, MutuallyExclusiveFlag::Exp);
    }

    #[test]
    fn test_stats_parameters_filter() {
        let flags = stats_parameters("> 50");
        assert_eq!(flags.filter_by, FilterBy::GreaterThan);
        assert_eq!(flags.filter_at, 50);
    }

    #[test]
    fn test_stats_parameters_start_end() {
        let flags = stats_parameters("^50 #99");
        assert_eq!(flags.start, 50);
        assert_eq!(flags.end, 99);
    }

    #[test]
    fn test_stats_parameters_search() {
        let flags = stats_parameters("@dragon");
        assert_eq!(flags.search, "dragon");
    }

    // --- strip_stats_parameters tests ---

    #[test]
    fn test_strip_stats_parameters_plain() {
        assert_eq!(strip_stats_parameters("player name"), "player name");
    }

    #[test]
    fn test_strip_stats_parameters_removes_flags() {
        let result = strip_stats_parameters("player -s");
        assert_eq!(result.trim(), "player");
    }

    #[test]
    fn test_strip_stats_parameters_removes_filter() {
        let result = strip_stats_parameters("player > 50");
        assert_eq!(result.trim(), "player");
    }

    // --- get_stats_regex tests ---

    #[test]
    fn test_get_stats_regex_compiles() {
        let re = get_stats_regex();
        assert!(re.is_match("-s"));
        assert!(re.is_match("> 50"));
        assert!(re.is_match("^99"));
        assert!(re.is_match("#50"));
        assert!(re.is_match("@search"));
    }

    #[test]
    fn test_get_stats_regex_no_account_flags() {
        let re = get_stats_regex();
        // Account type flags were removed — these should NOT match
        assert!(!re.is_match("-i"));
        assert!(!re.is_match("-u"));
        assert!(!re.is_match("-h"));
        assert!(!re.is_match("-d"));
        assert!(!re.is_match("-l"));
        assert!(!re.is_match("-t"));
    }
}
