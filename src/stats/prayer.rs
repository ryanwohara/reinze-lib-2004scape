use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Prayer {
    Guppy,
    Bones,
    BurntBones,
    WolfBones,
    MonkeyBones,
    BatBones,
    Cavefish,
    FiendishAshes,
    Tetra,
    BigBones,
    JogreBones,
    Catfish,
    WyrmlingBones,
    ZogreBones,
    ShaikahanBones,
    VileAshes,
    BabydragonBones,
    LoarRemains,
    PhrinRemains,
    WyrmBones,
    RiylRemains,
    MaliciousAshes,
    DragonBones,
    WyvernBones,
    DrakeBones,
    AsynRemains,
    FayrgBones,
    FiyrRemains,
    AbyssalAshes,
    LavaDragonBones,
    RaurgBones,
    HydraBones,
    InfernalAshes,
    UriumRemains,
    DagannothBones,
    EnsouledGoblinHead,
    OurgBones,
    EnsouledMonkeyHead,
    EnsouledImpHead,
    EnsouledMinotaurHead,
    EnsouledScorpionHead,
    EnsouledBearHead,
    EnsouledUnicornHead,
    EnsouledDogHead,
    EnsouledChaosDruidHead,
    EnsouledGiantHead,
    EnsouledOgreHead,
    EnsouledElfHead,
    EnsouledTrollHead,
    EnsouledHorrorHead,
    EnsouledKalphiteHead,
    EnsouledDagannothHead,
    EnsouledBloodveldHead,
    EnsouledTzhaarHead,
    EnsouledDemonHead,
    EnsouledHellhoundHead,
    EnsouledAviansieHead,
    EnsouledAbyssalHead,
    EnsouledDragonHead,
    BlessedBoneShards,
    SuperiorDragonBones,
}

