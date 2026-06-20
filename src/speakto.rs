use anyhow::Result;
use common::source::Source;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    if s.query.trim().is_empty() {
        return Ok(vec![format!(
            "{} {}",
            s.l("Speakto"),
            s.c1("You must specify a person.")
        )]);
    }

    // Key = the query with spaces turned into underscores, matched case-insensitively.
    let key = s.query.replace(' ', "_");

    let locations = find_locations(&key);

    let output = if locations.is_empty() {
        format!(
            "{} {}",
            s.l("Speakto"),
            s.c1("No matches found. Please check your spelling and try again.")
        )
    } else {
        // An NPC name can appear more than once (e.g. Bartender); show every location.
        let joined = locations
            .iter()
            .map(|location| s.c2(location))
            .collect::<Vec<String>>()
            .join(&s.c1(" | "));
        format!(
            "{} {} {} {} {}",
            s.l("Speakto"),
            s.c1("NPC:"),
            s.c2(caps(&s.query)),
            s.c1("Location:"),
            joined
        )
    };

    Ok(vec![output])
}

/// Capitalises the first letter of each whitespace-delimited word, mirroring
/// mIRC's `$caps`. The rest of each word is left unchanged.
fn caps(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut at_word_start = true;
    for c in text.chars() {
        if c.is_whitespace() {
            at_word_start = true;
            result.push(c);
        } else {
            if at_word_start {
                result.extend(c.to_uppercase());
            } else {
                result.push(c);
            }
            at_word_start = false;
        }
    }
    result
}

/// Resolves an NPC key (spaces as underscores) to every matching location,
/// case- and separator-insensitively, in table order.
fn find_locations(key: &str) -> Vec<&'static str> {
    Speakto::ALL
        .iter()
        .filter_map(|npc| {
            let (npc_key, location) = npc.details();
            npc_key
                .replace(' ', "_")
                .eq_ignore_ascii_case(key)
                .then_some(location)
        })
        .collect()
}

/// A "speak to" NPC dig-site hint, ported from the mIRC `[Speakto]` table.
/// Each variant is named after the NPC.
enum Speakto {
    Arhein,
    Bartender,
    Bartender2,
    BlackHeather,
    Donovan,
    Doric,
    Ellis,
    Gaius,
    GeneralBentnoze,
    Hajedy,
    Hans,
    Hazelmere,
    KangaiMau,
    KebabSeller,
    KingBolren,
    Lowe,
    Luthas,
    Murphy,
    Monk,
    Ned,
    Oracle,
    PartyPete,
    Referee,
    Roavar,
    SirKay,
    Squire,
    Tanner,
    Ulizius,
    Zeke,
    ZooKeeper,
}

