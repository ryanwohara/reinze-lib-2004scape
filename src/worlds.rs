use anyhow::{Context, Result, bail};
use common::source::Source;
use log::error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::time::Duration;

const WORLDS_URL: &str = "https://2004.losthq.rs/pages/api/worlds.php";

#[derive(Deserialize)]
struct World {
    world: String,
    location: String,
    count: u32,
    p2p: bool,
    #[serde(default)]
    hd: String,
}

/// `+worlds` — a one-line summary of every world.
pub fn all(s: &Source) -> Result<Vec<String>> {
    let worlds = match fetch_worlds() {
        Ok(worlds) => worlds,
        Err(e) => return Ok(vec![fetch_error(s, e)]),
    };
    if worlds.is_empty() {
        return Ok(vec![format!(
            "{} {}",
            s.l("Worlds"),
            s.c1("No worlds online.")
        )]);
    }
    Ok(vec![format_all(s, &worlds)])
}

/// `+world N` — details for a single world.
pub fn one(s: &Source) -> Result<Vec<String>> {
    // Accept both "+world 2" (query) and "+world2" (parsed as rsn_n).
    let n = if !s.query.trim().is_empty() {
        s.query.trim().to_string()
    } else if s.rsn_n != "0" {
        s.rsn_n.clone()
    } else {
        return Ok(vec![format!(
            "{} {}",
            s.l("World"),
            s.c1("Usage: +world <number>")
        )]);
    };

    let worlds = match fetch_worlds() {
        Ok(worlds) => worlds,
        Err(e) => return Ok(vec![fetch_error(s, e)]),
    };
    Ok(vec![format_one(s, &worlds, &n)])
}

fn fetch_error(s: &Source, e: anyhow::Error) -> String {
    error!("worlds fetch failed: {}", e);
    format!(
        "{} {}",
        s.l("Worlds"),
        s.c1("Could not reach the server right now.")
    )
}

fn fetch_worlds() -> Result<Vec<World>> {
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::new(5, 0))
        .build()
        .context("failed to build HTTP client")?;

    let body = match client
        .get(WORLDS_URL)
        .header(USER_AGENT, "Reinze.com")
        .send()
    {
        Ok(resp) => resp.text().context("failed to read worlds response body")?,
        Err(e) => {
            error!("{}", e);
            bail!("failed to make worlds HTTP request");
        }
    };

    parse_worlds(&body)
}

fn parse_worlds(json: &str) -> Result<Vec<World>> {
    serde_json::from_str(json).context("failed to parse worlds JSON")
}

fn kind(p2p: bool) -> &'static str {
    if p2p { "P2P" } else { "F2P" }
}

fn format_all(s: &Source, worlds: &[World]) -> String {
    let total: u32 = worlds.iter().map(|w| w.count).sum();
    let list = worlds
        .iter()
        .map(|w| {
            format!(
                "{} {} {} {}",
                s.c2(format!("W{}", w.world)),
                s.c1(&w.location),
                s.c1(kind(w.p2p)),
                s.c2(w.count),
            )
        })
        .collect::<Vec<_>>()
        .join(&s.c1(" | "));
    format!(
        "{} {} {}",
        s.l("Worlds"),
        s.p(format!("{} online", total)),
        list
    )
}

fn format_one(s: &Source, worlds: &[World], n: &str) -> String {
    match worlds.iter().find(|w| w.world == n) {
        Some(w) => format!(
            "{} {} {} {} {} {} {} {} {}",
            s.l(format!("World {}", w.world)),
            s.c1("Location:"),
            s.c2(&w.location),
            s.c1("Type:"),
            s.c2(if w.p2p { "Members" } else { "Free" }),
            s.c1("Players:"),
            s.c2(w.count),
            s.c1("Play:"),
            s.c2(&w.hd),
        ),
        None => format!(
            "{} {}",
            s.l("Worlds"),
            s.c1(format!("World {} is not online or doesn't exist.", n))
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[{"world":"1","location":"US (East)","count":4,"p2p":false,"hd":"https://w1-2004.lostcity.rs/rs2.cgi?plugin=0&world=1&lowmem=0","ld":"https://w1-2004.lostcity.rs/rs2.cgi?plugin=0&world=1&lowmem=1"},{"world":"2","location":"US (East)","count":120,"p2p":true,"hd":"https://w2-2004.lostcity.rs/rs2.cgi?plugin=0&world=2&lowmem=0","ld":"x"},{"world":"3","location":"Finland","count":65,"p2p":true,"hd":"h3","ld":"x"},{"world":"5","location":"Australia","count":35,"p2p":true,"hd":"h5","ld":"x"},{"world":"7","location":"Singapore","count":46,"p2p":true,"hd":"h7","ld":"x"}]"#;

    #[test]
    fn parses_the_payload() {
        let worlds = parse_worlds(SAMPLE).unwrap();
        assert_eq!(worlds.len(), 5);
        assert_eq!(worlds[0].world, "1");
        assert_eq!(worlds[0].location, "US (East)");
        assert_eq!(worlds[0].count, 4);
        assert!(!worlds[0].p2p);
        assert_eq!(worlds[1].count, 120);
        assert!(worlds[1].p2p);
    }

    #[test]
    fn parse_rejects_garbage() {
        assert!(parse_worlds("not json").is_err());
    }

    #[test]
    fn all_lists_worlds_and_total() {
        let worlds = parse_worlds(SAMPLE).unwrap();
        let out = format_all(&src(""), &worlds);
        assert!(out.contains("W2"));
        assert!(out.contains("Singapore"));
        assert!(out.contains("P2P"));
        assert!(out.contains("F2P"));
        assert!(out.contains("270 online")); // 4+120+65+35+46
    }

    #[test]
    fn one_shows_world_details() {
        let worlds = parse_worlds(SAMPLE).unwrap();
        let out = format_one(&src(""), &worlds, "3");
        assert!(out.contains("World 3"));
        assert!(out.contains("Finland"));
        assert!(out.contains("Members"));
        assert!(out.contains("65"));
    }

    #[test]
    fn one_reports_missing_world() {
        let worlds = parse_worlds(SAMPLE).unwrap();
        let out = format_one(&src(""), &worlds, "99");
        assert!(out.contains("not online or doesn't exist"));
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
            "worlds",
            query,
        )
    }
}
