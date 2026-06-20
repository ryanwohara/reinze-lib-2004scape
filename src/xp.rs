use crate::common::level_to_xp;
use anyhow::{Result, bail};
use common::commas;
use common::source::Source;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let query = &s.query;

    let split: Vec<&str> = query.split("-").collect();

    if split.len() > 2 || split.len() == 0 {
        return Ok(vec!["Invalid number of arguments".to_string()]);
    } else if split.len() == 2 {
        let start = match split[0].parse::<u32>() {
            Ok(start) => u32::min(start, 126),
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let end = match split[1].parse::<u32>() {
            Ok(end) => u32::min(end, 126),
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{} {}->{} = {} xp",
            s.l("Level->XP"),
            s.c1(&start.to_string()),
            end,
            s.c2(&commas((level_to_xp(end) - level_to_xp(start)) as f64, "d").to_string())
        )];

        return Ok(output);
    } else if split.len() == 1 {
        let level = match split[0].parse::<u32>() {
            Ok(level) => level,
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{} {} = {} xp",
            s.l("Level->XP"),
            s.c1(&level.to_string()),
            s.c2(&commas(level_to_xp(level as u32) as f64, "d"))
        )];

        return Ok(output);
    }

    bail!("Unreachable: xp lookup logic exhausted without returning")
}