impl Speakto {
    const ALL: &'static [Speakto] = &[
        Speakto::Arhein,
        Speakto::Bartender,
        Speakto::Bartender2,
        Speakto::BlackHeather,
        Speakto::Donovan,
        Speakto::Doric,
        Speakto::Ellis,
        Speakto::Gaius,
        Speakto::GeneralBentnoze,
        Speakto::Hajedy,
        Speakto::Hans,
        Speakto::Hazelmere,
        Speakto::KangaiMau,
        Speakto::KebabSeller,
        Speakto::KingBolren,
        Speakto::Lowe,
        Speakto::Luthas,
        Speakto::Murphy,
        Speakto::Monk,
        Speakto::Ned,
        Speakto::Oracle,
        Speakto::PartyPete,
        Speakto::Referee,
        Speakto::Roavar,
        Speakto::SirKay,
        Speakto::Squire,
        Speakto::Tanner,
        Speakto::Ulizius,
        Speakto::Zeke,
        Speakto::ZooKeeper,
    ];

    /// Returns the NPC key (as stored) and the location description.
    fn details(&self) -> (&'static str, &'static str) {
        match self {
            Speakto::Arhein => ("Arhein", "Catherby - The Dock in Catherby outside the Bank"),
            Speakto::Bartender => (
                "Bartender",
                "Varrock - In the Blue Moon Inn, across from the Sword Shop",
            ),
            Speakto::Bartender2 => ("Bartender", "Port Sarim - In the Rusty Anchor Bar"),
            Speakto::BlackHeather => (
                "Black Heather",
                "Bandit Camp (wilderness) - In level 23 Wilderness",
            ),
            Speakto::Donovan => (
                "Donovan",
                "Sinclair Mansion - North-west of Camelot Castle, on the second floor",
            ),
            Speakto::Doric => (
                "Doric",
                "In the house with anvils - East of the gate to Taverley",
            ),
            Speakto::Ellis => ("Ellis", "Al-Kharid - Tanner's Shop"),
            Speakto::Gaius => ("Gaius", "Taverley - 2-Handed Sword Shop"),
            Speakto::GeneralBentnoze => ("General_Bentnoze", "Goblin Village - North of Falador"),
            Speakto::Hajedy => (
                "Hajedy",
                "Brimhaven and N. Karamja - Near the Brimhaven port",
            ),
            Speakto::Hans => ("Hans", "Lumbridge - Castle"),
            Speakto::Hazelmere => (
                "Hazelmere",
                "Jungle Spiders Island - East of Yanille, Second floor of the Hut",
            ),
            Speakto::KangaiMau => (
                "Kangai_Mau",
                "Brimhaven and N. Karamja - Shrimp and Parrot Bar/Restaurant in Brimhaven.",
            ),
            Speakto::KebabSeller => (
                "Kebab_Seller",
                "Al-Kharid - Kebab Store, south of the furnace",
            ),
            Speakto::KingBolren => ("King_Bolren", "Tree Gnome Maze - By the Spirit Tree"),
            Speakto::Lowe => ("Lowe", "Varrock - Lowe's Archery Store"),
            Speakto::Luthas => ("Luthas", "NE. Karamja - In the Banana Plantation"),
            Speakto::Murphy => ("Murphy", "Port Khazard - Pier"),
            Speakto::Monk => ("Monk", "Ardougne - Clock Tower"),
            Speakto::Ned => ("Ned", "Draynor Village - Inside a house in Draynor Village"),
            Speakto::Oracle => (
                "Oracle",
                "Ice Mountain - South of the Black Knight's Fortress",
            ),
            Speakto::PartyPete => ("Party Pete", "Falador - Party Hall"),
            Speakto::Referee => ("Referee", "Tree Gnome Stronghold - Gnome-Ball course"),
            Speakto::Roavar => ("Roavar", "Canifis - Bar"),
            Speakto::SirKay => ("Sir_Kay", "Seers' Village - Camelot Castle"),
            Speakto::Squire => ("Squire", "Falador - White Knights Castle"),
            Speakto::Tanner => ("Tanner", "Al-Kharid - North of the furnace"),
            Speakto::Ulizius => ("Ulizius", "Mort Myre - Outside the Gates"),
            Speakto::Zeke => ("Zeke", "Al-Kharid - Scimitar Shop, East of the Tanner"),
            Speakto::ZooKeeper => ("Zoo_Keeper", "Ardougne - Zoo"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_known_npc() {
        let s = src("arhein");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("NPC:"));
        assert!(output[0].contains("Arhein"));
        assert!(output[0].contains("Location:"));
        assert!(output[0].contains("Catherby"));
    }

    #[test]
    fn title_cases_and_resolves_spaced_npc() {
        let s = src("black heather");
        let output = lookup(&s).unwrap();
        assert!(output[0].contains("Black Heather"));
        assert!(output[0].contains("Bandit Camp"));
    }

    #[test]
    fn matches_case_insensitively() {
        let upper = lookup(&src("DONOVAN")).unwrap();
        let lower = lookup(&src("donovan")).unwrap();
        assert!(upper[0].contains("Sinclair Mansion"));
        assert!(lower[0].contains("Sinclair Mansion"));
    }

    #[test]
    fn duplicate_npc_shows_all_locations() {
        let s = src("bartender");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("Blue Moon"));
        assert!(output[0].contains("Rusty Anchor"));
    }

    #[test]
    fn reports_no_match() {
        let s = src("not a real npc");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("No matches found. Please check your spelling and try again."));
    }

    #[test]
    fn requires_a_person_when_empty() {
        let s = src("");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("You must specify a person."));
    }

    #[test]
    fn every_npc_is_well_formed() {
        for npc in Speakto::ALL {
            let (key, location) = npc.details();
            assert!(!key.is_empty(), "empty key");
            assert!(!location.is_empty(), "empty location for {key}");
        }
        assert_eq!(Speakto::ALL.len(), 30);
    }

    // --- test helpers ---

    use common::ColorResult;
    use common::author::Author;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn src(query: &str) -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "speakto",
            query,
        )
    }
}
