use crate::stats::StatsFlags;
use anyhow::{Context, Result, bail};
use common::{database, source::Source, *};
use log::error;
use meval::eval_str;
use mysql::{prelude::*, *};
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::slice::Iter;
use std::time::Duration;

// Catches shorthand skill names and returns the full name
pub fn skill(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "overall" | "stats" | "total" | "combat" | "cmb" => "Overall",
        "attack" | "att" => "Attack",
        "defence" | "def" => "Defence",
        "strength" | "str" => "Strength",
        "hitpoints" | "hp" => "Hitpoints",
        "ranged" | "range" => "Ranged",
        "prayer" | "pray" => "Prayer",
        "magic" | "mage" => "Magic",
        "cooking" | "cook" => "Cooking",
        "woodcutting" | "wc" => "Woodcutting",
        "fletching" | "fletch" => "Fletching",
        "fishing" | "fish" => "Fishing",
        "firemaking" | "fm" => "Firemaking",
        "crafting" | "craft" => "Crafting",
        "smithing" | "smith" => "Smithing",
        "mining" | "mine" => "Mining",
        "herblore" | "herb" => "Herblore",
        "agility" | "agil" => "Agility",
        "thieving" | "thief" => "Thieving",
        "farming" | "farm" => "Farming",
        "runecraft" | "rc" => "Runecrafting",
        _ => "",
    }
    .to_string()
}

