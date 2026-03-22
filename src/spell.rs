use anyhow::Result;
use common::source::Source;

use crate::stats::magic::Magic;
use crate::stats::skill::{Details, Skill};

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    if s.query.is_empty() {
        return Ok(vec!["Syntax: -spell <name>".to_string()]);
    }

    let results = Magic::search(&s.query);

    if results.is_empty() {
        return Ok(vec![format!(
            "{} {}",
            s.l("Spell"),
            s.c1("No spells found")
        )]);
    }

    let mut output = vec![];

    for spell in results.iter() {
        if let Details::Magic(details) = Skill::details(spell) {
            output.push(format!(
                "{} {} {} {} {} {} {}",
                s.l(&details.name),
                s.c1("Level:"),
                s.c2(&details.level.to_string()),
                s.c1("Runes:"),
                s.c2(&details.runes),
                s.c1("Effect:"),
                s.c2(&details.effect),
            ));
        }
    }

    Ok(output)
}
