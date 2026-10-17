use crate::common::{skill_by_id, skill_id, skills};
use crate::stats::agility::{Agility, AgilityDetails};
use crate::stats::cooking::{Cooking, CookingDetails};
use crate::stats::crafting::{Crafting, CraftingDetails};
use crate::stats::firemaking::{Firemaking, FiremakingDetails};
use crate::stats::fishing::{Fishing, FishingDetails};
use crate::stats::fletching::{Fletching, FletchingDetails};
use crate::stats::herblore::{Herblore, HerbloreDetails};
use crate::stats::magic::{Magic, MagicDetails};
use crate::stats::mining::{Mining, MiningDetails};
use crate::stats::prayer::{Prayer, PrayerDetails, PrayerMultipliers};
use crate::stats::runecraft::{Runecraft, RunecraftDetails};
use crate::stats::smithing::{Smithing, SmithingDetails, SmithingMultipliers};
use crate::stats::thieving::{Thieving, ThievingDetails};
use crate::stats::woodcutting::{Woodcutting, WoodcuttingDetails};
pub use common::source::Source;

pub trait Skill {
    fn all() -> Vec<Self>
    where
        Self: Sized;
    fn defaults() -> Vec<Details>;
    fn details(&self) -> Details;
    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized;
}

#[derive(Clone, PartialOrd, PartialEq)]
#[allow(dead_code)]
pub enum Details {
    Agility(AgilityDetails),
    Cooking(CookingDetails),
    Crafting(CraftingDetails),
    Firemaking(FiremakingDetails),
    Fishing(FishingDetails),
    Fletching(FletchingDetails),
    Herblore(HerbloreDetails),
    Magic(MagicDetails),
    Mining(MiningDetails),
    Prayer(PrayerDetails),
    Runecraft(RunecraftDetails),
    Smithing(SmithingDetails),
    Thieving(ThievingDetails),
    Woodcutting(WoodcuttingDetails),
}

pub trait IntoString {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String;
}

impl Details {
    pub fn to_string(&self, s: &Source, xp_difference: f64) -> String {
        match self {
            Details::Agility(agility) => agility.to_string(s, xp_difference),
            Details::Cooking(cooking) => cooking.to_string(s, xp_difference),
            Details::Crafting(crafting) => crafting.to_string(s, xp_difference),
            Details::Firemaking(firemaking) => firemaking.to_string(s, xp_difference),
            Details::Fishing(fishing) => fishing.to_string(s, xp_difference),
            Details::Fletching(fletching) => fletching.to_string(s, xp_difference),
            Details::Herblore(herblore) => herblore.to_string(s, xp_difference),
            Details::Magic(magic) => magic.to_string(s, xp_difference),
            Details::Mining(mining) => mining.to_string(s, xp_difference),
            Details::Prayer(prayer) => prayer.to_string(s, xp_difference),
            Details::Runecraft(runecraft) => runecraft.to_string(s, xp_difference),
            Details::Smithing(smithing) => smithing.to_string(s, xp_difference),
            Details::Thieving(thieving) => thieving.to_string(s, xp_difference),
            Details::Woodcutting(woodcutting) => woodcutting.to_string(s, xp_difference),
        }
    }