// Returns a vector of all skills
pub fn skills() -> Vec<String> {
    vec![
        "Overall",
        "Attack",
        "Defence",
        "Strength",
        "Hitpoints",
        "Ranged",
        "Prayer",
        "Magic",
        "Cooking",
        "Woodcutting",
        "Fletching",
        "Fishing",
        "Firemaking",
        "Crafting",
        "Smithing",
        "Mining",
        "Herblore",
        "Agility",
        "Thieving",
        "Farming",
        "Runecrafting",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn skill_id<T>(skill: T) -> u32
where
    T: ToString,
{
    skills()
        .iter()
        .position(|s| s.to_string() == skill.to_string())
        .unwrap_or(0) as u32
}

pub fn skill_by_id(skill: u32) -> String {
    let mut s = skills();

    s.retain(|x| skill == skill_id(x));

    s.pop().unwrap_or("Overall".to_string())
}

// Converts a level to experience
pub fn level_to_xp(level: u32) -> u32 {
    let mut xp = 0.0;

    for i in 1..level {
        let x: f32 = i as f32;

        xp += (x + 300.0 * 2.0_f32.powf(x / 7.0)).floor() / 4.0;
    }

    xp.floor() as u32
        + match level {
            96..=99 => 1,
            105..110 => 2,
            110..115 => 5,
            115..120 => 3,
            120..126 => 7,
            126 => 4,
            _ => 0,
        }
}

// Converts experience to a level
pub fn xp_to_level(xp: u32) -> u32 {
    for level in 2..=127 {
        if xp < level_to_xp(level) {
            return level - 1;
        }
    }

    126
}

#[derive(Debug, Clone)]
pub struct Combat {
    pub level: f64,
    pub style: String,
}

impl Combat {
    pub fn calc(&self, stats: &Stats) -> Vec<(String, u32)> {
        let level_difference = self.level - self.level.floor();

        vec![
            "Attack",
            "Strength",
            "Defence",
            "Hitpoints",
            "Prayer",
            "Magic",
            "Ranged",
        ]
        .iter()
        .map(|skill| {
            (
                skill.to_string(),
                match stats.hiscores.skill(skill) {
                    Some(entry) => match entry.level {
                        99..=126 => 0.0,
                        _ => match skill.to_string().as_str() {
                            "Attack" | "Strength" => level_difference / 0.325,
                            "Defence" | "Hitpoints" => level_difference / 0.25,
                            "Prayer" => level_difference / 0.125,
                            _ => level_difference / 0.4875,
                        },
                    },
                    None => 0.0,
                }
                .ceil() as u32,
            )
        })
        .collect::<Vec<(String, u32)>>()
    }

    pub fn new(level: f64, style: &str) -> Combat {
        Combat {
            level,
            style: style.to_string(),
        }
    }

    pub fn to_string(&self, s: &Source) -> String {
        vec![
            s.c1("Combat:"),
            s.c2(&self.level.to_string()),
            s.p(&self.style),
        ]
        .join("")
    }
}

pub fn get_cmb(att: u32, str: u32, def: u32, hp: u32, range: u32, pray: u32, mage: u32) -> Combat {
    let base = ((def + hp) + (pray / 2)) as f64 * 0.25;

    let melee = 0.325 * (att + str) as f64;
    let ranged = 0.325 * ((range / 2) as f64 + range as f64);
    let magic = 0.325 * ((mage / 2) as f64 + mage as f64);

    let max_contribution = f64::max(melee, f64::max(ranged, magic));
    let level = f64::round((base + max_contribution) * 1000.0) / 1000.0;

    if melee > ranged && melee > magic {
        Combat::new(level, "Melee")
    } else if ranged > melee && ranged > magic {
        Combat::new(level, "Ranged")
    } else {
        // if magic > melee && magic > ranged
        Combat::new(level, "Magic")
    }
}

pub struct Stats {
    pub hiscores: Listings,
    pub flags: StatsFlags,
    #[allow(dead_code)]
    pub source: Source,
}

impl Stats {
    pub fn combat(&self) -> Combat {
        let attack = self.level("Attack");
        let strength = self.level("Strength");
        let defence = self.level("Defence");
        let hitpoints = self.level("Hitpoints");
        let magic = self.level("Magic");
        let ranged = self.level("Ranged");
        let prayer = self.level("Prayer");

        get_cmb(attack, strength, defence, hitpoints, ranged, prayer, magic)
    }

    pub fn summary(&self, skill: &str) -> String {
        let level = self.level(skill) as f64;
        let rank = self.rank(skill) as f64;
        let xp = self.xp(skill) as f64;

        format!(
            "{}{} {}{} {}{}",
            self.source.c1("Level:"),
            self.source.c2(&commas(level, "d")),
            self.source.c1("XP:"),
            self.source.c2(&commas(xp, "d")),
            self.source.c1("Rank:"),
            self.source.c2(&commas(rank, "d")),
        )
    }

    pub fn level(&self, skill: &str) -> u32 {
        self.skill_listing(skill).level
    }

    pub fn rank(&self, skill: &str) -> u32 {
        self.skill_listing(skill).rank
    }

    pub fn xp(&self, skill: &str) -> u32 {
        self.skill_listing(skill).xp
    }

    pub fn skill_listing(&self, skill: &str) -> Listing {
        self.hiscores
            .0
            .iter()
            .find(|listing| skill.eq(&listing.name.to_string()))
            .cloned()
            .unwrap_or(Listing {
                name: HiscoreName::None,
                rank: 0,
                level: 0,
                xp: 0,
            })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HiscoreName {
    Overall,
    Attack,
    Defence,
    Strength,
    Hitpoints,
    Ranged,
    Prayer,
    Magic,
    Cooking,
    Woodcutting,
    Fletching,
    Fishing,
    Firemaking,
    Crafting,
    Smithing,
    Mining,
    Herblore,
    Agility,
    Thieving,
    Farming,
    Runecrafting,
    None,
}

impl HiscoreName {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Overall,
            Self::Attack,
            Self::Defence,
            Self::Strength,
            Self::Hitpoints,
            Self::Ranged,
            Self::Prayer,
            Self::Magic,
            Self::Cooking,
            Self::Woodcutting,
            Self::Fletching,
            Self::Fishing,
            Self::Firemaking,
            Self::Crafting,
            Self::Smithing,
            Self::Mining,
            Self::Herblore,
            Self::Agility,
            Self::Thieving,
            Self::Farming,
            Self::Runecrafting,
            Self::None,
        ]
    }

    #[allow(dead_code)]
    pub fn index(&self) -> Option<usize> {
        Self::all().iter().position(|x| x == self)
    }

    #[allow(dead_code)]
    pub fn from_index(index: usize) -> Self {
        match Self::all().get(index) {
            Some(x) => x.to_owned(),
            None => Self::None,
        }
    }

    pub fn to(&self) -> Listing {
        Listing {
            name: self.to_owned(),
            rank: 0,
            level: 0,
            xp: 0,
        }
    }
}

impl From<&str> for HiscoreName {
    fn from(value: &str) -> Self {
        let mut all = Self::all();

        all.retain(|x| {
            x.to_string()
                .to_lowercase()
                .contains(&value.to_string().to_lowercase())
        });

        match all.first() {
            Some(&x) => x,
            None => Self::None,
        }
    }
}

impl Display for HiscoreName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Overall => "Overall",
            Self::Attack => "Attack",
            Self::Defence => "Defence",
            Self::Strength => "Strength",
            Self::Hitpoints => "Hitpoints",
            Self::Ranged => "Ranged",
            Self::Prayer => "Prayer",
            Self::Magic => "Magic",
            Self::Cooking => "Cooking",
            Self::Woodcutting => "Woodcutting",
            Self::Fletching => "Fletching",
            Self::Fishing => "Fishing",
            Self::Firemaking => "Firemaking",
            Self::Crafting => "Crafting",
            Self::Smithing => "Smithing",
            Self::Mining => "Mining",
            Self::Herblore => "Herblore",
            Self::Agility => "Agility",
            Self::Thieving => "Thieving",
            Self::Farming => "Farming",
            Self::Runecrafting => "Runecrafting",
            Self::None => "",
        };

        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Deserialize)]
