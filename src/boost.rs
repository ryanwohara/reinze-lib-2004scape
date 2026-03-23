use anyhow::Result;
use crate::common::skill;
use common::source::Source;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let query = &s.query;

    let prefix = s.l("Boost");
    let skill = skill(query);
    let mut found_params: Vec<String> = vec![];

    for (action, boost) in get_modifiers(&skill) {
        found_params.push(vec![s.c1(&action), s.c2(&boost)].join(" "));
    }

    let output = format!(
        "{} {}{}",
        prefix,
        format_skill(&skill, s),
        s.not_found(found_params)
    );

    Ok(vec![output])
}

fn format_skill(skill: &str, s: &Source) -> String {
    if skill.len() > 0 {
        format!("{} ", s.p(&skill))
    } else {
        "".to_string()
    }
}

fn get_modifiers(skill: &str) -> Vec<(String, String)> {
    match skill {
        "Agility" => vec![
            ("Summer pie", "+5"),
        ],
        "Attack" => vec![
            ("Kebab", "+1-3"),
            ("Jangerberries", "+2"),
            ("Attack potion", "+3-12"),
            ("Super attack", "+5-19"),
            ("Zamorak brew", "+2-21"),
        ],
        "Cooking" => vec![
            ("Chef's Delight", "+1-5"),
            ("Mature Chef's Delight", "+2-6"),
        ],
        "Crafting" => vec![
            ("Poison chalice", "+0-1"),
        ],
        "Defence" => vec![
            ("Defence potion", "+3-12"),
            ("Super defence", "+5-19"),
            ("Saradomin brew", "+2-21"),
        ],
        "Farming" => vec![
            ("Cider", "+1"),
            ("Mature cider", "+2"),
            ("Garden pie", "+3"),
        ],
        "Firemaking" => vec![],
        "Fishing" => vec![
            ("Fishing potion", "+3"),
            ("Fish pie", "+3"),
            ("Admiral pie", "+5"),
            ("Fishing guild", "+7"),
        ],
        "Fletching" => vec![],
        "Herblore" => vec![
            ("Greenman's ale", "+1"),
        ],
        "Hitpoints" => vec![
            ("Guthix rest", "+5"),
            ("Saradomin brew", "+3-16"),
        ],
        "Magic" => vec![
            ("Wizard's mind bomb", "+2-3"),
            ("Magic potion", "+5"),
        ],
        "Mining" => vec![
            ("Dwarven stout", "+1"),
        ],
        "Prayer" => vec![
            ("Monastery altar", "+2"),
        ],
        "Ranged" => vec![
            ("Ranging potion", "+4-13"),
        ],
        "Runecrafting" => vec![],
        "Smithing" => vec![
            ("Dwarven stout", "+1"),
        ],
        "Strength" => vec![
            ("Jangerberries", "+1"),
            ("Beer", "+0-3"),
            ("Kebab", "+1-3"),
            ("Poison chalice", "+1/+4"),
            ("Strength potion", "+3-12"),
            ("Combat potion", "+3-12"),
            ("Zamorak Brew", "+2-13"),
            ("Super strength", "+5-19"),
        ],
        "Thieving" => vec![
            ("Poison chalice", "+1"),
            ("Bandit's brew", "+1"),
            ("Spring sq'irk juice", "+1"),
            ("Autumn sq'irk juice", "+2"),
            ("Summer sq'irk juice", "+3"),
        ],
        "Woodcutting" => vec![
            ("Axeman's folly", "+1"),
            ("Mature axeman's folly", "+2"),
        ],
        _ => vec![],
    }
    .into_iter()
    .map(|(name, boost)| (name.to_string(), boost.to_string()))
    .collect::<Vec<(String, String)>>()
}