    pub fn level(&self) -> u32 {
        match self {
            Details::Agility(agility) => agility.level,
            Details::Cooking(cooking) => cooking.level,
            Details::Crafting(crafting) => crafting.level,
            Details::Firemaking(firemaking) => firemaking.level,
            Details::Fishing(fishing) => fishing.level,
            Details::Fletching(fletching) => fletching.level,
            Details::Herblore(herblore) => herblore.level,
            Details::Magic(magic) => magic.level,
            Details::Mining(mining) => mining.level,
            Details::Prayer(prayer) => prayer.level,
            Details::Runecraft(runecraft) => runecraft.level,
            Details::Smithing(smithing) => smithing.level,
            Details::Thieving(thieving) => thieving.level,
            Details::Woodcutting(woodcutting) => woodcutting.level,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> String {
        if let Some(result) = skills().get(self.skill_id() as usize) {
            return result.clone();
        };

        "".to_owned()
    }

    #[allow(dead_code)]
    pub fn skill_id(&self) -> u32 {
        skill_id(self.skill())
    }

    pub fn skill(&self) -> String {
        match self {
            Details::Agility(_) => "Agility",
            Details::Cooking(_) => "Cooking",
            Details::Crafting(_) => "Crafting",
            Details::Firemaking(_) => "Firemaking",
            Details::Fishing(_) => "Fishing",
            Details::Fletching(_) => "Fletching",
            Details::Herblore(_) => "Herblore",
            Details::Magic(_) => "Magic",
            Details::Mining(_) => "Mining",
            Details::Prayer(_) => "Prayer",
            Details::Runecraft(_) => "Runecrafting",
            Details::Smithing(_) => "Smithing",
            Details::Thieving(_) => "Thieving",
            Details::Woodcutting(_) => "Woodcutting",
        }
        .to_string()
    }

    #[allow(dead_code)]
    pub fn multipliers<T>(details: T) -> Vec<Multipliers>
    where
        T: Detail,
    {
        details.multipliers()
    }
}

pub trait Detail {
    fn multipliers(&self) -> Vec<Multipliers>;
    fn name(&self) -> String;
    #[allow(dead_code)]
    fn level(&self) -> u32;
    #[allow(dead_code)]
    fn xp(&self) -> f64;
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Multipliers {
    Prayer(PrayerMultipliers),
    Smithing(SmithingMultipliers),
}

/// How many actions to fall back to when none of a skill's defaults are
/// unlocked yet.
const FALLBACK_COUNT: usize = 5;

/// The skill's defaults the player can do at `level`. When none are unlocked
/// yet, the highest-level actions they can do instead, lowest first.
fn defaults_at<T: Skill>(level: u32) -> Vec<Details> {
    let mut defaults = T::defaults();
    defaults.retain(|detail| detail.level() <= level);

    if !defaults.is_empty() {
        return defaults;
    }

    let mut doable = T::all()
        .iter()
        .map(|x| x.details())
        .filter(|detail| detail.level() <= level)
        .collect::<Vec<Details>>();
    doable.sort_by_key(|detail| detail.level());

    let skip = doable.len().saturating_sub(FALLBACK_COUNT);

    doable.split_off(skip)
}

pub fn details_by_skill_id(id: u32, query: &str, level: u32) -> Vec<Details> {
    if query.len() == 0 {
        return match skill_by_id(id).as_str() {
            "Agility" => defaults_at::<Agility>(level),
            "Cooking" => defaults_at::<Cooking>(level),
            "Crafting" => defaults_at::<Crafting>(level),
            "Firemaking" => defaults_at::<Firemaking>(level),
            "Fishing" => defaults_at::<Fishing>(level),
            "Fletching" => defaults_at::<Fletching>(level),
            "Herblore" => defaults_at::<Herblore>(level),
            "Magic" => defaults_at::<Magic>(level),
            "Mining" => defaults_at::<Mining>(level),
            "Prayer" => defaults_at::<Prayer>(level),
            "Runecrafting" => defaults_at::<Runecraft>(level),
            "Smithing" => defaults_at::<Smithing>(level),
            "Thieving" => defaults_at::<Thieving>(level),
            "Woodcutting" => defaults_at::<Woodcutting>(level),
            _ => vec![],
        };
    }

    match skill_by_id(id).as_str() {
        "Agility" => Agility::search(query).iter().map(|x| x.details()).collect(),
        "Cooking" => Cooking::search(query).iter().map(|x| x.details()).collect(),
        "Crafting" => Crafting::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Firemaking" => Firemaking::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Fishing" => Fishing::search(query).iter().map(|x| x.details()).collect(),
        "Fletching" => Fletching::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Herblore" => Herblore::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Magic" => Magic::search(query).iter().map(|x| x.details()).collect(),
        "Mining" => Mining::search(query).iter().map(|x| x.details()).collect(),
        "Prayer" => Prayer::search(query).iter().map(|x| x.details()).collect(),
        "Runecrafting" => Runecraft::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Smithing" => Smithing::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Thieving" => Thieving::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Woodcutting" => Woodcutting::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_drop_actions_above_the_players_level() {
        let defaults = defaults_at::<Crafting>(60);

        assert!(!defaults.is_empty());
        assert!(defaults.iter().all(|detail| detail.level() <= 60));
        assert!(defaults.len() < Crafting::defaults().len());
    }

    #[test]
    fn defaults_are_unchanged_once_every_one_is_unlocked() {
        assert!(defaults_at::<Crafting>(99) == Crafting::defaults());
    }

    #[test]
    fn defaults_fall_back_to_the_best_doable_actions() {
        // Cooking's defaults start at Lobster (40).
        let defaults = defaults_at::<Cooking>(20);

        assert!(!defaults.is_empty());
        assert!(defaults.len() <= FALLBACK_COUNT);
        assert!(defaults.iter().all(|detail| detail.level() <= 20));
        assert!(defaults.windows(2).all(|w| w[0].level() <= w[1].level()));
    }

    #[test]
    fn a_query_ignores_the_players_level() {
        let results = details_by_skill_id(skill_id("Crafting".to_string()), "battlestaff", 1);

        assert!(results.iter().any(|detail| detail.level() > 1));
    }
}
