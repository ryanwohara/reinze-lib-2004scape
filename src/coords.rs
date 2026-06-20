use anyhow::Result;
use common::source::Source;
use regex::Regex;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    if s.query.trim().is_empty() {
        return Ok(vec![format!(
            "{} {}",
            s.l("Coords"),
            s.c1("You must specify coordinates.")
        )]);
    }

    let display = normalize_query(&s.query);
    let key = display.replace(' ', "_");

    let output = match find_location(&key) {
        Some(location) => format!("{} {} {}", s.l("Coords"), s.c1(&display), s.c2(location)),
        None => format!(
            "{} {} {}",
            s.l("Coords"),
            s.c1("There were no matches for the coordinates"),
            s.c2(&display)
        ),
    };

    Ok(vec![output])
}

/// Normalises the two coordinate tokens of a query into the canonical
/// "DD.MM Direction DD.MM Direction" display string.
fn normalize_query(query: &str) -> String {
    // Glue any direction word to the number that precedes it (drop the space),
    // then keep only the first two whitespace-separated tokens.
    let glue = Regex::new(r"(?i) (n(?:orth)?|s(?:outh)?|w(?:est)?|e(?:ast)?)").unwrap();
    let glued = glue.replace_all(query, "${1}");
    let tokens: Vec<&str> = glued.split_whitespace().collect();

    let first = tokens.first().copied().unwrap_or("");
    let second = tokens.get(1).copied().unwrap_or("");

    format!("{} {}", normalize_coord(first), normalize_coord(second))
}

/// Normalises a single coordinate token (e.g. "0400south") into "DD.MM Direction".
fn normalize_coord(token: &str) -> String {
    // Count digits with spaces and dots removed, then left-pad the original
    // token with zeros until it carries four digits.
    let stripped: String = token.chars().filter(|&c| c != ' ' && c != '.').collect();
    let digit_count = stripped.chars().filter(|c| c.is_ascii_digit()).count();
    let padded = if digit_count < 4 {
        format!("{}{}", "0".repeat(4 - digit_count), token)
    } else {
        token.to_string()
    };

    // Pad a leading single-digit degree (e.g. "1.2n" forms) up to two digits each.
    let widened = Regex::new(r"^(\d.)(\d\D)")
        .unwrap()
        .replace(&padded, "0${1}0${2}");

    // Insert the decimal point after the first two digits: DDMM -> DD.MM.
    let dotted = Regex::new(r"^(\d\d)(\d\d)")
        .unwrap()
        .replace(&widened, "${1}.${2}");

    // Collapse a direction word down to its uppercased initial letter.
    let lettered = Regex::new(r"(?i)([wesn])(est|ast|orth|outh)?")
        .unwrap()
        .replace_all(&dotted, |caps: &regex::Captures| caps[1].to_uppercase());

    // Separate the minutes from the direction letter and spell the direction out.
    let spaced = Regex::new(r"(?i)(\d)([wesn])")
        .unwrap()
        .replace_all(&lettered, "${1} ${2}");

    spaced
        .replace('N', "North")
        .replace('S', "South")
        .replace('E', "East")
        .replace('W', "West")
}

/// Looks up the location description for a canonical underscore-joined key.
fn find_location(key: &str) -> Option<&'static str> {
    Coord::ALL.iter().find_map(|coord| {
        let (k, location) = coord.details();
        (k == key).then_some(location)
    })
}