struct HiscoreEntry {
    #[serde(rename = "type")]
    skill_type: u32,
    level: u32,
    value: u32,
    rank: u32,
}

fn type_to_hiscore_name(skill_type: u32) -> HiscoreName {
    match skill_type {
        0 => HiscoreName::Overall,
        1 => HiscoreName::Attack,
        2 => HiscoreName::Defence,
        3 => HiscoreName::Strength,
        4 => HiscoreName::Hitpoints,
        5 => HiscoreName::Ranged,
        6 => HiscoreName::Prayer,
        7 => HiscoreName::Magic,
        8 => HiscoreName::Cooking,
        9 => HiscoreName::Woodcutting,
        10 => HiscoreName::Fletching,
        11 => HiscoreName::Fishing,
        12 => HiscoreName::Firemaking,
        13 => HiscoreName::Crafting,
        14 => HiscoreName::Smithing,
        15 => HiscoreName::Mining,
        16 => HiscoreName::Herblore,
        17 => HiscoreName::Agility,
        18 => HiscoreName::Thieving,
        21 => HiscoreName::Runecrafting,
        _ => HiscoreName::None,
    }
}

pub fn collect_hiscores(input: &str, source: &Source) -> Result<Listings> {
    let nick = source.author.nick.to_string();

    let rsn = if input.is_empty() {
        get_rsn(source)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or(nick)
    } else {
        input.to_string()
    }
    .split_whitespace()
    .collect::<Vec<&str>>()
    .join("_");

    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::new(5, 0))
        .build()
        .context("failed to build HTTP client")?;

    let resp = match client
        .get(&format!(
            "https://2004.lostcity.rs/api/hiscores/player/{}",
            rsn
        ))
        .header(USER_AGENT, "Reinze.com")
        .send()
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", e);
            bail!("failed to make hiscores HTTP request");
        }
    };

    let status = resp.status();
    if status != 200 {
        bail!("hiscores returned status {}", status);
    }

    let entries: Vec<HiscoreEntry> = match resp.json() {
        Ok(entries) => entries,
        Err(e) => {
            error!("{}", e);
            bail!("failed to parse hiscores JSON response");
        }
    };

    let mut listings = vec![];

    for entry in entries {
        let name = type_to_hiscore_name(entry.skill_type);
        if name == HiscoreName::None {
            continue;
        }

        listings.push(Listing {
            name,
            rank: entry.rank,
            level: entry.level,
            xp: entry.value / 10,
        });
    }

    if listings.is_empty() {
        bail!("no hiscores data found for player");
    }

    Ok(Listings::new(listings))
}

