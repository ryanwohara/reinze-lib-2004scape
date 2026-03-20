use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Cooking {
    Sinew,
    Anchovies,
    CookedChicken,
    CookedMeat,
    CookedRabbit,
    Shrimps,
    Bread,
    Sardine,
    UgthankiMeat,
    PoisonKarambwan,
    Herring,
    FruitBlast,
    Guppy,
    BakedPotato,
    PineapplePunch,
    SpicySauce,
    Mackerel,
    RedberryPie,
    ToadCrunchies,
    ChilliConCarne,
    RoastBirdMeat,
    ThinSnailMeat,
    SpicyCrunchies,
    ScrambledEgg,
    WormCrunchies,
    Cider,
    Trout,
    RoastRabbit,
    SpiderOnShaft,
    SpiderOnStick,
    ChocchipCrunchies,
    LeanSnailMeat,
    Cod,
    WizardBlizzard,
    DwarvenStout,
    Cavefish,
    CupOfTea,
    Pike,
    MeatPie,
    ShortGreenGuy,
    PotOfCream,
    RoastBeastMeat,
    CookedCrabMeat,
    FatSnailMeat,
    EggAndTomato,
    CookedWildKebbit,
    AsgarnianAle,
    Salmon,
    Stew,
    FruitBatta,
    ToadBatta,
    WormBatta,
    CookedSlimyEel,
    Sweetcorn,
    VegetableBatta,
    MudPie,
    CheeseAndTomatoBatta,
    GreenmansAle,
    CookedBream,
    CookedMossLizard,
    RoastedChompy,
    Tuna,
    ApplePie,
    WormHole,
    CookedKarambwan,
    CookedLarupia,
    Fishcake,
    CookedBarbtailedKebbit,
    DrunkDragon,
    Tetra,
    ChocSaturday,
    GardenPie,
    WizardsMindBomb,
    RainbowFish,
    PlainPizza,
    VegBall,
    JugOfWine,
    BlurberrySpecial,
    PatOfButter,
    CaveEel,
    PotatoWithButter,
    DragonBitter,
    Lobster,
    Cake,
    TangledToadsLegs,
    CookedGraahk,
    CookedJubbly,
    ChilliPotato,
    FriedOnions,
    ChocolateBomb,
    Bass,
    MoonlightMead,
    Swordfish,
    MeatPizza,
    Catfish,
    FriedMushrooms,
    PotatoWithCheese,
    FishPie,
    Cheese,
    AxemansFolly,
    CookedOomlieWrap,
    ChocolateCake,
    CookedKyatt,
    EggPotato,
    BotanicalPie,
    LavaEel,
    ChefsDelight,
    AnchovyPizza,
    MushroomAndOnion,
    PittaBread,
    UgthankiKebabFresh,
    SlayersRespite,
    MushroomPie,
    Curry,
    Monkfish,
    MushroomPotato,
    PineapplePizza,
    WineOfZamorak,
    TunaAndCorn,
    CookedSunlightAntelope,
    TunaPotato,
    AdmiralPie,
    SacredEel,
    DragonfruitPie,
    Shark,
    SeaTurtle,
    CookedDashingKebbit,
    Anglerfish,
    WildPie,
    DarkCrab,
    MantaRay,
    CookedMoonlightAntelope,
    SummerPie,
}

