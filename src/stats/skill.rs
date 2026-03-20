use crate::common::{skill_by_id, skill_id, skills};
use crate::stats::agility::{Agility, AgilityDetails, AgilityMultipliers};
use crate::stats::cooking::{Cooking, CookingDetails};
use crate::stats::crafting::{Crafting, CraftingDetails};
use crate::stats::firemaking::{Firemaking, FiremakingDetails, FiremakingMultipliers};
use crate::stats::fishing::{Fishing, FishingDetails, FishingMultipliers};
use crate::stats::fletching::{Fletching, FletchingDetails};
use crate::stats::herblore::{Herblore, HerbloreDetails};
use crate::stats::magic::{Magic, MagicDetails};
use crate::stats::mining::{Mining, MiningDetails, MiningMultipliers};
use crate::stats::prayer::{Prayer, PrayerDetails, PrayerMultipliers};
use crate::stats::runecraft::{Runecraft, RunecraftDetails, RunecraftMultipliers};
use crate::stats::smithing::{Smithing, SmithingDetails, SmithingMultipliers};
use crate::stats::thieving::{Thieving, ThievingDetails};
use crate::stats::woodcutting::{Woodcutting, WoodcuttingDetails, WoodcuttingMultipliers};
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
            Details::Runecraft(_) => "Runecraft",
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
    Agility(AgilityMultipliers),
    Firemaking(FiremakingMultipliers),
    Fishing(FishingMultipliers),
    #[allow(dead_code)]
    Mining(MiningMultipliers),
    Prayer(PrayerMultipliers),
    Runecraft(RunecraftMultipliers),
    Smithing(SmithingMultipliers),
    Woodcutting(WoodcuttingMultipliers),
}

pub fn details_by_skill_id(id: u32, query: &str) -> Vec<Details> {
    if query.len() == 0 {
        return match skill_by_id(id).as_str() {
            "Agility" => Agility::defaults(),
            "Cooking" => Cooking::defaults(),
            "Crafting" => Crafting::defaults(),
            "Firemaking" => Firemaking::defaults(),
            "Fishing" => Fishing::defaults(),
            "Fletching" => Fletching::defaults(),
            "Herblore" => Herblore::defaults(),
            "Magic" => Magic::defaults(),
            "Mining" => Mining::defaults(),
            "Prayer" => Prayer::defaults(),
            "Runecraft" => Runecraft::defaults(),
            "Smithing" => Smithing::defaults(),
            "Thieving" => Thieving::defaults(),
            "Woodcutting" => Woodcutting::defaults(),
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
        "Runecraft" => Runecraft::search(query)
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