#[derive(Clone, Debug)]
pub struct Listings(Vec<Listing>);

impl Listings {
    pub fn new(listings: Vec<Listing>) -> Self {
        Self(listings)
    }

    pub fn skill(&self, skill: &str) -> Option<Listing> {
        self.0
            .iter()
            .find(|listing| listing.name.to_string().eq(skill))
            .cloned()
    }

    pub fn iter(&'_ self) -> Iter<'_, Listing> {
        self.0.iter()
    }

    pub fn retain_combat(&mut self) {
        self.0.retain(|listing| {
            vec![
                "Attack",
                "Strength",
                "Defence",
                "Prayer",
                "Hitpoints",
                "Ranged",
                "Magic",
            ]
            .contains(&listing.name.to_string().as_str())
        });
    }

    pub fn filter(&mut self, flags: &StatsFlags) {
        self.0.retain(|listing| {
            listing.name.ne(&HiscoreName::Overall)
                && ((listing.level > 0 && flags.filter(&listing.level))
                    || (listing.level == 0 && flags.filter(&listing.xp)))
        })
    }
}

#[derive(Clone, Debug)]
pub struct Listing {
    pub name: HiscoreName,
    pub rank: u32,
    pub level: u32,
    pub xp: u32,
}

impl Listing {
    pub fn next_level(&self, flags: &StatsFlags) -> u32 {
        if flags.end > 0 {
            if flags.end <= 126 {
                flags.end
            } else {
                xp_to_level(flags.end)
            }
        } else {
            self.actual_level() + 1
        }
    }

    pub fn actual_level(&self) -> u32 {
        xp_to_level(self.xp)
    }
}

impl<'a> FromIterator<Listing> for Listings {
    fn from_iter<T: IntoIterator<Item = Listing>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let mut results = vec![];
        while let Some(index) = it.next() {
            results.push(index);
        }

        Self(results)
    }
}

impl<'a> FromIterator<&'a HiscoreName> for Listing {
    fn from_iter<T: IntoIterator<Item = &'a HiscoreName>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let index = it.next().unwrap_or(&HiscoreName::None);

        index.to()
    }
}

impl Display for Listing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}{} {}{}",
            c1("Lvl:"),
            c2(&commas(self.level as f64, "d")),
            c1("XP:"),
            c2(&commas(self.xp as f64, "d")),
            c1("Rank:"),
            c2(if self.rank == 0 {
                "N/A".to_string()
            } else {
                commas(self.rank as f64, "d")
            }
            .as_str())
        )
    }
}

pub fn get_rsn(source: &Source) -> core::result::Result<Vec<Row>, Error> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };

    let host = source.author.host.to_string();
    let rsn_n = source.rsn_n.to_string();

    match conn.exec_first(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => Ok(vec![rsn]),
        Ok(None) => Ok(vec![]),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    }
}

pub fn eval_query<T>(q: T) -> Result<f64>
where
    T: ToString,
{
    let query = q.to_string();

    let re_kmb = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
    let processed = re_kmb.replace_all(&query, replace_all).to_string();

    eval_str(&processed).map_err(|e| {
        error!("{}", e);
        anyhow::anyhow!("{}", e)
    })
}