impl Detail for Prayer {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Prayer(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Prayer(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Prayer(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Prayer {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Guppy,
            Self::Bones,
            Self::BurntBones,
            Self::WolfBones,
            Self::MonkeyBones,
            Self::BatBones,
            Self::Cavefish,
            Self::FiendishAshes,
            Self::Tetra,
            Self::BigBones,
            Self::JogreBones,
            Self::Catfish,
            Self::WyrmlingBones,
            Self::ZogreBones,
            Self::ShaikahanBones,
            Self::VileAshes,
            Self::BabydragonBones,
            Self::LoarRemains,
            Self::PhrinRemains,
            Self::WyrmBones,
            Self::RiylRemains,
            Self::MaliciousAshes,
            Self::DragonBones,
            Self::WyvernBones,
            Self::DrakeBones,
            Self::AsynRemains,
            Self::FayrgBones,
            Self::FiyrRemains,
            Self::AbyssalAshes,
            Self::LavaDragonBones,
            Self::RaurgBones,
            Self::HydraBones,
            Self::InfernalAshes,
            Self::UriumRemains,
            Self::DagannothBones,
            Self::EnsouledGoblinHead,
            Self::OurgBones,
            Self::EnsouledMonkeyHead,
            Self::EnsouledImpHead,
            Self::EnsouledMinotaurHead,
            Self::EnsouledScorpionHead,
            Self::EnsouledBearHead,
            Self::EnsouledUnicornHead,
            Self::EnsouledDogHead,
            Self::EnsouledChaosDruidHead,
            Self::EnsouledGiantHead,
            Self::EnsouledOgreHead,
            Self::EnsouledElfHead,
            Self::EnsouledTrollHead,
            Self::EnsouledHorrorHead,
            Self::EnsouledKalphiteHead,
            Self::EnsouledDagannothHead,
            Self::EnsouledBloodveldHead,
            Self::EnsouledTzhaarHead,
            Self::EnsouledDemonHead,
            Self::EnsouledHellhoundHead,
            Self::EnsouledAviansieHead,
            Self::EnsouledAbyssalHead,
            Self::EnsouledDragonHead,
            Self::BlessedBoneShards,
            Self::SuperiorDragonBones,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::DragonBones,
            Self::SuperiorDragonBones,
            Self::InfernalAshes,
            Self::EnsouledDragonHead,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Guppy => ("Guppy", 1, 4.0, PrayerMethod::Fish),
            Self::Bones => ("Bones", 1, 4.5, PrayerMethod::Bones),
            Self::BurntBones => ("Burnt Bones", 1, 4.5, PrayerMethod::Bones),
            Self::WolfBones => ("Wolf Bones", 1, 4.5, PrayerMethod::Bones),
            Self::MonkeyBones => ("Monkey Bones", 1, 5.0, PrayerMethod::Bones),
            Self::BatBones => ("Bat Bones", 1, 5.3, PrayerMethod::Bones),
            Self::Cavefish => ("Cavefish", 1, 7.0, PrayerMethod::Fish),
            Self::FiendishAshes => ("Fiendish Ashes", 1, 10.0, PrayerMethod::Ashes),
            Self::Tetra => ("Tetra", 1, 10.0, PrayerMethod::Fish),
            Self::BigBones => ("Big Bones", 1, 15.0, PrayerMethod::Bones),
            Self::JogreBones => ("Jogre Bones", 1, 15.0, PrayerMethod::Bones),
            Self::Catfish => ("Catfish", 1, 16.0, PrayerMethod::Fish),
            Self::WyrmlingBones => ("Wyrmling Bones", 1, 21.0, PrayerMethod::Bones),
            Self::ZogreBones => ("Zogre Bones", 1, 22.5, PrayerMethod::Bones),
            Self::ShaikahanBones => ("Shaikahan Bones", 1, 25.0, PrayerMethod::Bones),
            Self::VileAshes => ("Vile Ashes", 1, 25.0, PrayerMethod::Ashes),
            Self::BabydragonBones => ("Babydragon Bones", 1, 30.0, PrayerMethod::Bones),
            Self::LoarRemains => ("Loar Remains", 1, 33.0, PrayerMethod::Shade),
            Self::PhrinRemains => ("Phrin Remains", 1, 46.5, PrayerMethod::Shade),
            Self::WyrmBones => ("Wyrm Bones", 1, 50.0, PrayerMethod::Bones),
            Self::RiylRemains => ("Riyl Remains", 1, 59.5, PrayerMethod::Shade),
            Self::MaliciousAshes => ("Malicious Ashes", 1, 65.0, PrayerMethod::Ashes),
            Self::DragonBones => ("Dragon Bones", 1, 72.0, PrayerMethod::Bones),
            Self::WyvernBones => ("Wyvern Bones", 1, 72.0, PrayerMethod::Bones),
            Self::DrakeBones => ("Drake Bones", 1, 80.0, PrayerMethod::Bones),
            Self::AsynRemains => ("Asyn Remains", 1, 82.5, PrayerMethod::Shade),
            Self::FayrgBones => ("Fayrg Bones", 1, 84.0, PrayerMethod::Bones),
            Self::FiyrRemains => ("Fiyr Remains", 1, 84.0, PrayerMethod::Shade),
            Self::AbyssalAshes => ("Abyssal Ashes", 1, 85.0, PrayerMethod::Ashes),
            Self::LavaDragonBones => ("Lava Dragon Bones", 1, 85.0, PrayerMethod::Bones),
            Self::RaurgBones => ("Raurg Bones", 1, 96.0, PrayerMethod::Bones),
            Self::HydraBones => ("Hydra Bones", 1, 110.0, PrayerMethod::Bones),
            Self::InfernalAshes => ("Infernal Ashes", 1, 110.0, PrayerMethod::Ashes),
            Self::UriumRemains => ("Urium Remains", 1, 120.5, PrayerMethod::Shade),
            Self::DagannothBones => ("Dagannoth Bones", 1, 125.0, PrayerMethod::Bones),
            Self::EnsouledGoblinHead => {
                ("Ensouled Goblin Head", 1, 130.0, PrayerMethod::EnsouledHead)
            }
            Self::OurgBones => ("Ourg Bones", 1, 140.0, PrayerMethod::Bones),
            Self::EnsouledMonkeyHead => {
                ("Ensouled Monkey Head", 1, 182.0, PrayerMethod::EnsouledHead)
            }
            Self::EnsouledImpHead => ("Ensouled Imp Head", 1, 286.0, PrayerMethod::EnsouledHead),
            Self::EnsouledMinotaurHead => (
                "Ensouled Minotaur Head",
                1,
                364.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledScorpionHead => (
                "Ensouled Scorpion Head",
                1,
                454.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledBearHead => ("Ensouled Bear Head", 1, 480.0, PrayerMethod::EnsouledHead),
            Self::EnsouledUnicornHead => (
                "Ensouled Unicorn Head",
                1,
                494.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledDogHead => ("Ensouled Dog Head", 1, 520.0, PrayerMethod::EnsouledHead),
            Self::EnsouledChaosDruidHead => (
                "Ensouled Chaos Druid Head",
                1,
                584.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledGiantHead => {
                ("Ensouled Giant Head", 1, 650.0, PrayerMethod::EnsouledHead)
            }
            Self::EnsouledOgreHead => ("Ensouled Ogre Head", 1, 716.0, PrayerMethod::EnsouledHead),
            Self::EnsouledElfHead => ("Ensouled Elf Head", 1, 754.0, PrayerMethod::EnsouledHead),
            Self::EnsouledTrollHead => {
                ("Ensouled Troll Head", 1, 780.0, PrayerMethod::EnsouledHead)
            }
            Self::EnsouledHorrorHead => {
                ("Ensouled Horror Head", 1, 832.0, PrayerMethod::EnsouledHead)
            }
            Self::EnsouledKalphiteHead => (
                "Ensouled Kalphite Head",
                1,
                884.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledDagannothHead => (
                "Ensouled Dagannoth Head",
                1,
                936.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledBloodveldHead => (
                "Ensouled Bloodveld Head",
                1,
                1040.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledTzhaarHead => (
                "Ensouled Tzhaar Head",
                1,
                1104.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledDemonHead => {
                ("Ensouled Demon Head", 1, 1170.0, PrayerMethod::EnsouledHead)
            }
            Self::EnsouledHellhoundHead => (
                "Ensouled Hellhound Head",
                1,
                1200.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledAviansieHead => (
                "Ensouled Aviansie Head",
                1,
                1234.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledAbyssalHead => (
                "Ensouled Abyssal Head",
                1,
                1300.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::EnsouledDragonHead => (
                "Ensouled Dragon Head",
                1,
                1560.0,
                PrayerMethod::EnsouledHead,
            ),
            Self::BlessedBoneShards => (
                "Blessed Bone Shards",
                30,
                5.0,
                PrayerMethod::BlessedSunfireWine,
            ),
            Self::SuperiorDragonBones => ("Superior Dragon Bones", 70, 150.0, PrayerMethod::Bones),
        };

        Details::Prayer(PrayerDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: match details.3 {
                PrayerMethod::BlessedSunfireWine => vec![
                    Multipliers::Prayer(PrayerMultipliers::BlessedSunfireWine),
                    Multipliers::Prayer(PrayerMultipliers::ZealotRobes),
                    Multipliers::Prayer(PrayerMultipliers::BothBlessedSunfireWineAndZealotRobes),
                ],
                PrayerMethod::Bones => vec![
                    Multipliers::Prayer(
                        PrayerMultipliers::EitherSinisterOfferingOrSacredBoneBurner,
                    ),
                    Multipliers::Prayer(PrayerMultipliers::LitGildedAltar),
                    Multipliers::Prayer(PrayerMultipliers::BothLitGildedAltarAndZealotRobes),
                    Multipliers::Prayer(PrayerMultipliers::Ectofuntus),
                    Multipliers::Prayer(PrayerMultipliers::BothEctofuntusAndZealotRobes),
                    Multipliers::Prayer(PrayerMultipliers::ChaosAltar),
                    Multipliers::Prayer(PrayerMultipliers::ZealotRobes),
                ],
                PrayerMethod::Ashes => {
                    vec![Multipliers::Prayer(
                        PrayerMultipliers::EitherDemonicOfferingOrSacredBoneBurner,
                    )]
                }
                PrayerMethod::EnsouledHead => {
                    vec![Multipliers::Prayer(PrayerMultipliers::ZealotRobes)]
                }
                PrayerMethod::Fish => {
                    vec![Multipliers::Prayer(PrayerMultipliers::ZealotRobes)]
                }
                PrayerMethod::Shade => vec![Multipliers::Prayer(
                    PrayerMultipliers::MorytaniaDiary3Shades,
                )],
            },
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
                return if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 3
                {
                    index = index.add(1);

                    true
                } else {
                    false
                };
            });
        } else {
            return vec![];
        }

        all
    }
}

pub enum PrayerMethod {
    BlessedSunfireWine,
    Bones,
    Ashes,
    EnsouledHead,
    Fish,
    Shade,
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct PrayerDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for PrayerDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp as f64).ceil()).as_str(),
                "d"
            )
            .as_str())
        )];

        self.multipliers.iter().for_each(|x| {
            let a = match x {
                Multipliers::Prayer(x) => x,
                _ => return,
            };
            let d = a.details();
            vec.push(s.p(format!(
                "{} {}",
                s.c1(format!("{}:", d.name.as_str()).as_str()),
                s.c2(common::commas_from_string(
                    format!("{}", (xp_difference / (self.xp as f64 * d.value)).ceil()).as_str(),
                    "d"
                )
                .as_str())
            )
            .as_str()));
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
#[allow(dead_code)]
pub enum PrayerMultipliers {
    SacredBoneBurner,
    SinisterOffering,
    EitherSinisterOfferingOrSacredBoneBurner,
    LitGildedAltar,
    BothLitGildedAltarAndZealotRobes,
    Ectofuntus,
    BothEctofuntusAndZealotRobes,
    ChaosAltar,
    BlessedSunfireWine,
    DemonicOffering,
    EitherDemonicOfferingOrSacredBoneBurner,
    MorytaniaDiary3Shades,
    ZealotRobes,
    BothBlessedSunfireWineAndZealotRobes,
}

impl PrayerMultipliers {
    pub fn details(&self) -> PrayerMultiplierDetails {
        let details = match self {
            Self::SacredBoneBurner => ("Sacred Bone Burner", 3.0),
            Self::SinisterOffering => ("Sinister Offering", 3.0),
            Self::EitherSinisterOfferingOrSacredBoneBurner => ("Sin. Offer./Bone Burner", 3.0),
            Self::LitGildedAltar => ("Lit Gilded Altar", 3.5),
            Self::BothLitGildedAltarAndZealotRobes => ("w/Outfit", 3.5 * 1.05),
            Self::Ectofuntus => ("Ectofuntus", 4.0),
            Self::BothEctofuntusAndZealotRobes => ("w/Outfit", 4.0 * 1.05),
            Self::ChaosAltar => ("Chaos Altar", 7.0),
            Self::BlessedSunfireWine => ("Blessed Sunfire Wine", 1.2),
            Self::DemonicOffering => ("Demonic Offering", 3.0),
            Self::EitherDemonicOfferingOrSacredBoneBurner => ("Dem. Offer./Bone Burner", 3.0),
            Self::MorytaniaDiary3Shades => ("Mory Diary3", 1.5),
            Self::ZealotRobes => ("Outfit", 1.05),
            Self::BothBlessedSunfireWineAndZealotRobes => ("Both", 1.05 * 1.2),
        };

        PrayerMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct PrayerMultiplierDetails {
    pub name: String,
    pub value: f64,
}
