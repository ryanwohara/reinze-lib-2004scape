# reinze-lib-2004scape

A command plugin for the **[reinze](../rust-reinze)** IRC bot that brings
[2004scape](https://2004.lostcity.rs) — the 2004-era RuneScape private server —
into your IRC channel: hiscores lookups, XP/level math, treasure-trail clue
solvers, world status, and a channel trivia game.

It compiles to a native shared library (`.so` / `.dylib`) that the bot
hot-loads at runtime. There is no separate process to run — drop the built
library into the bot's `plugins/` directory and it starts answering commands.

---

## For users

Commands are triggered in IRC with the bot's prefix (shown here as `+`). Most
accept an optional RuneScape name; when omitted, the bot uses the name you've
bound to your IRC hostmask with `+rsn`.

### Hiscores & stats

| Command | What it does |
|---|---|
| `+stats [rsn]` | Full skill overview + combat level for a player |
| `+combat [rsn]`, `+cmb` | Combat level and what's needed for the next one |
| `+attack`, `+att`, `+def`, `+str`, `+hp`, `+range`, `+pray`, `+mage`, `+cook`, `+wc`, `+fletch`, `+fish`, `+fm`, `+craft`, `+smith`, `+mine`, `+herb`, `+agil`, `+thief`, `+rc` | A single skill: level, XP, rank, and XP to the next level |
| `+overall`, `+total` | Total level / total XP |

**Stat filters** (append to any stats command):

| Flag | Meaning |
|---|---|
| `> 70`, `< 50`, `= 99`, `>= 60`, `<= 40` | Only show skills matching the level/XP condition |
| `^50` / `#99` | Compute XP between a start (`^`) and end (`#`) level — accepts `1k`/`2m`/`1b` shorthand and arithmetic |
| `-s` / `-o` / `-r` / `-x` | Sort by XP-to-next-level / order by level / by rank / by raw XP |
| `@dragon` | Search within skill detail breakdowns |

You can also target a saved alternate name with a trailing number, e.g.
`+stats2` uses your second bound RSN.

### RuneScape name binding

| Command | What it does |
|---|---|
| `+rsn <name>` | Bind a RuneScape name to your IRC hostmask so `[rsn]` can be omitted |
| `+rsn2 <name>`, `+rsn3 …` | Bind additional names, addressable as `+stats2`, `+track3`, … |

### Progress tracking

| Command | What it does |
|---|---|
| `+track [rsn]` | Show XP/level gains since your last snapshot |
| `+trackN` | Same, for your Nth bound RSN |

Snapshots are captured automatically every 6 hours.

### Treasure trails & knowledge

| Command | What it does |
|---|---|
| `+anagram <clue>` | Solve a clue-scroll anagram → NPC and location |
| `+challenge <text>` | Look up the answer to a challenge-scroll question |
| `+coords <coords>`, `+clue` | Resolve a coordinate clue to its map location |
| `+speakto <npc>`, `+speak` | Where to find an NPC for a "speak to" clue |
| `+spell <name>` | Spell details |
| `+boost <skill>` | Temporary skill boosts and how much they give |
| `+noburn <food>`, `+burn` | The level at which you stop burning a food |
| `+level <xp>`, `+lvl` | Convert XP → level |
| `+exp <level>`, `+xp` | Convert level → XP |

### Server status

| Command | What it does |
|---|---|
| `+players` | Current online player count / server title |
| `+worlds` | One-line summary of every world |
| `+world N` | Details for a single world |

### Fun

| Command | What it does |
|---|---|
| `+grats [text]`, `+gz`, `+congrats` | A congratulations message |

### Trivia

A channel-scoped trivia game with **1,000+** curated 2004-era questions,
per-channel and per-player scoring, streaks, and records.

| Command | What it does |
|---|---|
| `+trivia` / `+trivia on` | Start the game in this channel |
| `+trivia off` | Stop it |
| `+question`, `+q` | Re-show the current question |
| `+guess <answer>` | Answer (case-, dot-, and whitespace-insensitive) |
| `+hint` | Reveal a bit more of the answer (up to 3 hints, ~25/50/75%) |
| `+skip`, `+next` | Skip to a new question |
| `+triviastats` | Your totals, best streak, and channel accuracy |

Correct answers are rewarded with the time taken, running streak, and
`[NEW RECORD]` / `[NEW PB]` markers. Scores persist in MySQL, so the game
survives bot restarts.

> **Era authenticity.** The question set is actively curated to match the 2004
> game — content from later updates (the Slayer skill, Construction, God Wars,
> post-2004 quests and minigames, etc.) is removed so answers stay true to the
> world players actually inhabit. See [the trivia data pipeline](#trivia-data-pipeline).

---

## For developers

### The big picture

The bot (`rust-reinze`) is the host process. Each plugin is a `dylib` exposing a
single C ABI entry point. On every matching message the host loads the library,
calls `exported`, reads back newline-separated reply lines, and prints them to
the channel.

```
IRC message ──► rust-reinze ──► libloading::get("exported")
                                      │
                                      ▼
                         exported(PluginContext) -> *mut c_char
                                      │
                        ┌─────────────┴──────────────┐
                        │ this crate (lib.rs)         │
                        │  • match command → module   │
                        │  • module builds reply      │
                        └─────────────┬──────────────┘
                                      ▼
                         "\n"-joined lines back to host
```

Everything hangs off one function in [`src/lib.rs`](src/lib.rs):

```rust
#[unsafe(no_mangle)]
pub extern "C" fn exported(context: *const PluginContext) -> *mut c_char
```

`PluginContext` (defined in the shared `reinze-lib-common` crate) carries the
command, its parameters, the author's hostmask, the channel, and a callback the
host uses to supply that user's IRC colour preferences.

The same entry point answers a few **meta** queries the host uses for
discovery:

- `command == ""` → returns the plugin's list of **trigger regexes** (`TRIGGERS`).
  The host matches incoming commands against these to decide whether to invoke
  the plugin at all.
- `command == "help"` → a human-readable command list.
- `command == "timers"` → scheduled background jobs (`tracksnapshot:6h`).

### Stateless calls, stateful game

Each command is a *fresh* library load: **no in-process state survives between
invocations.** Anything that must be remembered lives in MySQL:

- **RSN bindings** — keyed by IRC hostmask (`rsn` table).
- **Trivia** — the active question, streaks, and scores are keyed by channel and
  host, in tables the plugin auto-creates on first use (see the schema comment
  in [`src/trivia/store.rs`](src/trivia/store.rs)).

This is why the trivia game is DB-backed rather than holding a `HashMap` in
memory — there is no long-lived process to hold one.

### The trigger-uniqueness invariant

The host calls the plugin **once per matching trigger**. If two triggers both
match a single command, the user gets **duplicate output**. Concretely, the
unanchored `stats` trigger already matches `triviastats`, so `triviastats`
deliberately has *no* trigger of its own — it rides in on `stats` and is routed
by the `match` in `lib.rs`.

This invariant is enforced by tests (`every_trivia_command_matches_exactly_one_trigger`,
`stats_commands_remain_single_match`, …). **When you add or edit a trigger, keep
these green** — an overlap is a real, user-visible bug, not a style nit.

### Adding a command

1. Add a (non-overlapping) trigger regex to the `TRIGGERS` block in `lib.rs`.
2. Add a `match` arm dispatching the command name(s) to your handler.
3. Create `src/<yourcmd>.rs` with `pub fn lookup(s: &Source) -> Result<Vec<String>>`
   (the established signature) and `mod <yourcmd>;` in `lib.rs`.
4. Build replies with the `Source` colour helpers (below) and return `Vec<String>`
   — one entry per IRC line. Return `Ok(vec![])` for "nothing to say".
5. Never panic across the FFI boundary — turn errors into a clean message. The
   trivia module's `lookup`/`dispatch` split is the pattern to copy: fallible work
   in `dispatch`, error-to-message translation in `lookup`.
6. Add it to the `help` text.

### Formatting: IRC colours via `Source`

Replies are coloured with mIRC control codes. Rather than hardcode them, use the
[`Source`](../reinze-lib-common/src/source.rs) helpers, which respect each user's
configured colours:

| Helper | Renders | Typical use |
|---|---|---|
| `s.c1(x)` | primary colour | labels, separators (` \| `) |
| `s.c2(x)` | accent colour | values |
| `s.l(x)` | `[x]` bracketed label | the command's name tag, e.g. `[Trivia]` |
| `s.p(x)` | `(x)` parenthesised | secondary/aside info |

### Data sources

| Source | Used by |
|---|---|
| `https://2004.lostcity.rs/api/hiscores/player/{rsn}` (JSON) | stats, combat, track |
| `https://2004.lostcity.rs/title` | players |
| `https://2004.losthq.rs/pages/api/worlds.php` | worlds |
| MySQL (via `common::database`) | rsn bindings, trivia state/scores |

The hiscores parser aligns entries by their **stable `type` id**, not by
position in the feed (see `parse_hiscores_raw` and its tests). That means a
reordered or newly-added hiscore entry can never shift values onto the wrong
skill, and old snapshots stay comparable across API layout changes.

### Configuration

Database access reads from the environment (a local `.env` is loaded via
`dotenv`):

```
DB_HOST=  DB_PORT=  DB_USER=  DB_PASS=  DB_NAME=
```

Without a reachable database, DB-backed commands degrade gracefully (e.g. trivia
reports "temporarily unavailable") rather than crashing the host.

### Building & deploying

```sh
cargo build --release          # produces target/release/libreinze_lib_2004scape.{so,dylib}
cargo test                     # 100+ unit tests, all pure logic — no network or DB needed
cargo fmt                      # keep formatting clean (questions.rs is large & generated)
```

Deploy with the helper, which builds and drops a timestamped copy into the host's
plugin directory (the timestamp lets the host pick up the new build and retire the
old one):

```sh
bin/bd-prod.sh
```

It expects the host checkout at `../rust-reinze` and the shared crate at
`../reinze-lib-common` (a path dependency in `Cargo.toml`).

### Trivia data pipeline

There are two files, kept in lockstep:

- **`trivia.txt`** — the human-editable source of truth. `Question|Answer` per
  line, one per row. It is **latin1 / CP1252 encoded with CRLF line endings**
  (a legacy of its mIRC origin) — preserve that when editing.
- **`src/trivia/questions.rs`** — an auto-generated
  `pub const QUESTIONS: &[(&str, &str)]` compiled into the binary. **Do not edit
  by hand**; it is regenerated from `trivia.txt`.

The generator reads `trivia.txt`, applies targeted fixes and removals (used for
the era-authenticity curation), maps CP1252 punctuation to ASCII, escapes for
Rust, and rewrites both files so they never drift. Question selection at runtime
uses a small SplitMix64 PRNG seeded from the clock, re-rolling once to avoid
immediately repeating the current question.

### Repository layout

```
src/
  lib.rs            FFI entry point, trigger table, command dispatch
  common.rs         hiscores fetch/parse, XP↔level math, combat formula
  stats.rs          stats/combat command + flag parser; stats/<skill>.rs details
  trivia.rs         trivia game logic (pure bits unit-tested)
  trivia/store.rs   MySQL persistence + auto-created schema
  trivia/questions.rs  generated question table (from trivia.txt)
  anagram.rs challenge.rs coords.rs speakto.rs spell.rs boost.rs
  noburn.rs level.rs xp.rs track.rs worlds.rs players.rs rsn.rs grats.rs
bin/bd-prod.sh      build + deploy to ../rust-reinze/plugins/
trivia.txt          trivia source of truth (latin1/CRLF)
*.mrc, Ini.ini      original mIRC scripts these commands were ported from
```

### Testing conventions

Tests are colocated with the code and cover **pure logic only** — no network or
database calls — so `cargo test` is fast and hermetic. The high-value suites:

- XP↔level tables verified against known RuneScape values across the full range.
- Combat-level formula for melee / ranged / magic / maxed builds.
- Hiscores JSON parsing, including reordering, unknown types, and garbage input.
- Trivia hint masking, answer matching, and PRNG range/exclusion.
- Trigger-uniqueness (the duplicate-output guard described above).
