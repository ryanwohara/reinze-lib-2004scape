use anyhow::Result;
use common::source::Source;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    if s.query.trim().is_empty() {
        return Ok(vec![format!(
            "{} {}",
            s.l("Challenge"),
            s.c1("You must specify something to look up.")
        )]);
    }

    let output = match find_challenge(&s.query) {
        Some((question, answer)) => {
            format!("{} {} {}", s.l("Challenge"), s.c1(question), s.c2(answer))
        }
        None => format!(
            "{} {}",
            s.l("Challenge"),
            s.c1("Sorry, there were no matches. Please check your spelling and try again.")
        ),
    };

    Ok(vec![output])
}

/// Finds the first challenge whose question contains the query (case-insensitive
/// substring), mirroring mIRC's `isin` scan over the numbered `[Challenges]` keys.
fn find_challenge(query: &str) -> Option<(&'static str, &'static str)> {
    let needle = query.to_lowercase();
    Challenge::ALL.iter().find_map(|challenge| {
        let (question, answer) = challenge.details();
        question
            .to_lowercase()
            .contains(&needle)
            .then_some((question, answer))
    })
}

/// A treasure-trail challenge-scroll question, ported from the mIRC `[Challenges]`
/// table. Variants are numbered to match the table's sequential keys.
enum Challenge {
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
    Q8,
    Q9,
    Q10,
    Q11,
    Q12,
    Q13,
    Q14,
    Q15,
    Q16,
}

impl Challenge {
    const ALL: &'static [Challenge] = &[
        Challenge::Q1,
        Challenge::Q2,
        Challenge::Q3,
        Challenge::Q4,
        Challenge::Q5,
        Challenge::Q6,
        Challenge::Q7,
        Challenge::Q8,
        Challenge::Q9,
        Challenge::Q10,
        Challenge::Q11,
        Challenge::Q12,
        Challenge::Q13,
        Challenge::Q14,
        Challenge::Q15,
        Challenge::Q16,
    ];

    /// Returns the challenge question and its answer.
    fn details(&self) -> (&'static str, &'static str) {
        match self {
            Challenge::Q1 => ("What is 19 to the power of 3?", "6859"),
            Challenge::Q2 => ("What is 57x89+23?", "5096"),
            Challenge::Q3 => ("If x is 15 and y is 3, what is 3x + y?", "48"),
            Challenge::Q4 => (
                "I have 16 kebabs, I eat one myself and share the rest equally between 3 friends how many do they have each?",
                "5",
            ),
            Challenge::Q5 => ("How many cannons does Lumbridge Castle have?", "9"),
            Challenge::Q6 => (
                "How many animals in the Zoo?",
                "40 (41 if you have done the Eagles' Peak quest)",
            ),
            Challenge::Q7 => ("What is 5 times 5 add 3?", "28"),
            Challenge::Q8 => (
                "How many flowers are there in the clearing below this platform?",
                "13",
            ),
            Challenge::Q9 => ("How many banana trees are there in the plantation?", "33"),
            Challenge::Q10 => (
                "How many gnomes on the Gnome ball field have red patches on their uniforms?",
                "6",
            ),
            Challenge::Q11 => ("How many houses have a cross on the door?", "20"),
            Challenge::Q12 => (
                "How many pigeon cages are there around the back of Jerico's house?",
                "3",
            ),
            Challenge::Q13 => ("How many buildings are there in the village?", "11"),
            Challenge::Q14 => (
                "How many fishermen are there on the fishing platform?",
                "11",
            ),
            Challenge::Q15 => (
                "How many people are waiting for the next Bard to perform?",
                "4",
            ),
            Challenge::Q16 => ("How many bookcases are there in the palace library?", "24"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_known_challenge() {
        let s = src("cannons");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("How many cannons does Lumbridge Castle have?"));
        assert!(output[0].contains("9"));
    }

    #[test]
    fn matches_case_insensitively() {
        let output = lookup(&src("BANANA")).unwrap();
        assert!(output[0].contains("banana trees"));
        assert!(output[0].contains("33"));
    }

    #[test]
    fn returns_first_substring_match() {
        let output = lookup(&src("what is")).unwrap();
        assert!(output[0].contains("What is 19 to the power of 3?"));
        assert!(output[0].contains("6859"));
    }

    #[test]
    fn reports_no_match() {
        let s = src("definitely not a challenge");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(
            output[0].contains(
                "Sorry, there were no matches. Please check your spelling and try again."
            )
        );
    }

    #[test]
    fn requires_input_when_empty() {
        let s = src("");
        let output = lookup(&s).unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("You must specify something to look up."));
    }

    #[test]
    fn every_challenge_is_well_formed() {
        for challenge in Challenge::ALL {
            let (question, answer) = challenge.details();
            assert!(!question.is_empty(), "empty question");
            assert!(!answer.is_empty(), "empty answer for {question}");
        }
        assert_eq!(Challenge::ALL.len(), 16);
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
            "challenge",
            query,
        )
    }
}
