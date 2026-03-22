use anyhow::Result;
use crate::common::{eval_query, xp_to_level};
use common::commas;
use common::source::Source;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let err = vec![
        vec![
            s.l("XP->Level"),
            s.c1("You may only use experience within the range 0 - 200,000,000"),
        ]
        .join(" "),
    ];

    let xp = match eval_query(&s.query) {
        Ok(xp) => u32::min(xp as u32, 200000000),
        Err(_) => return Ok(err),
    };

    let output = vec![format!(
        "{} {} = {}",
        s.l("XP->Level"),
        s.c1(&commas(xp as f64, "d")),
        s.c2(&format!("Level {}", &xp_to_level(xp).to_string()))
    )];

    Ok(output)
}
