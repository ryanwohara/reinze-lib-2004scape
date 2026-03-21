use anyhow::{Result, bail};
use common::source::Source;
use common::{database, not_found};
use log::error;
use mysql::{prelude::Queryable, *};
use std::vec;

pub fn process(source: Source) -> Result<Vec<String>> {
    let query = source.query.as_str();

    if query.is_empty() {
        return help();
    }

    match query.split_whitespace().collect::<Vec<&str>>()[0] {
        "set" => set(source),
        "del" | "delete" => delete(source),
        "show" => show(source),
        "list" => list(source),
        _ => help(),
    }
}

fn set(source: Source) -> Result<Vec<String>> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let host = &source.author.host.to_string();
    let rsn_n = &source.rsn_n;
    let author = source.author.full.to_string();
    let query = match source.query.split_once(" ") {
        Some((_, rest)) => rest.to_string(),
        None => return Ok(vec!["Please specify an RSN to set.".to_string()]),
    };
    let query = &query;

    if query.is_empty() {
        return Ok(vec!["Please specify an RSN to set.".to_string()]);
    }

    match conn.exec_first::<String, &str, Params>(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => {
            match conn.exec_drop(
                "UPDATE rsn SET rsn = :query WHERE host = :host AND rsn_ident = :rsn_n",
                params! { query, host, rsn_n },
            ) {
                Ok(_) => Ok(vec![format!(
                    "{} {}",
                    source.l("RSN"),
                    format!(
                        "{}{} {} {} {} {}",
                        source.c1("Set rsn #"),
                        source.c2(rsn_n),
                        source.c1("from"),
                        source.c2(&rsn),
                        source.c1("to"),
                        source.c2(query)
                    )
                )]),
                Err(e) => {
                    error!(
                        "Database query failed: Error setting rsn #{} for {} to `{}`: {}",
                        rsn_n, author, query, e
                    );
                    bail!("Database query failed: {}", e);
                }
            }
        }
        Ok(None) => {
            match conn.exec_first::<String, &str, Params>(
                "INSERT INTO rsn (host, rsn_ident, rsn, private) VALUES (:host, :rsn_n, :query, 1)",
                params! { host, rsn_n, query },
            ) {
                Ok(_) => Ok(vec![format!(
                    "{} {}",
                    source.l("RSN"),
                    format!(
                        "{}{} {} {}",
                        source.c1("Set rsn #"),
                        source.c2(rsn_n),
                        source.c1("to"),
                        source.c2(query)
                    )
                )]),
                Err(e) => {
                    error!(
                        "Database query failed: Error setting rsn{} for {} to `{}`: {}",
                        rsn_n, author, query, e
                    );
                    bail!("Database query failed: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Database query failed: {}", e);
            bail!("Database query failed: {}", e);
        }
    }
}

fn delete(source: Source) -> Result<Vec<String>> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let host = source.author.host.to_string();
    let rsn_n = &source.rsn_n;
    let author = source.author.full.to_string();

    match conn.exec_first::<String, &str, Params>(
        "DELETE FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(_) => Ok(vec![format!(
            "{} {}",
            source.l("RSN"),
            format!("{}{}", source.c1("Deleted rsn #"), source.c2(rsn_n))
        )]),
        Err(e) => {
            error!(
                "Database query failed: Error deleting rsn #{} for {}: {}",
                rsn_n, author, e
            );
            bail!("Database query failed: {}", e);
        }
    }
}

fn show(source: Source) -> Result<Vec<String>> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let host = source.author.host.to_string();
    let rsn_n = &source.rsn_n;
    let author = source.author.full.to_string();

    match conn.exec_first::<String, &str, Params>(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => Ok(vec![format!(
            "{} {}",
            source.l("RSN"),
            source.l(&format!("#{} {}", rsn_n, rsn))
        )]),
        Ok(None) => Ok(vec![format!(
            "{} {}",
            source.l("RSN"),
            &format!(
                "{}{} {}",
                source.c1("No rsn #"),
                source.c2(rsn_n),
                source.c1("set")
            )
        )]),
        Err(e) => {
            error!(
                "Database query failed: Error getting rsn #{} for {}: {}",
                rsn_n, author, e
            );
            bail!("Database query failed: {}", e);
        }
    }
}

fn list(source: Source) -> Result<Vec<String>> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let host = source.author.host.to_string();

    match conn.exec(
        "SELECT rsn_ident, rsn FROM rsn WHERE host = :host",
        params! { host, },
    ) {
        Ok(rows) => {
            let mapped = rows
                .into_iter()
                .map(from_row::<(u32, String)>)
                .map(|(id, rsn): (u32, String)| (id.to_string(), rsn));

            Ok(vec![format!(
                "{} {}",
                source.l("RSN"),
                not_found(
                    mapped
                        .into_iter()
                        .map(|(id, rsn)| source.l(&format!("#{} {}", &id, &rsn)))
                        .collect::<Vec<String>>()
                )
            )])
        }
        Err(e) => {
            error!("Database query failed: {}", e);
            bail!("Database query failed: {}", e);
        }
    }
}

fn help() -> Result<Vec<String>> {
    Ok(vec![
        "Syntax: -rsn[N] set <name> | -rsn[N] show | -rsn[N] del | -rsn list".to_string(),
        "Examples: -rsn1 set exampleName | -rsn1 show | -rsn1 del | -rsn list".to_string(),
    ])
}