/// A treasure-trail coordinate dig-site, ported from the mIRC `[Coords]` table.
/// Each variant is named `<lat-dir><lat><lon-dir><lon>` (e.g. `N0000W0713`).
enum Coord {
    N0000W0713,
    S0005E0113,
    S0013E1358,
    S0018E0928,
    S0020E2315,
    N0030E2416,
    S0031E1743,
    N0052E0058,
    N0054E0007,
    S0118E1415,
    N0124W0805,
    N0126E0801,
    N0133E0415,
    S0135E0728,
    N0246E2911,
    N0248E2230,
    N0250E0620,
    S0335E1335,
    S0345E2245,
    S0400E1246,
    S0403E0311,
    S0405E0424,
    N0413E1213,
    N0413E1245,
    S0416E1616,
    N0418E1245,
    N0441W0309,
    S0520E0428,
    N0537E3115,
    N0543E2305,
    S0550E1005,
    S0600E2148,
    S0611E1507,
    N0631W0146,
    N0705E3056,
    N0733E1500,
    S0743E1226,
    N0803E3116,
    S0805E1556,
    S0826E1028,
    N0833W0139,
    N0933E0215,
    N0948E1739,
    N1022E0858,
    N1103E3120,
    N1105W0045,
    N1107E1224,
    N1141E1458,
    N1248E2020,
    N1346E2101,
    N1454E0913,
    N1603E1407,
    N1631E1254,
    N1635E2701,
    N1643E1913,
    N1750E0830,
    N1803E2516,
    N1822E1633,
    N1943E2507,
    N2005E2152,
    N2007E1833,
    N2033E1548,
    N2124E1754,
    N2230E0301,
    N2235E1918,
    N2245E2633,
    N2424E2624,
    N2456E2228,
    N2458E1843,
    N2503E1705,
    N2503E2324,
}

