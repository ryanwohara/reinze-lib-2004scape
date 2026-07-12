use anyhow::Result;
use common::snapshot;
use common::source::Source;
use log::error;

use crate::common::{HiscoreName, Listings, fetch_hiscores_raw, parse_hiscores_raw, resolve_rsn};
use crate::stats::{stats_parameters, strip_stats_parameters};

pub struct Change {
    pub name: HiscoreName,
    pub old_level: u32,
    pub new_level: u32,
    pub old_xp: u32,
    pub new_xp: u32,
}

pub fn diff_listings(old: &Listings, new: &Listings) -> Vec<Change> {
    let mut changes = vec![];

    for new_listing in new.iter() {
        let name = new_listing.name;
        if name == HiscoreName::None {
            continue;
        }

        if let Some(old_listing) = old.skill(&name.to_string()) {
            if old_listing.xp != new_listing.xp || old_listing.level != new_listing.level {
                changes.push(Change {
                    name,
                    old_level: old_listing.level,
                    new_level: new_listing.level,
                    old_xp: old_listing.xp,
                    new_xp: new_listing.xp,
                });
            }
        }
    }

    changes
}

fn format_xp_delta(delta: u32) -> String {
    if delta >= 1_000_000 {
        format!("{:.1}m", delta as f64 / 1_000_000.0)
    } else if delta >= 1_000 {
        format!("{:.1}k", delta as f64 / 1_000.0)
    } else {
        format!("{}", delta)
    }
}

pub fn format_changes(changes: &[Change], source: &Source, duration_str: &str) -> Vec<String> {
    if changes.is_empty() {
        return vec![format!(
            "{} {}: {}",
            source.l("Track"),
            source.c2(&format!("({})", duration_str)),
            source.c1("No changes")
        )];
    }

    let parts: Vec<String> = changes
        .iter()
        .map(|c| {
            let xp_delta = c.new_xp.saturating_sub(c.old_xp);
            if c.old_level != c.new_level {
                format!(
                    "{} {}→{} (+{} XP)",
                    source.c1(&c.name.to_string()),
                    c.old_level,
                    c.new_level,
                    format_xp_delta(xp_delta)
                )
            } else {
                format!(
                    "{} +{} XP",
                    source.c1(&c.name.to_string()),
                    format_xp_delta(xp_delta)
                )
            }
        })
        .collect();

    vec![format!(
        "{} {}: {}",
        source.l("Track"),
        source.c2(&format!("({})", duration_str)),
        parts.join(&source.c2(" | "))
    )]
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let query = source.query.clone();
    let flags = stats_parameters(&query);
    let cleaned = strip_stats_parameters(&query);

    let rsn = resolve_rsn(cleaned.trim(), &source);

    let live_raw = fetch_hiscores_raw(&rsn)?;
    let live_listings = parse_hiscores_raw(&live_raw)?;

    // Resolve the comparison baseline BEFORE recording the current snapshot.
    // Otherwise the no-duration "latest" lookup just returns the snapshot we are
    // about to save (identical to live), so every result is "No changes".
    let baseline: Option<(String, String)> = if flags.search.is_empty() {
        match snapshot::get_latest_snapshot("2004scape", "main", &rsn) {
            Ok(opt) => opt.map(|data| (data, "latest".to_string())),
            Err(e) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!("Snapshot lookup failed: {}", e))
                )]);
            }
        }
    } else {
        let hours = match snapshot::parse_duration(&flags.search) {
            Ok(h) => h,
            Err(_) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!(
                        "Invalid duration '{}'. Use e.g. @3d, @1w, @12h, @2w3d",
                        flags.search
                    ))
                )]);
            }
        };
        match snapshot::get_snapshot("2004scape", "main", &rsn, hours) {
            Ok(opt) => opt.map(|data| (data, flags.search.clone())),
            Err(e) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!("Snapshot lookup failed: {}", e))
                )]);
            }
        }
    };

    // Record the current snapshot for future comparisons (also bootstraps the
    // first-ever lookup for a player, which has no baseline yet).
    let _ = snapshot::save_snapshot("2004scape", "main", &rsn, &live_raw);

    let (old_raw, duration_str) = match baseline {
        Some(b) => b,
        None => {
            let scope = if flags.search.is_empty() {
                String::new()
            } else {
                format!(" within {}", flags.search)
            };
            return Ok(vec![format!(
                "{} {}",
                source.l("Track"),
                source.c1(&format!(
                    "No snapshot found for {}{}",
                    rsn.replace("_", " "),
                    scope
                ))
            )]);
        }
    };

    // Surface a bad stored snapshot as a visible message: a `?` here would
    // propagate to the FFI boundary, which swallows errors into silence.
    let old_listings = match parse_hiscores_raw(&old_raw) {
        Ok(listings) => listings,
        Err(e) => {
            return Ok(vec![format!(
                "{} {}",
                source.l("Track"),
                source.c1(&format!(
                    "Can't compare against the {} snapshot: {}",
                    duration_str, e
                ))
            )]);
        }
    };
    let changes = diff_listings(&old_listings, &live_listings);
    Ok(format_changes(&changes, &source, &duration_str))
}

/// Called by the bot timer system every 6h.
pub fn snapshot_all() -> Result<Vec<String>> {
    let rsns = snapshot::get_tracked_players("2004scape")?;
    let mut count = 0;

    for rsn in &rsns {
        match fetch_hiscores_raw(rsn) {
            Ok(raw) => {
                let _ = snapshot::save_snapshot("2004scape", "main", rsn, &raw);
                count += 1;
            }
            Err(e) => {
                error!("Failed to snapshot {}: {}", rsn, e);
            }
        }
    }

    Ok(vec![format!(
        "Snapshotted {}/{} players",
        count,
        rsns.len()
    )])
}
