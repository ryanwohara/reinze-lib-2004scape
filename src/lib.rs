extern crate core;

mod common;
mod stats;

use ::common::author::Author;
use ::common::source::Source;
use ::common::{PluginContext, to_str_or_default};
use log::error;
use regex::Regex;
use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(context: *const PluginContext) -> *mut c_char {
    unsafe {
        let nil = CString::new("").unwrap().into_raw();

        if context.is_null() {
            return nil;
        }

        let mut command = to_str_or_default((*context).cmd);
        let query = to_str_or_default((*context).param);
        let author = to_str_or_default((*context).author);
        let color = (*context).color;

        let re = Regex::new(r"^([a-zA-Z]+)(\d+)$").unwrap();
        let cmd = command.to_string();
        let re_match = match re.captures(&cmd) {
            Some(captures) => vec![captures],
            None => vec![],
        };

        let mut rsn_n = "0";

        if re_match.len() > 0 {
            command = re_match[0].get(1).unwrap().as_str().to_string();
            rsn_n = re_match[0].get(2).unwrap().as_str();
        }

        let source = Source::create(
            rsn_n,
            Author::create(author, color),
            &command.to_string(),
            &query,
        );

        match match command.as_str() {
            "overall" | "stats" | "total" | "attack" | "att" | "defence" | "def" | "strength"
            | "str" | "hitpoints" | "hp" | "ranged" | "range" | "prayer" | "pray" | "magic"
            | "mage" | "cooking" | "cook" | "woodcutting" | "wc" | "fletching" | "fletch"
            | "fishing" | "fish" | "firemaking" | "fm" | "crafting" | "craft" | "smithing"
            | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil"
            | "thieving" | "thief" | "slayer" | "slay" | "runecraft" | "rc" => {
                stats::lookup(source)
            }
            "combat" | "cmb" => stats::combat(source),
            "help" => Ok(r"combat[N]
stats[N]"
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()),
            "" => Ok(r"co?mb(at)?\d*$
stats
overall
total
att(ack)?
def(ence)?
str(ength)?
h(it)?p(oints)?
ranged?
pray(er)?
mag(e|ic)
cook(ing)?
w(ood)?c(utting)?
fletch(ing)?
fish(ing)?
f(ire)?m(aking)?
craft(ing)?
smith(ing)?
min(e|ing)
herb(lore)?
agil(ity)?
thie(f|ving)
slay(er)?
r(une)?c(raft)?"
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()),
            _ => Ok(vec![]),
        } {
            Ok(output) => match CString::new(output.join("\n")) {
                Ok(output) => output.into_raw(),
                Err(_) => nil,
            },
            Err(e) => {
                error!("Command '{}' failed: {:?}", command, e);
                nil
            }
        }
    }
}