pub fn replace_all(caps: &regex::Captures) -> String {
    let (num, kmb) = (
        caps.name("num").unwrap().as_str(),
        caps.name("kmb").unwrap().as_str(),
    );
    let mut num = num.parse::<f64>().unwrap_or_default();

    if let Some(factor) = match kmb {
        "k" => Some(1_000.0),
        "m" => Some(1_000_000.0),
        "b" => Some(1_000_000_000.0),
        _ => None,
    } {
        num *= factor;
    }
    num.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- get_cmb tests ---

    #[test]
    fn test_get_cmb_melee() {
        // Classic melee build: high att/str, low range/mage
        let cmb = get_cmb(99, 99, 99, 99, 1, 99, 1);
        assert_eq!(cmb.style, "Melee");
        assert!(cmb.level > 120.0);
    }

    #[test]
    fn test_get_cmb_ranged() {
        let cmb = get_cmb(1, 1, 99, 99, 99, 99, 1);
        assert_eq!(cmb.style, "Ranged");
    }

    #[test]
    fn test_get_cmb_magic() {
        let cmb = get_cmb(1, 1, 99, 99, 1, 99, 99);
        assert_eq!(cmb.style, "Magic");
    }

    #[test]
    fn test_get_cmb_all_ones() {
        let cmb = get_cmb(1, 1, 1, 10, 1, 1, 1);
        assert!(cmb.level > 0.0);
        assert!(cmb.level < 10.0);
    }

    #[test]
    fn test_get_cmb_maxed() {
        let cmb = get_cmb(99, 99, 99, 99, 99, 99, 99);
        assert!((cmb.level - 126.0).abs() < 1.0);
    }

    // --- eval_query tests ---

    #[test]
    fn test_eval_query_simple() {
        assert_eq!(eval_query("42").unwrap(), 42.0);
    }

    #[test]
    fn test_eval_query_expression() {
        assert_eq!(eval_query("2+3").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_query_kmb() {
        assert_eq!(eval_query("1k").unwrap(), 1000.0);
        assert_eq!(eval_query("2m").unwrap(), 2_000_000.0);
        assert_eq!(eval_query("1b").unwrap(), 1_000_000_000.0);
        assert_eq!(eval_query("1.5k").unwrap(), 1500.0);
    }

    #[test]
    fn test_eval_query_invalid() {
        assert!(eval_query("abc").is_err());
    }

    // --- replace_all tests ---

    #[test]
    fn test_replace_all_k() {
        let re = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
        let result = re.replace_all("10k", replace_all).to_string();
        assert_eq!(result, "10000");
    }

    #[test]
    fn test_replace_all_m() {
        let re = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
        let result = re.replace_all("2.5m", replace_all).to_string();
        assert_eq!(result, "2500000");
    }

    // --- HiscoreName tests ---

    #[test]
    fn test_hiscore_name_from_str() {
        assert_eq!(HiscoreName::from("Attack"), HiscoreName::Attack);
        assert_eq!(HiscoreName::from("Agility"), HiscoreName::Agility);
        assert_eq!(HiscoreName::from("Runecrafting"), HiscoreName::Runecrafting);
        assert_eq!(HiscoreName::from("nonexistent_xyz"), HiscoreName::None);
    }

    #[test]
    fn test_hiscore_name_from_str_case_insensitive() {
        assert_eq!(HiscoreName::from("attack"), HiscoreName::Attack);
        assert_eq!(HiscoreName::from("MINING"), HiscoreName::Mining);
    }

    #[test]
    fn test_hiscore_name_index() {
        assert_eq!(HiscoreName::Overall.index(), Some(0));
        assert_eq!(HiscoreName::Attack.index(), Some(1));
    }

    #[test]
    fn test_hiscore_name_from_index() {
        assert_eq!(HiscoreName::from_index(0), HiscoreName::Overall);
        assert_eq!(HiscoreName::from_index(1), HiscoreName::Attack);
        assert_eq!(HiscoreName::from_index(9999), HiscoreName::None);
    }

    #[test]
    fn test_hiscore_name_display() {
        assert_eq!(HiscoreName::Overall.to_string(), "Overall");
        assert_eq!(HiscoreName::Woodcutting.to_string(), "Woodcutting");
        assert_eq!(HiscoreName::None.to_string(), "");
    }

    #[test]
    fn test_hiscore_name_to_listing() {
        let listing = HiscoreName::Attack.to();
        assert_eq!(listing.name, HiscoreName::Attack);
        assert_eq!(listing.rank, 0);
        assert_eq!(listing.level, 0);
        assert_eq!(listing.xp, 0);
    }

    // --- Listing tests ---

    #[test]
    fn test_listing_accessors() {
        let listing = Listing {
            name: HiscoreName::Mining,
            rank: 500,
            level: 72,
            xp: 912345,
        };
        assert_eq!(listing.name, HiscoreName::Mining);
        assert_eq!(listing.rank, 500);
        assert_eq!(listing.level, 72);
        assert_eq!(listing.xp, 912345);
    }

    #[test]
    fn test_listing_actual_level() {
        let listing = Listing {
            name: HiscoreName::Attack,
            rank: 1,
            level: 50,
            xp: 13034431, // 99 xp
        };
        assert_eq!(listing.actual_level(), 99);
    }

    #[test]
    fn test_listing_next_level_default() {
        use crate::stats::{FilterBy, MutuallyExclusiveFlag, Prefix};
        let listing = Listing {
            name: HiscoreName::Attack,
            rank: 1,
            level: 50,
            xp: 166636, // level 55
        };
        let flags = StatsFlags {
            filter_by: FilterBy::None,
            filter_at: 0,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        assert_eq!(listing.next_level(&flags), 56);
    }

    #[test]
    fn test_listing_next_level_with_end() {
        use crate::stats::{FilterBy, MutuallyExclusiveFlag, Prefix};
        let listing = Listing {
            name: HiscoreName::Attack,
            rank: 1,
            level: 50,
            xp: 166636,
        };
        let flags = StatsFlags {
            filter_by: FilterBy::None,
            filter_at: 0,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 99,
            search: "".to_string(),
        };
        assert_eq!(listing.next_level(&flags), 99);
    }

    // --- Listings tests ---

    #[test]
    fn test_listings_new_and_skill() {
        let listings = Listings::new(vec![
            Listing {
                name: HiscoreName::Attack,
                rank: 1,
                level: 99,
                xp: 13034431,
            },
            Listing {
                name: HiscoreName::Mining,
                rank: 50,
                level: 72,
                xp: 912345,
            },
        ]);
        assert!(listings.skill("Attack").is_some());
        assert!(listings.skill("Mining").is_some());
        assert!(listings.skill("Prayer").is_none());
    }

    #[test]
    fn test_listings_retain_combat() {
        let mut listings = Listings::new(vec![
            Listing {
                name: HiscoreName::Attack,
                rank: 1,
                level: 99,
                xp: 13034431,
            },
            Listing {
                name: HiscoreName::Mining,
                rank: 50,
                level: 72,
                xp: 912345,
            },
            Listing {
                name: HiscoreName::Defence,
                rank: 5,
                level: 80,
                xp: 2000000,
            },
        ]);
        listings.retain_combat();
        assert_eq!(listings.iter().count(), 2);
        let names: Vec<_> = listings.iter().map(|l| l.name).collect();
        assert!(names.contains(&HiscoreName::Attack));
        assert!(names.contains(&HiscoreName::Defence));
        assert!(!names.contains(&HiscoreName::Mining));
    }

    #[test]
    fn test_listings_filter() {
        use crate::stats::{FilterBy, MutuallyExclusiveFlag, Prefix};
        let mut listings = Listings::new(vec![
            Listing {
                name: HiscoreName::Overall,
                rank: 1,
                level: 500,
                xp: 1000000,
            },
            Listing {
                name: HiscoreName::Attack,
                rank: 1,
                level: 99,
                xp: 13034431,
            },
            Listing {
                name: HiscoreName::Mining,
                rank: 50,
                level: 30,
                xp: 13363,
            },
        ]);
        let flags = StatsFlags {
            filter_by: FilterBy::GreaterThan,
            filter_at: 50,
            prefix: Prefix::None,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        };
        listings.filter(&flags);
        // Overall is always filtered out, Mining level 30 < 50, only Attack remains
        assert_eq!(listings.iter().count(), 1);
        assert_eq!(listings.iter().next().unwrap().name, HiscoreName::Attack);
    }

    // --- skill_id / skill_by_id tests ---

    #[test]
    fn test_skill_id() {
        assert_eq!(skill_id("Overall".to_string()), 0);
        assert_eq!(skill_id("Attack".to_string()), 1);
        assert_eq!(skill_id("Mining".to_string()), 15);
    }

    #[test]
    fn test_skill_by_id() {
        assert_eq!(skill_by_id(0), "Overall");
        assert_eq!(skill_by_id(1), "Attack");
        assert_eq!(skill_by_id(999), "Overall");
    }

    #[test]
    fn test_skill() {
        assert_eq!(skill("overall"), "Overall");
        assert_eq!(skill("stats"), "Overall");
        assert_eq!(skill("total"), "Overall");
        assert_eq!(skill("attack"), "Attack");
        assert_eq!(skill("att"), "Attack");
        assert_eq!(skill("defence"), "Defence");
        assert_eq!(skill("def"), "Defence");
        assert_eq!(skill("strength"), "Strength");
        assert_eq!(skill("str"), "Strength");
        assert_eq!(skill("hitpoints"), "Hitpoints");
        assert_eq!(skill("hp"), "Hitpoints");
        assert_eq!(skill("ranged"), "Ranged");
        assert_eq!(skill("range"), "Ranged");
        assert_eq!(skill("prayer"), "Prayer");
        assert_eq!(skill("pray"), "Prayer");
        assert_eq!(skill("magic"), "Magic");
        assert_eq!(skill("mage"), "Magic");
        assert_eq!(skill("cooking"), "Cooking");
        assert_eq!(skill("cook"), "Cooking");
        assert_eq!(skill("woodcutting"), "Woodcutting");
        assert_eq!(skill("wc"), "Woodcutting");
        assert_eq!(skill("fletching"), "Fletching");
        assert_eq!(skill("fletch"), "Fletching");
        assert_eq!(skill("fishing"), "Fishing");
        assert_eq!(skill("fish"), "Fishing");
        assert_eq!(skill("firemaking"), "Firemaking");
        assert_eq!(skill("fm"), "Firemaking");
        assert_eq!(skill("crafting"), "Crafting");
        assert_eq!(skill("craft"), "Crafting");
        assert_eq!(skill("smithing"), "Smithing");
        assert_eq!(skill("smith"), "Smithing");
        assert_eq!(skill("mining"), "Mining");
        assert_eq!(skill("mine"), "Mining");
        assert_eq!(skill("herblore"), "Herblore");
        assert_eq!(skill("herb"), "Herblore");
        assert_eq!(skill("agility"), "Agility");
        assert_eq!(skill("agil"), "Agility");
        assert_eq!(skill("thieving"), "Thieving");
        assert_eq!(skill("thief"), "Thieving");
        assert_eq!(skill("farming"), "Farming");
        assert_eq!(skill("farm"), "Farming");
        assert_eq!(skill("runecraft"), "Runecrafting");
        assert_eq!(skill("rc"), "Runecrafting");
        assert_eq!(skill("invalid"), "");
    }

    #[test]
    fn test_skills() {
        assert_eq!(skills().len(), 21);
        assert_eq!(
            skills(),
            vec![
                HiscoreName::Overall,
                HiscoreName::Attack,
                HiscoreName::Defence,
                HiscoreName::Strength,
                HiscoreName::Hitpoints,
                HiscoreName::Ranged,
                HiscoreName::Prayer,
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
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_level_to_xp() {
        assert_eq!(level_to_xp(1), 0);
        assert_eq!(level_to_xp(2), 83);
        assert_eq!(level_to_xp(3), 174);
        assert_eq!(level_to_xp(4), 276);
        assert_eq!(level_to_xp(5), 388);
        assert_eq!(level_to_xp(6), 512);
        assert_eq!(level_to_xp(7), 650);
        assert_eq!(level_to_xp(8), 801);
        assert_eq!(level_to_xp(9), 969);
        assert_eq!(level_to_xp(10), 1154);
        assert_eq!(level_to_xp(11), 1358);
        assert_eq!(level_to_xp(12), 1584);
        assert_eq!(level_to_xp(13), 1833);
        assert_eq!(level_to_xp(14), 2107);
        assert_eq!(level_to_xp(15), 2411);
        assert_eq!(level_to_xp(16), 2746);
        assert_eq!(level_to_xp(17), 3115);
        assert_eq!(level_to_xp(18), 3523);
        assert_eq!(level_to_xp(19), 3973);
        assert_eq!(level_to_xp(20), 4470);
        assert_eq!(level_to_xp(21), 5018);
        assert_eq!(level_to_xp(22), 5624);
        assert_eq!(level_to_xp(23), 6291);
        assert_eq!(level_to_xp(24), 7028);
        assert_eq!(level_to_xp(25), 7842);
        assert_eq!(level_to_xp(26), 8740);
        assert_eq!(level_to_xp(27), 9730);
        assert_eq!(level_to_xp(28), 10824);
        assert_eq!(level_to_xp(29), 12031);
        assert_eq!(level_to_xp(30), 13363);
        assert_eq!(level_to_xp(45), 61512);
        assert_eq!(level_to_xp(55), 166636);
        assert_eq!(level_to_xp(75), 1210421);
        assert_eq!(level_to_xp(92), 6517253);
        assert_eq!(level_to_xp(95), 8771558);
        assert_eq!(level_to_xp(96), 9684577);
        assert_eq!(level_to_xp(97), 10692629);
        assert_eq!(level_to_xp(98), 11805606);
        assert_eq!(level_to_xp(99), 13034431);
        assert_eq!(level_to_xp(100), 14391160);
        assert_eq!(level_to_xp(105), 23611006);
        assert_eq!(level_to_xp(110), 38737661);
        assert_eq!(level_to_xp(115), 63555443);
        assert_eq!(level_to_xp(120), 104273167);
        assert_eq!(level_to_xp(126), 188884740);
        assert_eq!(level_to_xp(127), 208545568);
    }

    #[test]
    fn test_xp_to_level() {
        assert_eq!(xp_to_level(0), 1);
        assert_eq!(xp_to_level(83), 2);
        assert_eq!(xp_to_level(174), 3);
        assert_eq!(xp_to_level(276), 4);
        assert_eq!(xp_to_level(388), 5);
        assert_eq!(xp_to_level(512), 6);
        assert_eq!(xp_to_level(650), 7);
        assert_eq!(xp_to_level(801), 8);
        assert_eq!(xp_to_level(969), 9);
        assert_eq!(xp_to_level(1154), 10);
        assert_eq!(xp_to_level(1358), 11);
        assert_eq!(xp_to_level(1584), 12);
        assert_eq!(xp_to_level(1833), 13);
        assert_eq!(xp_to_level(2107), 14);
        assert_eq!(xp_to_level(2411), 15);
        assert_eq!(xp_to_level(2746), 16);
        assert_eq!(xp_to_level(3115), 17);
        assert_eq!(xp_to_level(3523), 18);
        assert_eq!(xp_to_level(3973), 19);
        assert_eq!(xp_to_level(4470), 20);
        assert_eq!(xp_to_level(5018), 21);
        assert_eq!(xp_to_level(5624), 22);
        assert_eq!(xp_to_level(6291), 23);
        assert_eq!(xp_to_level(7028), 24);
        assert_eq!(xp_to_level(7842), 25);
        assert_eq!(xp_to_level(8740), 26);
        assert_eq!(xp_to_level(9730), 27);
        assert_eq!(xp_to_level(10824), 28);
        assert_eq!(xp_to_level(12031), 29);
        assert_eq!(xp_to_level(13363), 30);
        assert_eq!(xp_to_level(61512), 45);
        assert_eq!(xp_to_level(166636), 55);
        assert_eq!(xp_to_level(1210421), 75);
        assert_eq!(xp_to_level(6517253), 92);
        assert_eq!(xp_to_level(8771558), 95);
        assert_eq!(xp_to_level(9684577), 96);
        assert_eq!(xp_to_level(10692629), 97);
        assert_eq!(xp_to_level(11805606), 98);
        assert_eq!(xp_to_level(12352331), 98);
        assert_eq!(xp_to_level(13034431), 99);
        assert_eq!(xp_to_level(14391160), 100);
        assert_eq!(xp_to_level(23611006), 105);
        assert_eq!(xp_to_level(38737661), 110);
        assert_eq!(xp_to_level(63555443), 115);
        assert_eq!(xp_to_level(104273167), 120);
        assert_eq!(xp_to_level(188884740), 126);
        assert_eq!(xp_to_level(200000000), 126);
    }
}