impl Coord {
    const ALL: &'static [Coord] = &[
        Coord::N0000W0713,
        Coord::S0005E0113,
        Coord::S0013E1358,
        Coord::S0018E0928,
        Coord::S0020E2315,
        Coord::N0030E2416,
        Coord::S0031E1743,
        Coord::N0052E0058,
        Coord::N0054E0007,
        Coord::S0118E1415,
        Coord::N0124W0805,
        Coord::N0126E0801,
        Coord::N0133E0415,
        Coord::S0135E0728,
        Coord::N0246E2911,
        Coord::N0248E2230,
        Coord::N0250E0620,
        Coord::S0335E1335,
        Coord::S0345E2245,
        Coord::S0400E1246,
        Coord::S0403E0311,
        Coord::S0405E0424,
        Coord::N0413E1213,
        Coord::N0413E1245,
        Coord::S0416E1616,
        Coord::N0418E1245,
        Coord::N0441W0309,
        Coord::S0520E0428,
        Coord::N0537E3115,
        Coord::N0543E2305,
        Coord::S0550E1005,
        Coord::S0600E2148,
        Coord::S0611E1507,
        Coord::N0631W0146,
        Coord::N0705E3056,
        Coord::N0733E1500,
        Coord::S0743E1226,
        Coord::N0803E3116,
        Coord::S0805E1556,
        Coord::S0826E1028,
        Coord::N0833W0139,
        Coord::N0933E0215,
        Coord::N0948E1739,
        Coord::N1022E0858,
        Coord::N1103E3120,
        Coord::N1105W0045,
        Coord::N1107E1224,
        Coord::N1141E1458,
        Coord::N1248E2020,
        Coord::N1346E2101,
        Coord::N1454E0913,
        Coord::N1603E1407,
        Coord::N1631E1254,
        Coord::N1635E2701,
        Coord::N1643E1913,
        Coord::N1750E0830,
        Coord::N1803E2516,
        Coord::N1822E1633,
        Coord::N1943E2507,
        Coord::N2005E2152,
        Coord::N2007E1833,
        Coord::N2033E1548,
        Coord::N2124E1754,
        Coord::N2230E0301,
        Coord::N2235E1918,
        Coord::N2245E2633,
        Coord::N2424E2624,
        Coord::N2456E2228,
        Coord::N2458E1843,
        Coord::N2503E1705,
        Coord::N2503E2324,
    ];

    /// Returns the canonical underscore-joined key and the location description.
    fn details(&self) -> (&'static str, &'static str) {
        match self {
            Coord::N0000W0713 => (
                "00.00_North_07.13_West",
                "Southern part of Isafdar, northeast of Tyras Camp. You must have started the Regicide Quest and have level 56 agility to access this area. (The charter ship may be convenient.) Saradomin wizard will attack.",
            ),
            Coord::S0005E0113 => (
                "00.05_South_01.13_East",
                "To the West of the Gnome Maze on the river's edge. Directly across the river from the Observatory (not far from Fairy Ring CIQ)",
            ),
            Coord::S0013E1358 => (
                "00.13_South_13.58_East",
                "Between the two lakes on Karamja, west of the banana plantation and general store",
            ),
            Coord::S0018E0928 => (
                "00.18_South_09.28_East",
                "Southwest of Brimhaven in gold mine (near dungeon entrance)",
            ),
            Coord::S0020E2315 => (
                "00.20_South_23.15_East",
                "Lumbridge Swamp, directly to the Southeast of the Water Altar, about 12 steps to the North of the Southern shore",
            ),
            Coord::N0030E2416 => (
                "00.30_North_24.16_East",
                "In Lumbridge Swamp go to the shack for the Lost City (Zanaris), and head Northeast until you see an open area with rocks on the ground. It's easy to spot once you see the area.",
            ),
            Coord::S0031E1743 => (
                "00.31_South_17.43_East",
                "Near entrance to the Ice caves, South of Falador",
            ),
            Coord::N0052E0058 => (
                "00.52_North_00.58_East",
                "Small goblin village Northeast of the Observatory. Dig on the inside of the Eastern fence, near the North end.",
            ),
            Coord::N0054E0007 => (
                "00.54_North_00.07_East",
                "Inside the Observatory office building, Northeast corner of the main room",
            ),
            Coord::S0118E1415 => (
                "01.18_South_14.15_East",
                "Northwest of the Shipyard on Karamja, most Northwest tip of the island (near Fairy Ring DKP)",
            ),
            Coord::N0124W0805 => (
                "01.24_North_08.05_West",
                "From the Elven camp in Tirannwn, go South over the log bridge, then West into the area where the rabbits/bears are. Go a little Southwest. There's a stick trap there; dig just past the stick trap. You must have started the Regicide Quest and have level 56 agility to access this area. Saradomin wizard will attack.",
            ),
            Coord::N0126E0801 => (
                "01.26_North_08.01_East",
                "Moss Giant Island, West of Brimhaven. You need level 10 agility to use the rope swing to get to the island.",
            ),
            Coord::N0133E0415 => (
                "01.33_North_04.15_East",
                "North of Fight Arena, south of Clock Tower",
            ),
            Coord::S0135E0728 => (
                "01.35_South_07.28_East",
                "Green Spider Island, East of Yanille (convenient to Fairy Ring CLS)",
            ),
            Coord::N0246E2911 => (
                "02.46_North_29.11_East",
                "Duel Arena. Need a friend to duel, select obstacles; no Saradomin wizard attack here.",
            ),
            Coord::N0248E2230 => (
                "02.48_North_22.30_East",
                "Go east following the dirt road that starts next to the Draynor Jail until you come to H.A.M. Headquarters (the abandoned house with all the small spiders.) Dig next to the plant outside the Northwest wall of the house.",
            ),
            Coord::N0250E0620 => (
                "02.50_North_06.20_East",
                "A bit south of Ardougne, on the East coast, north of the Tower of Life and Necromancer Tower. There's a small outcrop toward the water with a willow tree on it. Dig on the Northeast side of the tree.",
            ),
            Coord::S0335E1335 => (
                "03.35_South_13.35_East",
                "Northeast of the Nature Altar and the Karamja Jungle mine",
            ),
            Coord::S0345E2245 => (
                "03.45_South_22.45_East",
                "North edge of the pond in the Kharidian Desert Bedabin camp. You must have at least started the Tourist Trap Quest to get to this area. (Magic Carpet is very helpful.) Saradomin wizard will attack.",
            ),
            Coord::S0400E1246 => (
                "04.00_South_12.46_East",
                "Karamja Jungle mine, to the Northwest of the Nature Altar, in the centre of the mining area.",
            ),
            Coord::S0403E0311 => (
                "04.03_South_03.11_East",
                "Ogre City, in the centre on the Eastern side. You must have started the Watchtower Quest to access this area. Go up the ramp to the gate, all the way south along the western wall, and jump two obstacles to reach this dead end area.",
            ),
            Coord::S0405E0424 => (
                "04.05_South_04.24_East",
                "The coordinates are located on the small island East of Ogre City, between the spear wall and the dead tree. However, to get to that island, you have to walk around the outside of Gu'Tanoth (Ogre City) until you get to the bottom. Start on the Castle Wars side (West side of Gu'Tanoth) and go South until you reach the bottom corner. Then start walking East. Not too far from the Southwestern corner, you'll see a dungeon entrance just beyond a some ruins near the slope. Go inside the cave, and you'll instantly appear on the island. No quests are required to get onto the island.",
            ),
            Coord::N0413E1213 => (
                "04.13_North_12.13_East",
                "Crandor Island, a little West of the North mining area. You must have at least started the Dragonslayer Quest to get onto Crandor Island.",
            ),
            Coord::N0413E1245 => (
                "04.13_North_12.45_East",
                "Crandor, near the broken walls and pillars and scorpions toward the northeast. You must have at least started the Dragonslayer Quest to get onto Crandor Island.",
            ),
            Coord::S0416E1616 => (
                "04.16_South_16.16_East",
                "Karamja Shipyard, in the Southernmost house. Dig at the east window between the two bookshelves. The password to the Shipyard is ka-lu-min, but you must have started the Grand Tree Quest to access this area. Saradomin wizard will attack.",
            ),
            Coord::N0418E1245 => (
                "04.18_North_12.45_East",
                "Crandor, near Hobgoblins, past the lesser demons. You must have at least started the Dragonslayer Quest to get onto Crandor Island.",
            ),
            Coord::N0441W0309 => (
                "04.41_North_03.09_West",
                "Arandar, South of the winding path to Prifddinas, amidst the red spider egg spawns. You must have at least started the Regicide quest.",
            ),
            Coord::S0520E0428 => (
                "05.20_South_04.28_East",
                "In Feldip Hills, in the swampy area just to the South of the two rockslides (north of Rantz)",
            ),
            Coord::N0537E3115 => (
                "05.37_North_31.15_East",
                "Mort Myre Swamp, on the Nature Grotto island, just North of the tree. You must have started the Nature Spirit Quest to get into Mort Myre Swamp.",
            ),
            Coord::N0543E2305 => (
                "05.43_North_23.05_East",
                "North of the windmill between Draynor and Lumbridge, inside the fenced area with the cows. A little East of small island in the river.",
            ),
            Coord::S0550E1005 => (
                "05.50_South_10.05_East",
                "Cairn Isle, inside the ruined building. Saradomin wizard will attack.",
            ),
            Coord::S0600E2148 => (
                "06.00_South_21.48_East",
                "In the Kharidian desert to the West of the Bandit camp, on the tip of the peninsula. Dig 3 steps to the West of the cactus. You must have at least started the Tourist Trap Quest to get to this area.",
            ),
            Coord::S0611E1507 => (
                "06.11_South_15.07_East",
                "Karamja Jungle, southwest of glider area towards the river where the jogres are",
            ),
            Coord::N0631W0146 => (
                "06.31_North_01.46_West",
                "Southwest outside the Gnome Stronghold, at the 3 ponds (north of the Ancient Gate to Arandar). Dig between the two most Eastern ponds.",
            ),
            Coord::N0705E3056 => (
                "07.05_North_30.56_East",
                "Mort Myre Swamp, directly across the river from the digsite, well south and west of the swamp gate entrance, one pond South of the fishing spots pond. Find the dead tree and dig next to it. You must have started the Nature Spirit Quest to get into Mort Myre Swamp.",
            ),
            Coord::N0733E1500 => (
                "07.33_North_15.00_East",
                "Peninsula South of Taverly tree patch, near the Lady of the Lake from Merlin's Crystal Quest, amidst the willow trees",
            ),
            Coord::S0743E1226 => (
                "07.43_South_12.26_East",
                "South edge of Kharazi Jungle pond. You must have started the Legends Quest to get into the Kharazi Jungle, and carry a machete and woodcutting axe. Saradomin wizard will attack.",
            ),
            Coord::N0803E3116 => (
                "08.03_North_31.16_East",
                "Mort Myre Swamp, across the river from the digsite, straight south of the swamp gate entrance, northeast of the fishing spots pond. You must have started the Nature Spirit Quest to get into Mort Myre Swamp. Saradomin wizard will attack.",
            ),
            Coord::S0805E1556 => (
                "08.05_South_15.56_East",
                "Kharazi Jungle, towards the east coast, just north of the last totem pole. You must have started the Legends Quest and bring a machete and axe to get into the Kharazi Jungle. Saradomin wizard will attack.",
            ),
            Coord::S0826E1028 => (
                "08.26_South_10.28_East",
                "Kharazi Jungle, on the small peninsula in the most Southwestern corner. Dig 2 steps North of the corpse. You must have started the Legends Quest and bring a machete and axe to get into the Kharazi Jungle. Saradomin wizard will attack.",
            ),
            Coord::N0833W0139 => (
                "08.33_North_01.39_West",
                "Inside the Tree Gnome Stronghold, on the other side of the river to the West. Cross the bridge into the terrorbird enclosure, and go out the Northeastern gate. Dig a little to the Southeast of the gate towards the water.",
            ),
            Coord::N0933E0215 => (
                "09.33_North_02.15_East",
                "Baxtorian Falls, on the second island in the river, bring a rope. Use a raft to get to the first island, then use a rope with the rock on the second island to get to the second island. Dig next to the dead tree. You must have at least started the Waterfall Quest to get to the second island.",
            ),
            Coord::N0948E1739 => (
                "09.48_North_17.39_East",
                "Ice Mountain to the North of the Dwarven Mine entrance, centre of the Southern side of the mountain amongst the icefiends",
            ),
            Coord::N1022E0858 => (
                "10.22_North_08.58_East",
                "Outside, next to the Eastern wall of the anvil house in Seers' Village. If you go inside the house and find the stool that's next to the East wall, the spot is just on the other side of the wall.",
            ),
            Coord::N1103E3120 => (
                "11.03_North_31.20_East",
                "Morytania, south of the Slayer tower, a little to the East of the gate in the fence, near a double root sticking out of the ground. You must have completed the Priest in Peril Quest to get into Morytania.",
            ),
            Coord::N1105W0045 => (
                "11.05_North_00.45_West",
                "Tree Gnome Stronghold, in the toad and worm swamp northwest of the Grand Tree",
            ),
            Coord::N1107E1224 => (
                "11.07_North_12.24_East",
                "White Wolf mountain, Northwest of the glider in the valley between the two dungeon markers, next to the rocks (pickaxe not required for clue)",
            ),
            Coord::N1141E1458 => (
                "11.41_North_14.58_East",
                "Burthorpe, between the picnic tables in the pub's fenced outdoor area",
            ),
            Coord::N1248E2020 => (
                "12.48_North_20.20_East",
                "Northeast of the Air Obelisk (reached by travelling through wilderness section of Edgeville dungeon.) It's 3 paces east of the ladder. Beware of revenants, this is level 7 wilderness. Zamorak wizard will attack.",
            ),
            Coord::N1346E2101 => (
                "13.46_North_21.01_East",
                "Level 11 Wilderness, North of Edgeville. Near the bears and the bronze arrow respawn, dig in the middle of the 4 mushrooms. Zamorak mage will attack.",
            ),
            Coord::N1454E0913 => (
                "14.54_North_09.13_East",
                "Southwest side of the swaying lyre tree east of Rellekka, northwest of golden apple tree, southwest of mountain camp",
            ),
            Coord::N1603E1407 => (
                "16.03_North_14.07_East",
                "Just above the entrance to Mad Eadgar's cave at the peak of Trollheim. You must have started the Troll Stronghold Quest to reach this area. Saradomin wizard will attack.",
            ),
            Coord::N1631E1254 => (
                "16.31_North_12.54_East",
                "Troll marketplace west of Trollheim, at the bottom of the ramp leading into the Stronghold itself. Dig next to the sleeping troll. You must have started the Troll Stronghold Quest to reach this area. Saradomin wizard will attack.",
            ),
            Coord::N1635E2701 => (
                "16.35_North_27.01_East",
                "Level 22 Wilderness, north of the black salamander Hunter area, East of the Clan Wars starting area and south of Clan Wars arena itself. Dig near the centre of the 3 volcanoes. Zamorak mage will attack.",
            ),
            Coord::N1643E1913 => (
                "16.43_North_19.13_East",
                "Inside Bandit Camp in the Wilderness, north end of the east wall at pond. Zamorak mage will attack.",
            ),
            Coord::N1750E0830 => (
                "17.50_North_08.30_East",
                "Near dock outside Rellekka to the northeast, west of the quest start point, on a seaweed spawn at the water's edge. Saradomin mage will attack.",
            ),
            Coord::N1803E2516 => (
                "18.03_North_25.16_East",
                "Level 28 Wilderness Ruins located between Bounty Hunter and Clan Wars areas. Clue is north of the eastern building. Zamorak Mage will attack.",
            ),
            Coord::N1822E1633 => (
                "18.22_North_16.33_East",
                "Level 30 Wilderness, Southwest section of the Forgotten Cemetery, where the level 2 men are. Zamorak mage will attack.",
            ),
            Coord::N1943E2507 => (
                "19.43_North_25.07_East",
                "Level 34-35 Wilderness, between Clan Wars and Red Dragon Isle, north of \"Eastern\" Ruins and Graveyard of Shadows. There are Chaos Dwarves and rocks. Dig in the middle of the 4 large, white rocks. Zamorak mage will attack.",
            ),
            Coord::N2005E2152 => (
                "20.05_North_21.52_East",
                "(North of the Moss Giants) in the Wilderness area north of Bounty Hunter, southwest of Red Dragon Isle, southeast of Lava Maze. The exact spot is a little to the North of the small pond. Zamorak mage will attack.",
            ),
            Coord::N2007E1833 => (
                "20.07_North_18.33_East",
                "Level 36 Wilderness, Hobgoblin Mine. Three steps west of the large cluster of iron rocks forming the east edge. Zamorak Mage will attack.",
            ),
            Coord::N2033E1548 => (
                "20.33_North_15.48_East",
                "Altar South of Ice Giants in the Wilderness, southwest of Lava Maze, outside of the building on the far West side. Zamorak Mage will attack.",
            ),
            Coord::N2124E1754 => (
                "21.24_North_17.54_East",
                "Inside the Lesser Demon pen on the West side of the Lava Maze in the Wilderness, about 2 steps Southwest of ladder. Zamorak Mage will attack.",
            ),
            Coord::N2230E0301 => (
                "22.30_North_03.01_East",
                "Miscellania, on the path to the Northern mining area. Dig right before the end of the path. You must have completed the Fremennik Trials Quest to take the boat to Miscellania.",
            ),
            Coord::N2235E1918 => (
                "22.35_North_19.18_East",
                "Rune rocks in the Wilderness, on the North side of the Lava Maze. Dig on the West side of the Northernmost rock. Zamorak Mage will attack.",
            ),
            Coord::N2245E2633 => (
                "22.45_North_26.33_East",
                "Demonic ruins in the Wilderness, north of Clan Wars. Dig on the most Northern pentagram inside the building. Zamorak Mage will attack.",
            ),
            Coord::N2424E2624 => (
                "24.24_North_26.24_East",
                "Rogues' Castle in Members Wilderness, directly to the North of the North tower. Be on the lookout for the level 305 Chaos Elemental, who likes to hide on the West side of the Castle. Get too close, and he will attack you! Zamorak Mage will attack when you dig.",
            ),
            Coord::N2456E2228 => (
                "24.56_North_22.28_East",
                "North of the entrance to the Ardougne Teleport House area (Deserted Keep) in the Wilderness. Dig a little to the Northeast of the cobweb. Remember to bring a knife to cut the cobwebs at the entrance. Zamorak Mage will attack.",
            ),
            Coord::N2458E1843 => (
                "24.58_North_18.43_East",
                "Northwest corner outside the Pirate's Hall in the Wilderness, facing the water. Zamorak Mage will attack.",
            ),
            Coord::N2503E1705 => (
                "25.03_North_17.05_East",
                "Northwest outside the Wilderness agility course, towards the northern edge, close to the fence and the water",
            ),
            Coord::N2503E2324 => (
                "25.03_North_23.24_East",
                "Axe House in the Wilderness, one step West outside the North door",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_four_digit_token() {
        assert_eq!(normalize_coord("0400south"), "04.00 South");
        assert_eq!(normalize_coord("0311east"), "03.11 East");
        assert_eq!(normalize_coord("0713west"), "07.13 West");
        assert_eq!(normalize_coord("2503north"), "25.03 North");
    }

    #[test]
    fn pads_short_tokens_with_leading_zeros() {
        assert_eq!(normalize_coord("4s"), "00.04 South");
        assert_eq!(normalize_coord("311e"), "03.11 East");
    }

    #[test]
    fn empty_token_becomes_zero() {
        assert_eq!(normalize_coord(""), "00.00");
    }

    #[test]
    fn normalize_query_glues_spaced_directions() {
        assert_eq!(
            normalize_query("0400 south 0311 east"),
            "04.00 South 03.11 East"
        );
    }

    #[test]
    fn normalize_query_handles_glued_directions() {
        assert_eq!(
            normalize_query("0400south 0311east"),
            "04.00 South 03.11 East"
        );
    }

    #[test]
    fn find_location_matches_known_key() {
        let location = find_location("00.00_North_07.13_West").unwrap();
        assert!(location.contains("Isafdar"));
    }

    #[test]
    fn find_location_returns_none_for_unknown_key() {
        assert!(find_location("99.99_North_99.99_East").is_none());
    }

    #[test]
    fn lookup_finds_a_coordinate() {
        let s = src("0403 south 0311 east");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("04.03 South 03.11 East"));
        assert!(output[0].contains("Ogre City"));
    }

    #[test]
    fn lookup_reports_no_match() {
        let s = src("9999 north 9999 east");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("There were no matches for the coordinates"));
        assert!(output[0].contains("99.99 North 99.99 East"));
    }

    #[test]
    fn lookup_requires_coordinates_when_empty() {
        let s = src("");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("You must specify coordinates."));
    }

    #[test]
    fn every_coord_key_is_canonical_and_unique() {
        let re = Regex::new(r"^\d\d\.\d\d_(North|South)_\d\d\.\d\d_(East|West)$").unwrap();
        let mut seen = std::collections::HashSet::new();
        for coord in Coord::ALL {
            let (key, location) = coord.details();
            assert!(re.is_match(key), "non-canonical key: {key}");
            assert!(!location.is_empty(), "empty location for {key}");
            assert!(seen.insert(key), "duplicate key: {key}");
        }
        assert_eq!(Coord::ALL.len(), 71);
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
            "coords",
            query,
        )
    }
}