impl Skill for Cooking {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Sinew,
            Self::Anchovies,
            Self::CookedChicken,
            Self::CookedMeat,
            Self::CookedRabbit,
            Self::Shrimps,
            Self::Bread,
            Self::Sardine,
            Self::UgthankiMeat,
            Self::PoisonKarambwan,
            Self::Herring,
            Self::FruitBlast,
            Self::Guppy,
            Self::BakedPotato,
            Self::PineapplePunch,
            Self::SpicySauce,
            Self::Mackerel,
            Self::RedberryPie,
            Self::ToadCrunchies,
            Self::ChilliConCarne,
            Self::RoastBirdMeat,
            Self::ThinSnailMeat,
            Self::SpicyCrunchies,
            Self::ScrambledEgg,
            Self::WormCrunchies,
            Self::Cider,
            Self::Trout,
            Self::RoastRabbit,
            Self::SpiderOnShaft,
            Self::SpiderOnStick,
            Self::ChocchipCrunchies,
            Self::LeanSnailMeat,
            Self::Cod,
            Self::WizardBlizzard,
            Self::DwarvenStout,
            Self::Cavefish,
            Self::CupOfTea,
            Self::Pike,
            Self::MeatPie,
            Self::ShortGreenGuy,
            Self::PotOfCream,
            Self::RoastBeastMeat,
            Self::CookedCrabMeat,
            Self::FatSnailMeat,
            Self::EggAndTomato,
            Self::CookedWildKebbit,
            Self::AsgarnianAle,
            Self::Salmon,
            Self::Stew,
            Self::FruitBatta,
            Self::ToadBatta,
            Self::WormBatta,
            Self::CookedSlimyEel,
            Self::Sweetcorn,
            Self::VegetableBatta,
            Self::MudPie,
            Self::CheeseAndTomatoBatta,
            Self::GreenmansAle,
            Self::CookedBream,
            Self::CookedMossLizard,
            Self::RoastedChompy,
            Self::Tuna,
            Self::ApplePie,
            Self::WormHole,
            Self::CookedKarambwan,
            Self::CookedLarupia,
            Self::Fishcake,
            Self::CookedBarbtailedKebbit,
            Self::DrunkDragon,
            Self::Tetra,
            Self::ChocSaturday,
            Self::GardenPie,
            Self::WizardsMindBomb,
            Self::RainbowFish,
            Self::PlainPizza,
            Self::VegBall,
            Self::JugOfWine,
            Self::BlurberrySpecial,
            Self::PatOfButter,
            Self::CaveEel,
            Self::PotatoWithButter,
            Self::DragonBitter,
            Self::Lobster,
            Self::Cake,
            Self::TangledToadsLegs,
            Self::CookedGraahk,
            Self::CookedJubbly,
            Self::ChilliPotato,
            Self::FriedOnions,
            Self::ChocolateBomb,
            Self::Bass,
            Self::MoonlightMead,
            Self::Swordfish,
            Self::MeatPizza,
            Self::Catfish,
            Self::FriedMushrooms,
            Self::PotatoWithCheese,
            Self::FishPie,
            Self::Cheese,
            Self::AxemansFolly,
            Self::CookedOomlieWrap,
            Self::ChocolateCake,
            Self::CookedKyatt,
            Self::EggPotato,
            Self::BotanicalPie,
            Self::LavaEel,
            Self::ChefsDelight,
            Self::AnchovyPizza,
            Self::MushroomAndOnion,
            Self::PittaBread,
            Self::UgthankiKebabFresh,
            Self::SlayersRespite,
            Self::MushroomPie,
            Self::Curry,
            Self::Monkfish,
            Self::MushroomPotato,
            Self::PineapplePizza,
            Self::WineOfZamorak,
            Self::TunaAndCorn,
            Self::CookedSunlightAntelope,
            Self::TunaPotato,
            Self::AdmiralPie,
            Self::SacredEel,
            Self::DragonfruitPie,
            Self::Shark,
            Self::SeaTurtle,
            Self::CookedDashingKebbit,
            Self::Anglerfish,
            Self::WildPie,
            Self::DarkCrab,
            Self::MantaRay,
            Self::CookedMoonlightAntelope,
            Self::SummerPie,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![Self::Lobster, Self::Swordfish, Self::Monkfish, Self::Shark]
            .iter()
            .map(|x| x.details())
            .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Sinew => ("Sinew", 1, 3.0),
            Self::Anchovies => ("Anchovies", 1, 30.0),
            Self::CookedChicken => ("Cooked Chicken", 1, 30.0),
            Self::CookedMeat => ("Cooked Meat", 1, 30.0),
            Self::CookedRabbit => ("Cooked Rabbit", 1, 30.0),
            Self::Shrimps => ("Shrimps", 1, 30.0),
            Self::Bread => ("Bread", 1, 40.0),
            Self::Sardine => ("Sardine", 1, 40.0),
            Self::UgthankiMeat => ("Ugthanki Meat", 1, 40.0),
            Self::PoisonKarambwan => ("Poison Karambwan", 1, 80.0),
            Self::Herring => ("Herring", 5, 50.0),
            Self::FruitBlast => ("Fruit Blast", 6, 50.0),
            Self::Guppy => ("Guppy", 7, 12.0),
            Self::BakedPotato => ("Baked Potato", 7, 15.0),
            Self::PineapplePunch => ("Pineapple Punch", 8, 70.0),
            Self::SpicySauce => ("Spicy Sauce", 9, 25.0),
            Self::Mackerel => ("Mackerel", 10, 60.0),
            Self::RedberryPie => ("Redberry Pie", 10, 78.0),
            Self::ToadCrunchies => ("Toad Crunchies", 10, 100.0),
            Self::ChilliConCarne => ("Chilli Con Carne", 11, 55.0),
            Self::RoastBirdMeat => ("Roast Bird Meat", 11, 62.5),
            Self::ThinSnailMeat => ("Thin Snail Meat", 12, 70.0),
            Self::SpicyCrunchies => ("Spicy Crunchies", 12, 100.0),
            Self::ScrambledEgg => ("Scrambled Egg", 13, 50.0),
            Self::WormCrunchies => ("Worm Crunchies", 14, 104.0),
            Self::Cider => ("Cider", 14, 182.0),
            Self::Trout => ("Trout", 15, 70.0),
            Self::RoastRabbit => ("Roast Rabbit", 16, 72.5),
            Self::SpiderOnShaft => ("Spider On Shaft", 16, 80.0),
            Self::SpiderOnStick => ("Spider On Stick", 16, 80.0),
            Self::ChocchipCrunchies => ("Chocchip Crunchies", 16, 100.0),
            Self::LeanSnailMeat => ("Lean Snail Meat", 17, 80.0),
            Self::Cod => ("Cod", 18, 75.0),
            Self::WizardBlizzard => ("Wizard Blizzard", 18, 110.0),
            Self::DwarvenStout => ("Dwarven Stout", 19, 215.0),
            Self::Cavefish => ("Cavefish", 20, 23.0),
            Self::CupOfTea => ("Cup Of Tea", 20, 52.0),
            Self::Pike => ("Pike", 20, 80.0),
            Self::MeatPie => ("Meat Pie", 20, 110.0),
            Self::ShortGreenGuy => ("Short Green Guy", 20, 120.0),
            Self::PotOfCream => ("Pot Of Cream", 21, 18.0),
            Self::RoastBeastMeat => ("Roast Beast Meat", 21, 82.5),
            Self::CookedCrabMeat => ("Cooked Crab Meat", 21, 100.0),
            Self::FatSnailMeat => ("Fat Snail Meat", 22, 95.0),
            Self::EggAndTomato => ("Egg And Tomato", 23, 50.0),
            Self::CookedWildKebbit => ("Cooked Wild Kebbit", 23, 73.0),
            Self::AsgarnianAle => ("Asgarnian Ale", 24, 248.0),
            Self::Salmon => ("Salmon", 25, 90.0),
            Self::Stew => ("Stew", 25, 117.0),
            Self::FruitBatta => ("Fruit Batta", 25, 150.0),
            Self::ToadBatta => ("Toad Batta", 26, 152.0),
            Self::WormBatta => ("Worm Batta", 27, 154.0),
            Self::CookedSlimyEel => ("Cooked Slimy Eel", 28, 95.0),
            Self::Sweetcorn => ("Sweetcorn", 28, 104.0),
            Self::VegetableBatta => ("Vegetable Batta", 28, 156.0),
            Self::MudPie => ("Mud Pie", 29, 128.0),
            Self::CheeseAndTomatoBatta => ("Cheese And Tomato Batta", 29, 158.0),
            Self::GreenmansAle => ("Greenmans Ale", 29, 281.0),
            Self::CookedBream => ("Cooked Bream", 30, 45.0),
            Self::CookedMossLizard => ("Cooked Moss Lizard", 30, 60.0),
            Self::RoastedChompy => ("Roasted Chompy", 30, 100.0),
            Self::Tuna => ("Tuna", 30, 100.0),
            Self::ApplePie => ("Apple Pie", 30, 130.0),
            Self::WormHole => ("Worm Hole", 30, 170.0),
            Self::CookedKarambwan => ("Cooked Karambwan", 30, 190.0),
            Self::CookedLarupia => ("Cooked Larupia", 31, 92.0),
            Self::Fishcake => ("Fishcake", 31, 100.0),
            Self::CookedBarbtailedKebbit => ("Cooked Barbtailed Kebbit", 32, 106.0),
            Self::DrunkDragon => ("Drunk Dragon", 32, 160.0),
            Self::Tetra => ("Tetra", 33, 31.0),
            Self::ChocSaturday => ("Choc Saturday", 33, 170.0),
            Self::GardenPie => ("Garden Pie", 34, 138.0),
            Self::WizardsMindBomb => ("Wizards Mind Bomb", 34, 314.0),
            Self::RainbowFish => ("Rainbow Fish", 35, 110.0),
            Self::PlainPizza => ("Plain Pizza", 35, 143.0),
            Self::VegBall => ("Veg Ball", 35, 175.0),
            Self::JugOfWine => ("Jug Of Wine", 35, 200.0),
            Self::BlurberrySpecial => ("Blurberry Special", 37, 180.0),
            Self::PatOfButter => ("Pat Of Butter", 38, 40.5),
            Self::CaveEel => ("Cave Eel", 38, 115.0),
            Self::PotatoWithButter => ("Potato With Butter", 39, 40.0),
            Self::DragonBitter => ("Dragon Bitter", 39, 347.0),
            Self::Lobster => ("Lobster", 40, 120.0),
            Self::Cake => ("Cake", 40, 180.0),
            Self::TangledToadsLegs => ("Tangled Toads Legs", 40, 185.0),
            Self::CookedGraahk => ("Cooked Graahk", 41, 124.0),
            Self::CookedJubbly => ("Cooked Jubbly", 41, 160.0),
            Self::ChilliPotato => ("Chilli Potato", 41, 165.5),
            Self::FriedOnions => ("Fried Onions", 42, 60.0),
            Self::ChocolateBomb => ("Chocolate Bomb", 42, 190.0),
            Self::Bass => ("Bass", 43, 130.0),
            Self::MoonlightMead => ("Moonlight Mead", 44, 380.0),
            Self::Swordfish => ("Swordfish", 45, 140.0),
            Self::MeatPizza => ("Meat Pizza", 45, 169.0),
            Self::Catfish => ("Catfish", 46, 43.0),
            Self::FriedMushrooms => ("Fried Mushrooms", 46, 60.0),
            Self::PotatoWithCheese => ("Potato With Cheese", 47, 40.0),
            Self::FishPie => ("Fish Pie", 47, 164.0),
            Self::Cheese => ("Cheese", 48, 64.0),
            Self::AxemansFolly => ("Axemans Folly", 49, 413.0),
            Self::CookedOomlieWrap => ("Cooked Oomlie Wrap", 50, 30.0),
            Self::ChocolateCake => ("Chocolate Cake", 50, 210.0),
            Self::CookedKyatt => ("Cooked Kyatt", 51, 143.0),
            Self::EggPotato => ("Egg Potato", 51, 195.5),
            Self::BotanicalPie => ("Botanical Pie", 52, 180.0),
            Self::LavaEel => ("Lava Eel", 53, 30.0),
            Self::ChefsDelight => ("Chefs Delight", 54, 446.0),
            Self::AnchovyPizza => ("Anchovy Pizza", 55, 182.0),
            Self::MushroomAndOnion => ("Mushroom And Onion", 57, 120.0),
            Self::PittaBread => ("Pitta Bread", 58, 40.0),
            Self::UgthankiKebabFresh => ("Ugthanki Kebab Fresh", 58, 80.0),
            Self::SlayersRespite => ("Slayers Respite", 59, 479.0),
            Self::MushroomPie => ("Mushroom Pie", 60, 200.0),
            Self::Curry => ("Curry", 60, 280.0),
            Self::Monkfish => ("Monkfish", 62, 150.0),
            Self::MushroomPotato => ("Mushroom Potato", 64, 270.5),
            Self::PineapplePizza => ("Pineapple Pizza", 65, 188.0),
            Self::WineOfZamorak => ("Wine Of Zamorak", 65, 200.0),
            Self::TunaAndCorn => ("Tuna And Corn", 67, 204.0),
            Self::CookedSunlightAntelope => ("Cooked Sunlight Antelope", 68, 175.0),
            Self::TunaPotato => ("Tuna Potato", 68, 309.5),
            Self::AdmiralPie => ("Admiral Pie", 70, 210.0),
            Self::SacredEel => ("Sacred Eel", 72, 109.0),
            Self::DragonfruitPie => ("Dragonfruit Pie", 73, 220.0),
            Self::Shark => ("Shark", 80, 210.0),
            Self::SeaTurtle => ("Sea Turtle", 82, 211.3),
            Self::CookedDashingKebbit => ("Cooked Dashing Kebbit", 82, 215.0),
            Self::Anglerfish => ("Anglerfish", 84, 230.0),
            Self::WildPie => ("Wild Pie", 85, 240.0),
            Self::DarkCrab => ("Dark Crab", 90, 215.0),
            Self::MantaRay => ("Manta Ray", 91, 216.3),
            Self::CookedMoonlightAntelope => ("Cooked Moonlight Antelope", 92, 220.0),
            Self::SummerPie => ("Summer Pie", 95, 260.0),
        };

        Details::Cooking(CookingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![],
        })
    }

    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized,
    {
        let mut all = Self::all();

        let q = query.to_string().to_lowercase();

        if let Ok(pattern) = Regex::new(q.as_str()) {
            let mut index = 0;
            all.retain(|activity| {
                if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 10
                {
                    index = index.add(1);

                    return true;
                }

                return false;
            });
        } else {
            return vec![];
        }

        all
    }
}

impl Detail for Cooking {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Cooking(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Cooking(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Cooking(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct CookingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for CookingDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }
}
